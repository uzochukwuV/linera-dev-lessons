use async_graphql::{Error, Object, Schema, EmptySubscription};
use linera_sdk::base::ChainId;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    CreateGame,
    JoinGame { opponent_chain: ChainId },
    CommitMove { move_hash: Vec<u8> },
    RevealMove { game_move: Move, secret: Vec<u8> },
    PlayAgain,
    LeaveGame,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    /// Calculate the hash of a move with a secret
    pub fn hash_with_secret(&self, secret: &[u8]) -> Vec<u8> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[*self as u8]);
        hasher.update(secret);
        hasher.finalize().as_bytes().to_vec()
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn dummy(&self) -> bool {
        true
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create a new game
    async fn create_game(&self) -> Result<Vec<u8>, Error> {
        Ok(bcs::to_bytes(&Operation::CreateGame)?)
    }

    /// Join an existing game
    async fn join_game(&self, opponent_chain: String) -> Result<Vec<u8>, Error> {
        let chain_id: ChainId = opponent_chain
            .parse()
            .map_err(|_| Error::new("Invalid chain ID"))?;

        Ok(bcs::to_bytes(&Operation::JoinGame {
            opponent_chain: chain_id,
        })?)
    }

    /// Commit a move (returns the operation bytes and the secret for later reveal)
    async fn commit_move(&self, game_move: String, secret: String) -> Result<CommitResult, Error> {
        let move_enum = match game_move.to_lowercase().as_str() {
            "rock" => Move::Rock,
            "paper" => Move::Paper,
            "scissors" => Move::Scissors,
            _ => return Err(Error::new("Invalid move. Use: Rock, Paper, or Scissors")),
        };

        let secret_bytes = secret.as_bytes().to_vec();
        let move_hash = move_enum.hash_with_secret(&secret_bytes);

        let operation = Operation::CommitMove {
            move_hash: move_hash.clone(),
        };

        Ok(CommitResult {
            operation_bytes: bcs::to_bytes(&operation)?,
            move_hash_hex: hex::encode(&move_hash),
            secret: secret,
        })
    }

    /// Reveal a move
    async fn reveal_move(&self, game_move: String, secret: String) -> Result<Vec<u8>, Error> {
        let move_enum = match game_move.to_lowercase().as_str() {
            "rock" => Move::Rock,
            "paper" => Move::Paper,
            "scissors" => Move::Scissors,
            _ => return Err(Error::new("Invalid move. Use: Rock, Paper, or Scissors")),
        };

        let secret_bytes = secret.as_bytes().to_vec();

        let operation = Operation::RevealMove {
            game_move: move_enum,
            secret: secret_bytes,
        };

        Ok(bcs::to_bytes(&operation)?)
    }

    /// Play another round
    async fn play_again(&self) -> Result<Vec<u8>, Error> {
        Ok(bcs::to_bytes(&Operation::PlayAgain)?)
    }

    /// Leave the game
    async fn leave_game(&self) -> Result<Vec<u8>, Error> {
        Ok(bcs::to_bytes(&Operation::LeaveGame)?)
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct CommitResult {
    pub operation_bytes: Vec<u8>,
    pub move_hash_hex: String,
    pub secret: String,
}

#[wasm_bindgen]
pub async fn graphql(query: String) -> String {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();

    let response = schema.execute(&query).await;

    serde_json::to_string(&response).unwrap()
}

// Helper module for hex encoding (simple implementation)
mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
}
