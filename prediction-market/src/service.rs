#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::{base::{Owner, WithServiceAbi}, Service, ServiceRuntime};

use prediction_market::{Bet, Market, MarketSummary, PredictionMarketAbi, UserBalance};
use state::PredictionMarketState;

pub struct PredictionMarketService {
    state: PredictionMarketState,
}

linera_sdk::service!(PredictionMarketService);

impl WithServiceAbi for PredictionMarketService {
    type Abi = PredictionMarketAbi;
}

impl Service for PredictionMarketService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = PredictionMarketState::load(
            runtime
                .root_view_storage_context()
                .await
                .expect("Failed to get storage context"),
        )
        .await
        .expect("Failed to load state");

        PredictionMarketService { state }
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
    state: &'a PredictionMarketState,
}

#[Object]
impl<'a> QueryRoot<'a> {
    /// Get all markets
    async fn markets(&self) -> Vec<Market> {
        self.state.get_all_markets().await
    }

    /// Get a specific market by ID
    async fn market(&self, id: u64) -> Option<Market> {
        self.state.get_market(&id).await
    }

    /// Get all bets for a market
    async fn market_bets(&self, market_id: u64) -> Vec<Bet> {
        self.state.get_market_bets(market_id).await
    }

    /// Get user's bets for a specific market
    async fn user_market_bets(&self, market_id: u64, owner: String) -> Vec<Bet> {
        let owner: Owner = owner.parse().unwrap_or_else(|_| panic!("Invalid owner"));
        self.state.get_user_market_bets(market_id, &owner).await
    }

    /// Get user balance
    async fn balance(&self, owner: String) -> u64 {
        let owner: Owner = owner.parse().unwrap_or_else(|_| panic!("Invalid owner"));
        self.state.get_balance(&owner).await
    }

    /// Get all user balances
    async fn balances(&self) -> Vec<UserBalance> {
        self.state.get_all_balances().await
    }

    /// Get total number of markets
    async fn total_markets(&self) -> u64 {
        *self.state.total_markets.get()
    }

    /// Get market summary with user bets
    async fn market_summary(&self, market_id: u64, owner: Option<String>) -> Option<MarketSummary> {
        let market = self.state.get_market(&market_id).await?;
        let total_bets = self.state.get_market_bets(market_id).await.len() as u64;

        let user_bets = if let Some(owner_str) = owner {
            if let Ok(owner) = owner_str.parse::<Owner>() {
                self.state.get_user_market_bets(market_id, &owner).await
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        Some(MarketSummary {
            market,
            total_bets,
            user_bets,
        })
    }

    /// Get active markets
    async fn active_markets(&self) -> Vec<Market> {
        let markets = self.state.get_all_markets().await;
        markets
            .into_iter()
            .filter(|m| matches!(m.status, prediction_market::MarketStatus::Active))
            .collect()
    }

    /// Get resolved markets
    async fn resolved_markets(&self) -> Vec<Market> {
        let markets = self.state.get_all_markets().await;
        markets
            .into_iter()
            .filter(|m| matches!(m.status, prediction_market::MarketStatus::Resolved { .. }))
            .collect()
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
