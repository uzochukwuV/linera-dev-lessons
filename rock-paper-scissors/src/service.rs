#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::{base::WithServiceAbi, Service, ServiceRuntime};

use rock_paper_scissors::{GameStatus, Move, PlayerInfo, RockPaperScissorsAbi};
use state::RockPaperScissorsState;

pub struct RockPaperScissorsService {
    state: RockPaperScissorsState,
}

linera_sdk::service!(RockPaperScissorsService);

impl WithServiceAbi for RockPaperScissorsService {
    type Abi = RockPaperScissorsAbi;
}

impl Service for RockPaperScissorsService {
    type Parameters = ();

    async fn new(_runtime: ServiceRuntime<Self>) -> Self {
        let state = RockPaperScissorsState::load(
            _runtime
                .root_view_storage_context()
                .await
                .expect("Failed to get storage context"),
        )
        .await
        .expect("Failed to load state");

        RockPaperScissorsService { state }
    }

    async fn handle_query(&self, query: Request) -> Response {
        let schema = Schema::build(
            QueryRoot { state: &self.state },
            MutationRoot,
            EmptySubscription,
        )
        .finish();

        schema.execute(query).await
    }
}

struct QueryRoot<'a> {
    state: &'a RockPaperScissorsState,
}

#[Object]
impl<'a> QueryRoot<'a> {
    /// Get the current game status
    async fn game_status(&self) -> GameStatus {
        self.state.game_status.get().clone()
    }

    /// Get player 1 information
    async fn player1(&self) -> Option<PlayerInfo> {
        self.state.get_player1_info().await
    }

    /// Get player 2 information
    async fn player2(&self) -> Option<PlayerInfo> {
        self.state.get_player2_info().await
    }

    /// Get player 1's revealed move (if any)
    async fn player1_move(&self) -> Option<Move> {
        self.state.player1_revealed.get().clone()
    }

    /// Get player 2's revealed move (if any)
    async fn player2_move(&self) -> Option<Move> {
        self.state.player2_revealed.get().clone()
    }

    /// Get last round's player 1 move
    async fn last_round_p1_move(&self) -> Option<Move> {
        self.state.last_round_p1_move.get().clone()
    }

    /// Get last round's player 2 move
    async fn last_round_p2_move(&self) -> Option<Move> {
        self.state.last_round_p2_move.get().clone()
    }

    /// Get last round winner
    async fn last_round_winner(&self) -> Option<String> {
        self.state
            .last_round_winner
            .get()
            .as_ref()
            .map(|owner| owner.to_string())
    }

    /// Get total rounds played
    async fn total_rounds(&self) -> u32 {
        *self.state.total_rounds.get()
    }

    /// Check if both players have committed moves
    async fn both_committed(&self) -> bool {
        self.state.both_committed().await
    }

    /// Check if both players have revealed moves
    async fn both_revealed(&self) -> bool {
        self.state.both_revealed().await
    }

    /// Check if there's an active game
    async fn has_active_game(&self) -> bool {
        self.state.has_active_game().await
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    // Mutations are handled by operations, not directly in the service
    async fn dummy(&self) -> bool {
        true
    }
}
