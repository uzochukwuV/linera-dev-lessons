use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::{
    base::{ChainId, ContractAbi, Owner, ServiceAbi},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};

pub struct RockPaperScissorsAbi;

impl ContractAbi for RockPaperScissorsAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for RockPaperScissorsAbi {
    type Query = Request;
    type QueryResponse = Response;
}

/// Operations that can be executed on the contract
#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    /// Create a new game
    CreateGame,
    /// Join an existing game as player 2
    JoinGame { opponent_chain: ChainId },
    /// Commit a move (hashed to prevent cheating)
    CommitMove { move_hash: Vec<u8> },
    /// Reveal the actual move with the secret
    RevealMove { game_move: Move, secret: Vec<u8> },
    /// Start a new round
    PlayAgain,
    /// Leave the game
    LeaveGame,
}

/// Cross-chain messages for game coordination
#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    /// Notification that a player has joined
    PlayerJoined { player: Owner, player_chain: ChainId },
    /// Notification that a move has been committed
    MoveCommitted { player: Owner },
    /// Notification that a move has been revealed
    MoveRevealed { player: Owner, game_move: Move },
    /// Notification of round result
    RoundResult { winner: Option<Owner>, reason: ResultReason },
    /// Game ended notification
    GameEnded { winner: Owner, reason: EndReason },
}

/// Game moves
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, SimpleObject)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

/// Game status
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, SimpleObject)]
pub enum GameStatus {
    /// No game in progress
    Idle,
    /// Waiting for opponent to join
    WaitingForOpponent,
    /// Both players joined, waiting for moves
    WaitingForMoves,
    /// Waiting for reveals
    WaitingForReveals,
    /// Round complete
    RoundComplete,
    /// Game complete
    GameComplete,
}

/// Reason for round result
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, SimpleObject)]
pub enum ResultReason {
    /// Normal win
    Win,
    /// Tie
    Tie,
    /// Opponent cheated (hash doesn't match)
    OpponentCheated,
    /// Opponent left
    OpponentLeft,
}

/// Reason for game ending
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, SimpleObject)]
pub enum EndReason {
    /// Normal completion
    Victory,
    /// Opponent left
    OpponentLeft,
    /// Opponent cheated
    OpponentCheated,
}

/// Player information
#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct PlayerInfo {
    pub owner: Owner,
    pub chain: ChainId,
    pub rounds_won: u32,
}

impl Move {
    /// Determine if this move beats another move
    pub fn beats(&self, other: &Move) -> bool {
        matches!(
            (self, other),
            (Move::Rock, Move::Scissors) | (Move::Paper, Move::Rock) | (Move::Scissors, Move::Paper)
        )
    }

    /// Calculate the hash of a move with a secret
    pub fn hash_with_secret(&self, secret: &[u8]) -> Vec<u8> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[*self as u8]);
        hasher.update(secret);
        hasher.finalize().as_bytes().to_vec()
    }

    /// Verify that a move matches a hash
    pub fn verify_hash(&self, secret: &[u8], expected_hash: &[u8]) -> bool {
        let actual_hash = self.hash_with_secret(secret);
        actual_hash == expected_hash
    }
}

/// Error types
#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("Game already in progress")]
    GameInProgress,

    #[error("No game in progress")]
    NoGame,

    #[error("Not your turn")]
    NotYourTurn,

    #[error("Invalid game state for this operation")]
    InvalidState,

    #[error("You are not a player in this game")]
    NotAPlayer,

    #[error("Invalid move hash")]
    InvalidHash,

    #[error("Move hash does not match revealed move")]
    HashMismatch,

    #[error("Already committed move")]
    AlreadyCommitted,

    #[error("Already revealed move")]
    AlreadyRevealed,

    #[error("Cannot play against yourself")]
    CannotPlaySelf,
}
