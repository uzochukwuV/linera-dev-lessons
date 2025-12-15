use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::{
    base::{ContractAbi, Owner, ServiceAbi, Timestamp},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};

pub struct PredictionMarketAbi;

impl ContractAbi for PredictionMarketAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for PredictionMarketAbi {
    type Query = Request;
    type QueryResponse = Response;
}

/// Operations that can be executed on the contract
#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    /// Create a new prediction market
    CreateMarket {
        question: String,
        outcomes: Vec<String>,
        resolution_time: Timestamp,
        oracle_url: Option<String>,
    },
    /// Place a bet on an outcome
    PlaceBet {
        market_id: u64,
        outcome_index: usize,
        amount: u64,
    },
    /// Resolve a market (callable after resolution_time)
    ResolveMarket {
        market_id: u64,
        winning_outcome_index: usize,
    },
    /// Claim winnings from a resolved market
    ClaimWinnings { market_id: u64 },
}

/// Cross-chain messages for market updates
#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    /// Notify about new market creation
    MarketCreated { market_id: u64, question: String },
    /// Notify about new bet
    BetPlaced {
        market_id: u64,
        bettor: Owner,
        outcome_index: usize,
        amount: u64,
    },
    /// Notify about market resolution
    MarketResolved {
        market_id: u64,
        winning_outcome_index: usize,
    },
}

/// Market status
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, SimpleObject)]
pub enum MarketStatus {
    Active,
    Resolved { winning_outcome: usize },
    Cancelled,
}

/// A prediction market
#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct Market {
    pub id: u64,
    pub creator: Owner,
    pub question: String,
    pub outcomes: Vec<String>,
    pub resolution_time: Timestamp,
    pub oracle_url: Option<String>,
    pub status: MarketStatus,
    pub total_pool: u64,
    pub outcome_pools: Vec<u64>, // Amount bet on each outcome
}

/// A user's bet in a market
#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct Bet {
    pub market_id: u64,
    pub bettor: Owner,
    pub outcome_index: usize,
    pub amount: u64,
    pub claimed: bool,
}

/// User balance
#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct UserBalance {
    pub owner: Owner,
    pub balance: u64,
}

/// Market summary for display
#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct MarketSummary {
    pub market: Market,
    pub total_bets: u64,
    pub user_bets: Vec<Bet>,
}

/// Error types
#[derive(Debug, thiserror::Error)]
pub enum MarketError {
    #[error("Market not found")]
    MarketNotFound,

    #[error("Market already resolved")]
    MarketAlreadyResolved,

    #[error("Market not yet resolved")]
    MarketNotResolved,

    #[error("Market resolution time not reached")]
    ResolutionTimeNotReached,

    #[error("Invalid outcome index")]
    InvalidOutcome,

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("No winnings to claim")]
    NoWinnings,

    #[error("Winnings already claimed")]
    AlreadyClaimed,

    #[error("Only market creator can resolve")]
    OnlyCreatorCanResolve,

    #[error("Bet amount must be greater than zero")]
    InvalidBetAmount,

    #[error("Market must have at least 2 outcomes")]
    InsufficientOutcomes,
}
