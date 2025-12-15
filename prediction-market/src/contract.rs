#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{
    base::{Owner, Timestamp, WithContractAbi},
    Contract, ContractRuntime,
};

use prediction_market::{
    Market, MarketError, MarketStatus, Message, Operation, PredictionMarketAbi, Bet,
};
use state::PredictionMarketState;

pub struct PredictionMarketContract {
    state: PredictionMarketState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(PredictionMarketContract);

impl WithContractAbi for PredictionMarketContract {
    type Abi = PredictionMarketAbi;
}

impl Contract for PredictionMarketContract {
    type Message = Message;
    type Parameters = ();
    type InstantiationArgument = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = PredictionMarketState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        PredictionMarketContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        self.state.next_market_id.set(0);
        self.state.total_markets.set(0);
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        let owner = self
            .runtime
            .authenticated_signer()
            .expect("Operation must be authenticated");

        match operation {
            Operation::CreateMarket {
                question,
                outcomes,
                resolution_time,
                oracle_url,
            } => {
                self.create_market(owner, question, outcomes, resolution_time, oracle_url)
                    .await
                    .expect("Failed to create market");
            }

            Operation::PlaceBet {
                market_id,
                outcome_index,
                amount,
            } => {
                self.place_bet(owner, market_id, outcome_index, amount)
                    .await
                    .expect("Failed to place bet");
            }

            Operation::ResolveMarket {
                market_id,
                winning_outcome_index,
            } => {
                self.resolve_market(owner, market_id, winning_outcome_index)
                    .await
                    .expect("Failed to resolve market");
            }

            Operation::ClaimWinnings { market_id } => {
                self.claim_winnings(owner, market_id)
                    .await
                    .expect("Failed to claim winnings");
            }
        }
    }

    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            Message::MarketCreated { .. } => {
                // Just for notifications, state already updated
            }
            Message::BetPlaced { .. } => {
                // Just for notifications, state already updated
            }
            Message::MarketResolved { .. } => {
                // Just for notifications, state already updated
            }
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl PredictionMarketContract {
    /// Create a new prediction market
    async fn create_market(
        &mut self,
        creator: Owner,
        question: String,
        outcomes: Vec<String>,
        resolution_time: Timestamp,
        oracle_url: Option<String>,
    ) -> Result<(), MarketError> {
        // Validate
        if outcomes.len() < 2 {
            return Err(MarketError::InsufficientOutcomes);
        }

        let market_id = self.state.next_market_id().await;

        let outcome_pools = vec![0; outcomes.len()];

        let market = Market {
            id: market_id,
            creator,
            question: question.clone(),
            outcomes,
            resolution_time,
            oracle_url,
            status: MarketStatus::Active,
            total_pool: 0,
            outcome_pools,
        };

        self.state.add_market(market).await;

        // Broadcast market creation (could be cross-chain)
        self.runtime
            .prepare_message(Message::MarketCreated {
                market_id,
                question,
            })
            .send_to(self.runtime.chain_id());

        Ok(())
    }

    /// Place a bet on a market outcome
    async fn place_bet(
        &mut self,
        bettor: Owner,
        market_id: u64,
        outcome_index: usize,
        amount: u64,
    ) -> Result<(), MarketError> {
        // Validate amount
        if amount == 0 {
            return Err(MarketError::InvalidBetAmount);
        }

        // Get market
        let mut market = self
            .state
            .get_market(&market_id)
            .await
            .ok_or(MarketError::MarketNotFound)?;

        // Check market is active
        if market.status != MarketStatus::Active {
            return Err(MarketError::MarketAlreadyResolved);
        }

        // Check outcome index is valid
        if outcome_index >= market.outcomes.len() {
            return Err(MarketError::InvalidOutcome);
        }

        // Check resolution time not passed
        if self.runtime.system_time() >= market.resolution_time {
            return Err(MarketError::ResolutionTimeNotReached);
        }

        // Check user balance (for now, give users initial balance of 1000 if they have 0)
        let balance = self.state.get_balance(&bettor).await;
        let balance = if balance == 0 {
            self.state.set_balance(&bettor, 1000).await;
            1000
        } else {
            balance
        };

        if balance < amount {
            return Err(MarketError::InsufficientBalance);
        }

        // Deduct from balance
        self.state
            .subtract_balance(&bettor, amount)
            .await
            .map_err(|_| MarketError::InsufficientBalance)?;

        // Create bet
        let bet = Bet {
            market_id,
            bettor,
            outcome_index,
            amount,
            claimed: false,
        };

        self.state.add_bet(bet).await;

        // Update market pools
        market.total_pool += amount;
        market.outcome_pools[outcome_index] += amount;
        self.state.update_market(market).await;

        // Broadcast bet
        self.runtime
            .prepare_message(Message::BetPlaced {
                market_id,
                bettor,
                outcome_index,
                amount,
            })
            .send_to(self.runtime.chain_id());

        Ok(())
    }

    /// Resolve a market (only creator can resolve)
    async fn resolve_market(
        &mut self,
        caller: Owner,
        market_id: u64,
        winning_outcome_index: usize,
    ) -> Result<(), MarketError> {
        // Get market
        let mut market = self
            .state
            .get_market(&market_id)
            .await
            .ok_or(MarketError::MarketNotFound)?;

        // Check market is active
        if market.status != MarketStatus::Active {
            return Err(MarketError::MarketAlreadyResolved);
        }

        // Check caller is creator
        if caller != market.creator {
            return Err(MarketError::OnlyCreatorCanResolve);
        }

        // Check resolution time has passed
        if self.runtime.system_time() < market.resolution_time {
            return Err(MarketError::ResolutionTimeNotReached);
        }

        // Check outcome index is valid
        if winning_outcome_index >= market.outcomes.len() {
            return Err(MarketError::InvalidOutcome);
        }

        // TODO: If oracle_url is set, fetch result from oracle
        // For now, we trust the creator to provide the correct outcome

        // Update market status
        market.status = MarketStatus::Resolved {
            winning_outcome: winning_outcome_index,
        };
        self.state.update_market(market).await;

        // Broadcast resolution
        self.runtime
            .prepare_message(Message::MarketResolved {
                market_id,
                winning_outcome_index,
            })
            .send_to(self.runtime.chain_id());

        Ok(())
    }

    /// Claim winnings from a resolved market
    async fn claim_winnings(&mut self, claimer: Owner, market_id: u64) -> Result<(), MarketError> {
        // Get market
        let market = self
            .state
            .get_market(&market_id)
            .await
            .ok_or(MarketError::MarketNotFound)?;

        // Check market is resolved
        let winning_outcome_index = match market.status {
            MarketStatus::Resolved { winning_outcome } => winning_outcome,
            _ => return Err(MarketError::MarketNotResolved),
        };

        // Get user's bets on the winning outcome
        let bets = self.state.get_user_market_bets(market_id, &claimer).await;

        let mut total_winnings = 0u64;

        for bet in bets {
            // Skip if already claimed
            if bet.claimed {
                continue;
            }

            // Skip if not winning outcome
            if bet.outcome_index != winning_outcome_index {
                continue;
            }

            // Calculate winnings
            // Formula: (bet_amount / winning_pool) * total_pool
            let winning_pool = market.outcome_pools[winning_outcome_index];
            if winning_pool == 0 {
                continue;
            }

            let winnings = (bet.amount as u128 * market.total_pool as u128) / winning_pool as u128;
            total_winnings += winnings as u64;

            // Mark bet as claimed
            let mut updated_bet = bet.clone();
            updated_bet.claimed = true;
            self.state.update_bet(updated_bet).await;
        }

        if total_winnings == 0 {
            return Err(MarketError::NoWinnings);
        }

        // Add winnings to user balance
        self.state.add_balance(&claimer, total_winnings).await;

        Ok(())
    }
}
