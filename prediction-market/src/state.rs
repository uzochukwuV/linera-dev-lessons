use async_graphql::SimpleObject;
use linera_sdk::{
    base::Owner,
    views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext},
};
use serde::{Deserialize, Serialize};

use crate::{Bet, Market, MarketStatus, UserBalance};

/// The application state for Prediction Market
#[derive(RootView, SimpleObject)]
#[view(context = ViewStorageContext)]
pub struct PredictionMarketState {
    /// Counter for generating market IDs
    pub next_market_id: RegisterView<u64>,

    /// All markets (market_id -> Market)
    pub markets: MapView<u64, Market>,

    /// All bets (key: "market_id:bettor:bet_index" -> Bet)
    pub bets: MapView<String, Bet>,

    /// User balances (Owner -> balance)
    pub balances: MapView<Owner, u64>,

    /// Total number of markets
    pub total_markets: RegisterView<u64>,
}

impl PredictionMarketState {
    /// Get the next market ID and increment the counter
    pub async fn next_market_id(&mut self) -> u64 {
        let id = *self.next_market_id.get();
        self.next_market_id.set(id + 1);
        let total = *self.total_markets.get();
        self.total_markets.set(total + 1);
        id
    }

    /// Add a new market
    pub async fn add_market(&mut self, market: Market) {
        self.markets.insert(&market.id, market).expect("Failed to insert market");
    }

    /// Get a market by ID
    pub async fn get_market(&self, market_id: &u64) -> Option<Market> {
        self.markets.get(market_id).await.ok().flatten()
    }

    /// Update a market
    pub async fn update_market(&mut self, market: Market) {
        self.markets.insert(&market.id, market).expect("Failed to update market");
    }

    /// Add a bet
    pub async fn add_bet(&mut self, bet: Bet) {
        let key = format!("{}:{}:{}", bet.market_id, bet.bettor, bet.outcome_index);
        self.bets.insert(&key, bet).expect("Failed to insert bet");
    }

    /// Get all bets for a market
    pub async fn get_market_bets(&self, market_id: u64) -> Vec<Bet> {
        let mut bets = Vec::new();
        let prefix = format!("{}:", market_id);

        self.bets.for_each_key_value(|key, bet| {
            if key.starts_with(&prefix) {
                bets.push(bet);
            }
            Ok(())
        }).await.expect("Failed to iterate bets");

        bets
    }

    /// Get user's bets for a specific market
    pub async fn get_user_market_bets(&self, market_id: u64, owner: &Owner) -> Vec<Bet> {
        let mut bets = Vec::new();
        let prefix = format!("{}:{}:", market_id, owner);

        self.bets.for_each_key_value(|key, bet| {
            if key.starts_with(&prefix) {
                bets.push(bet);
            }
            Ok(())
        }).await.expect("Failed to iterate bets");

        bets
    }

    /// Update a bet
    pub async fn update_bet(&mut self, bet: Bet) {
        let key = format!("{}:{}:{}", bet.market_id, bet.bettor, bet.outcome_index);
        self.bets.insert(&key, bet).expect("Failed to update bet");
    }

    /// Get user balance
    pub async fn get_balance(&self, owner: &Owner) -> u64 {
        self.balances.get(owner).await.ok().flatten().unwrap_or(0)
    }

    /// Set user balance
    pub async fn set_balance(&mut self, owner: &Owner, balance: u64) {
        self.balances.insert(owner, balance).expect("Failed to set balance");
    }

    /// Add to user balance
    pub async fn add_balance(&mut self, owner: &Owner, amount: u64) {
        let current = self.get_balance(owner).await;
        self.set_balance(owner, current + amount).await;
    }

    /// Subtract from user balance
    pub async fn subtract_balance(&mut self, owner: &Owner, amount: u64) -> Result<(), &'static str> {
        let current = self.get_balance(owner).await;
        if current < amount {
            return Err("Insufficient balance");
        }
        self.set_balance(owner, current - amount).await;
        Ok(())
    }

    /// Get all markets
    pub async fn get_all_markets(&self) -> Vec<Market> {
        let mut markets = Vec::new();

        self.markets.for_each_key_value(|_key, market| {
            markets.push(market);
            Ok(())
        }).await.expect("Failed to iterate markets");

        markets
    }

    /// Get all balances
    pub async fn get_all_balances(&self) -> Vec<UserBalance> {
        let mut balances = Vec::new();

        self.balances.for_each_key_value(|owner, balance| {
            balances.push(UserBalance { owner, balance });
            Ok(())
        }).await.expect("Failed to iterate balances");

        balances
    }
}
