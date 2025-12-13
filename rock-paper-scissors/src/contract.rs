#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{
    base::{ChainId, Owner, WithContractAbi},
    Contract, ContractRuntime,
};

use rock_paper_scissors::{
    EndReason, GameError, GameStatus, Message, Move, Operation, ResultReason,
    RockPaperScissorsAbi,
};
use state::RockPaperScissorsState;

pub struct RockPaperScissorsContract {
    state: RockPaperScissorsState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(RockPaperScissorsContract);

impl WithContractAbi for RockPaperScissorsContract {
    type Abi = RockPaperScissorsAbi;
}

impl Contract for RockPaperScissorsContract {
    type Message = Message;
    type Parameters = ();
    type InstantiationArgument = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = RockPaperScissorsState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        RockPaperScissorsContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        self.state.game_status.set(GameStatus::Idle);
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        let owner = self
            .runtime
            .authenticated_signer()
            .expect("Operation must be authenticated");

        match operation {
            Operation::CreateGame => {
                self.create_game(owner).await.expect("Failed to create game");
            }

            Operation::JoinGame { opponent_chain } => {
                self.join_game(owner, opponent_chain)
                    .await
                    .expect("Failed to join game");
            }

            Operation::CommitMove { move_hash } => {
                self.commit_move(owner, move_hash)
                    .await
                    .expect("Failed to commit move");
            }

            Operation::RevealMove { game_move, secret } => {
                self.reveal_move(owner, game_move, secret)
                    .await
                    .expect("Failed to reveal move");
            }

            Operation::PlayAgain => {
                self.play_again(owner).await.expect("Failed to start new round");
            }

            Operation::LeaveGame => {
                self.leave_game(owner).await.expect("Failed to leave game");
            }
        }
    }

    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            Message::PlayerJoined {
                player,
                player_chain,
            } => {
                // Set player 2 information
                self.state.player2_owner.set(Some(player));
                self.state.player2_chain.set(Some(player_chain));
                self.state.game_status.set(GameStatus::WaitingForMoves);
            }

            Message::MoveCommitted { .. } => {
                // Check if both players have committed
                if self.state.both_committed().await {
                    self.state.game_status.set(GameStatus::WaitingForReveals);
                }
            }

            Message::MoveRevealed { player, game_move } => {
                // Update opponent's revealed move
                if self.state.is_player1(&player).await {
                    self.state.player1_revealed.set(Some(game_move));
                } else if self.state.is_player2(&player).await {
                    self.state.player2_revealed.set(Some(game_move));
                }

                // Check if both have revealed
                if self.state.both_revealed().await {
                    self.finalize_round().await;
                }
            }

            Message::RoundResult { winner, .. } => {
                if let Some(ref winner_owner) = winner {
                    self.state.increment_score(winner_owner).await;
                }
                self.state.game_status.set(GameStatus::RoundComplete);
            }

            Message::GameEnded { .. } => {
                self.state.game_status.set(GameStatus::GameComplete);
            }
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl RockPaperScissorsContract {
    /// Create a new game
    async fn create_game(&mut self, owner: Owner) -> Result<(), GameError> {
        if self.state.has_active_game().await {
            return Err(GameError::GameInProgress);
        }

        // Initialize game with player 1
        self.state.reset_game().await;
        self.state.player1_owner.set(Some(owner));
        self.state.player1_chain.set(Some(self.runtime.chain_id()));
        self.state.game_status.set(GameStatus::WaitingForOpponent);

        Ok(())
    }

    /// Join an existing game as player 2
    async fn join_game(&mut self, owner: Owner, opponent_chain: ChainId) -> Result<(), GameError> {
        // Cannot join your own game
        if opponent_chain == self.runtime.chain_id() {
            return Err(GameError::CannotPlaySelf);
        }

        // Set local player 2 info
        self.state.player2_owner.set(Some(owner));
        self.state.player2_chain.set(Some(self.runtime.chain_id()));
        self.state.player1_chain.set(Some(opponent_chain));
        self.state.game_status.set(GameStatus::WaitingForMoves);

        // Notify opponent that we joined
        self.runtime
            .prepare_message(Message::PlayerJoined {
                player: owner,
                player_chain: self.runtime.chain_id(),
            })
            .send_to(opponent_chain);

        Ok(())
    }

    /// Commit a move (hashed)
    async fn commit_move(&mut self, owner: Owner, move_hash: Vec<u8>) -> Result<(), GameError> {
        // Verify game state
        if *self.state.game_status.get() != GameStatus::WaitingForMoves {
            return Err(GameError::InvalidState);
        }

        // Verify player
        if !self.state.is_player(&owner).await {
            return Err(GameError::NotAPlayer);
        }

        // Store committed move
        if self.state.is_player1(&owner).await {
            if self.state.player1_committed.get().is_some() {
                return Err(GameError::AlreadyCommitted);
            }
            self.state.player1_committed.set(Some(move_hash));

            // Notify opponent
            if let Some(p2_chain) = *self.state.player2_chain.get() {
                self.runtime
                    .prepare_message(Message::MoveCommitted { player: owner })
                    .send_to(p2_chain);
            }
        } else {
            if self.state.player2_committed.get().is_some() {
                return Err(GameError::AlreadyCommitted);
            }
            self.state.player2_committed.set(Some(move_hash));

            // Notify opponent
            if let Some(p1_chain) = *self.state.player1_chain.get() {
                self.runtime
                    .prepare_message(Message::MoveCommitted { player: owner })
                    .send_to(p1_chain);
            }
        }

        // Check if both committed
        if self.state.both_committed().await {
            self.state.game_status.set(GameStatus::WaitingForReveals);
        }

        Ok(())
    }

    /// Reveal a move
    async fn reveal_move(
        &mut self,
        owner: Owner,
        game_move: Move,
        secret: Vec<u8>,
    ) -> Result<(), GameError> {
        // Verify game state
        if *self.state.game_status.get() != GameStatus::WaitingForReveals {
            return Err(GameError::InvalidState);
        }

        // Verify player
        if !self.state.is_player(&owner).await {
            return Err(GameError::NotAPlayer);
        }

        // Verify hash and store revealed move
        if self.state.is_player1(&owner).await {
            if self.state.player1_revealed.get().is_some() {
                return Err(GameError::AlreadyRevealed);
            }

            // Verify hash
            let committed_hash = self
                .state
                .player1_committed
                .get()
                .as_ref()
                .ok_or(GameError::InvalidState)?;

            if !game_move.verify_hash(&secret, committed_hash) {
                return Err(GameError::HashMismatch);
            }

            self.state.player1_revealed.set(Some(game_move));

            // Notify opponent
            if let Some(p2_chain) = *self.state.player2_chain.get() {
                self.runtime
                    .prepare_message(Message::MoveRevealed {
                        player: owner,
                        game_move,
                    })
                    .send_to(p2_chain);
            }
        } else {
            if self.state.player2_revealed.get().is_some() {
                return Err(GameError::AlreadyRevealed);
            }

            // Verify hash
            let committed_hash = self
                .state
                .player2_committed
                .get()
                .as_ref()
                .ok_or(GameError::InvalidState)?;

            if !game_move.verify_hash(&secret, committed_hash) {
                return Err(GameError::HashMismatch);
            }

            self.state.player2_revealed.set(Some(game_move));

            // Notify opponent
            if let Some(p1_chain) = *self.state.player1_chain.get() {
                self.runtime
                    .prepare_message(Message::MoveRevealed {
                        player: owner,
                        game_move,
                    })
                    .send_to(p1_chain);
            }
        }

        // Check if both revealed
        if self.state.both_revealed().await {
            self.finalize_round().await;
        }

        Ok(())
    }

    /// Finalize the round and determine winner
    async fn finalize_round(&mut self) {
        let winner = self.state.determine_winner().await;

        // Save round moves
        self.state
            .last_round_p1_move
            .set(*self.state.player1_revealed.get());
        self.state
            .last_round_p2_move
            .set(*self.state.player2_revealed.get());
        self.state.last_round_winner.set(winner.clone());

        // Increment total rounds
        let rounds = *self.state.total_rounds.get();
        self.state.total_rounds.set(rounds + 1);

        // Increment winner's score
        if let Some(ref winner_owner) = winner {
            self.state.increment_score(winner_owner).await;
        }

        // Notify opponent of result
        let message = Message::RoundResult {
            winner,
            reason: ResultReason::Win,
        };

        if let Some(p1_chain) = *self.state.player1_chain.get() {
            if p1_chain != self.runtime.chain_id() {
                self.runtime.prepare_message(message.clone()).send_to(p1_chain);
            }
        }

        if let Some(p2_chain) = *self.state.player2_chain.get() {
            if p2_chain != self.runtime.chain_id() {
                self.runtime.prepare_message(message).send_to(p2_chain);
            }
        }

        self.state.game_status.set(GameStatus::RoundComplete);
    }

    /// Start a new round
    async fn play_again(&mut self, owner: Owner) -> Result<(), GameError> {
        // Verify game state
        if *self.state.game_status.get() != GameStatus::RoundComplete {
            return Err(GameError::InvalidState);
        }

        // Verify player
        if !self.state.is_player(&owner).await {
            return Err(GameError::NotAPlayer);
        }

        // Reset round
        self.state.reset_round().await;

        Ok(())
    }

    /// Leave the game
    async fn leave_game(&mut self, owner: Owner) -> Result<(), GameError> {
        // Verify player
        if !self.state.is_player(&owner).await {
            return Err(GameError::NotAPlayer);
        }

        // Determine opponent
        let opponent_chain = if self.state.is_player1(&owner).await {
            *self.state.player2_chain.get()
        } else {
            *self.state.player1_chain.get()
        };

        // Notify opponent
        if let Some(opp_chain) = opponent_chain {
            let winner = if self.state.is_player1(&owner).await {
                self.state.player2_owner.get().clone()
            } else {
                self.state.player1_owner.get().clone()
            };

            if let Some(winner_owner) = winner {
                self.runtime
                    .prepare_message(Message::GameEnded {
                        winner: winner_owner,
                        reason: EndReason::OpponentLeft,
                    })
                    .send_to(opp_chain);
            }
        }

        // Reset game
        self.state.reset_game().await;

        Ok(())
    }
}
