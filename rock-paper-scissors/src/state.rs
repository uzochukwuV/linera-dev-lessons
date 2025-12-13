use async_graphql::SimpleObject;
use linera_sdk::{
    base::{ChainId, Owner},
    views::{linera_views, RegisterView, RootView, ViewStorageContext},
};
use serde::{Deserialize, Serialize};

use crate::{GameStatus, Move, PlayerInfo};

/// The application state for Rock Paper Scissors game
#[derive(RootView, SimpleObject)]
#[view(context = ViewStorageContext)]
pub struct RockPaperScissorsState {
    /// Current game status
    pub game_status: RegisterView<GameStatus>,

    /// Player 1 (game creator)
    pub player1_owner: RegisterView<Option<Owner>>,
    pub player1_chain: RegisterView<Option<ChainId>>,
    pub player1_rounds_won: RegisterView<u32>,

    /// Player 2 (opponent)
    pub player2_owner: RegisterView<Option<Owner>>,
    pub player2_chain: RegisterView<Option<ChainId>>,
    pub player2_rounds_won: RegisterView<u32>,

    /// Current round - committed moves (hashed)
    pub player1_committed: RegisterView<Option<Vec<u8>>>,
    pub player2_committed: RegisterView<Option<Vec<u8>>>,

    /// Current round - revealed moves
    pub player1_revealed: RegisterView<Option<Move>>,
    pub player2_revealed: RegisterView<Option<Move>>,

    /// Last round result
    pub last_round_winner: RegisterView<Option<Owner>>,
    pub last_round_p1_move: RegisterView<Option<Move>>,
    pub last_round_p2_move: RegisterView<Option<Move>>,

    /// Total rounds played
    pub total_rounds: RegisterView<u32>,
}

impl RockPaperScissorsState {
    /// Check if there is an active game
    pub async fn has_active_game(&self) -> bool {
        let status = self.game_status.get();
        !matches!(*status, GameStatus::Idle | GameStatus::GameComplete)
    }

    /// Check if a player is in the current game
    pub async fn is_player(&self, owner: &Owner) -> bool {
        let p1 = self.player1_owner.get();
        let p2 = self.player2_owner.get();

        if let Some(ref p1_owner) = *p1 {
            if p1_owner == owner {
                return true;
            }
        }

        if let Some(ref p2_owner) = *p2 {
            if p2_owner == owner {
                return true;
            }
        }

        false
    }

    /// Get player 1 info
    pub async fn get_player1_info(&self) -> Option<PlayerInfo> {
        let owner = self.player1_owner.get().clone()?;
        let chain = self.player1_chain.get().clone()?;
        let rounds_won = *self.player1_rounds_won.get();

        Some(PlayerInfo {
            owner,
            chain,
            rounds_won,
        })
    }

    /// Get player 2 info
    pub async fn get_player2_info(&self) -> Option<PlayerInfo> {
        let owner = self.player2_owner.get().clone()?;
        let chain = self.player2_chain.get().clone()?;
        let rounds_won = *self.player2_rounds_won.get();

        Some(PlayerInfo {
            owner,
            chain,
            rounds_won,
        })
    }

    /// Check if player is player 1
    pub async fn is_player1(&self, owner: &Owner) -> bool {
        if let Some(ref p1) = *self.player1_owner.get() {
            p1 == owner
        } else {
            false
        }
    }

    /// Check if player is player 2
    pub async fn is_player2(&self, owner: &Owner) -> bool {
        if let Some(ref p2) = *self.player2_owner.get() {
            p2 == owner
        } else {
            false
        }
    }

    /// Reset round state for a new round
    pub async fn reset_round(&mut self) {
        self.player1_committed.set(None);
        self.player2_committed.set(None);
        self.player1_revealed.set(None);
        self.player2_revealed.set(None);
        self.game_status.set(GameStatus::WaitingForMoves);
    }

    /// Reset entire game state
    pub async fn reset_game(&mut self) {
        self.game_status.set(GameStatus::Idle);
        self.player1_owner.set(None);
        self.player1_chain.set(None);
        self.player1_rounds_won.set(0);
        self.player2_owner.set(None);
        self.player2_chain.set(None);
        self.player2_rounds_won.set(0);
        self.player1_committed.set(None);
        self.player2_committed.set(None);
        self.player1_revealed.set(None);
        self.player2_revealed.set(None);
        self.last_round_winner.set(None);
        self.last_round_p1_move.set(None);
        self.last_round_p2_move.set(None);
        self.total_rounds.set(0);
    }

    /// Check if both players have committed
    pub async fn both_committed(&self) -> bool {
        self.player1_committed.get().is_some() && self.player2_committed.get().is_some()
    }

    /// Check if both players have revealed
    pub async fn both_revealed(&self) -> bool {
        self.player1_revealed.get().is_some() && self.player2_revealed.get().is_some()
    }

    /// Determine winner of the round
    pub async fn determine_winner(&self) -> Option<Owner> {
        let p1_move = self.player1_revealed.get().as_ref()?;
        let p2_move = self.player2_revealed.get().as_ref()?;

        if p1_move == p2_move {
            // Tie
            None
        } else if p1_move.beats(p2_move) {
            // Player 1 wins
            self.player1_owner.get().clone()
        } else {
            // Player 2 wins
            self.player2_owner.get().clone()
        }
    }

    /// Increment winner's score
    pub async fn increment_score(&mut self, winner: &Owner) {
        if self.is_player1(winner).await {
            let current = *self.player1_rounds_won.get();
            self.player1_rounds_won.set(current + 1);
        } else if self.is_player2(winner).await {
            let current = *self.player2_rounds_won.get();
            self.player2_rounds_won.set(current + 1);
        }
    }
}
