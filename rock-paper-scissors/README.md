# 🪨📄✂️ Rock Paper Scissors - Linera Blockchain Game

A fully decentralized, two-player PvP Rock Paper Scissors game built on the Linera blockchain. Features cross-chain messaging, commit-reveal pattern for fairness, and a modern Vue.js UI with CheCko wallet integration.

## 🎮 Features

- **Two-Player PvP**: Play against anyone on the Linera network
- **Cross-Chain**: Each player can be on different microchains
- **Provably Fair**: Uses commit-reveal pattern to prevent cheating
- **Real-time Updates**: Game state synchronizes across players
- **Modern UI**: Built with Vue 3 + Quasar Framework
- **Wallet Integration**: CheCko/MetaMask support via Linera provider
- **Score Tracking**: Persistent round scores

## 🏗️ Architecture

### Smart Contract Components

```
rock-paper-scissors/
├── src/
│   ├── lib.rs        # ABI definitions, operations, messages
│   ├── state.rs      # Game state management
│   ├── contract.rs   # Game logic and message handling
│   └── service.rs    # GraphQL query service
└── webui/
    ├── wasm/         # WASM operation serialization
    └── src/          # Vue.js frontend
```

### Game Flow

1. **Game Creation**: Player 1 creates a new game
2. **Join**: Player 2 joins using Player 1's chain ID
3. **Commit Phase**: Both players commit hashed moves
4. **Reveal Phase**: Both players reveal actual moves
5. **Result**: Winner determined, scores updated
6. **Repeat**: Play another round or leave

### Commit-Reveal Pattern

To prevent cheating (one player seeing the other's move first):

1. **Commit**: Player sends `hash(move + secret)` to blockchain
2. **Reveal**: After both commit, players reveal `move` + `secret`
3. **Verify**: Contract verifies hash matches revealed move
4. **Determine**: Calculate winner using standard RPS rules

## 📋 Prerequisites

- Rust toolchain (1.83.0)
- Linera CLI tools
- Node.js (>= 18.0.0)
- npm (>= 8.0.0)
- wasm-pack
- CheCko wallet or MetaMask with Linera support

## 🚀 Quick Start

### 1. Build and Deploy Contract

```bash
# Make deployment script executable
chmod +x run.sh

# Run deployment
./run.sh
```

This script will:
- Build WASM contract and service modules
- Initialize wallet with testnet faucet
- Publish and create the application
- Start Linera service on port 8080

**Important**: Save the Application ID printed at the end!

### 2. Set Up Frontend

```bash
# Navigate to webui
cd webui

# Install dependencies
npm install

# Build WASM module for operation serialization
npm run build:wasm

# Start development server
npm run dev
```

The UI will open at `http://localhost:3000`

### 3. Configure Application ID

In your browser console:

```javascript
localStorage.setItem('applicationId', 'YOUR_APPLICATION_ID_HERE');
```

Replace `YOUR_APPLICATION_ID_HERE` with the Application ID from deployment.

## 🎯 How to Play

### Creating a Game

1. Click "Connect" to connect your Linera wallet
2. Click "Create New Game"
3. Share your Chain ID with your opponent

### Joining a Game

1. Get the opponent's Chain ID
2. Click "Join Game"
3. Enter the opponent's Chain ID
4. Click "Join"

### Playing a Round

1. **Choose Move**: Click Rock (🪨), Paper (📄), or Scissors (✂️)
2. **Commit**: Your move is committed (hashed) to the blockchain
3. **Wait**: Wait for opponent to commit their move
4. **Reveal**: Click "Reveal Move" to reveal your actual move
5. **Result**: See the winner and updated scores
6. **Continue**: Click "Play Next Round" or "Leave Game"

## 🔧 Development

### Project Structure

#### Rust Contract

- **lib.rs**: Type definitions, ABI, operations, messages, errors
- **state.rs**: `RockPaperScissorsState` with game state management
- **contract.rs**: Contract implementation with game logic
- **service.rs**: GraphQL service for querying state

#### Frontend

- **RockPaperScissorsGame.vue**: Main game component
- **wasm/**: WASM module for operation serialization (BCS encoding)
- **apollo/**: GraphQL client configuration
- **router/**: Vue Router setup

### Key Technologies

**Backend:**
- Linera SDK 0.15.7
- async-graphql 7.0.17
- blake3 (for hashing)
- BCS serialization

**Frontend:**
- Vue 3.5
- Quasar 2.17
- TypeScript 5.7
- GraphQL + WebSocket
- Web3 4.16

## 🎲 Game Logic

### Move Determination

```
Rock beats Scissors
Paper beats Rock
Scissors beats Paper
Same moves = Tie
```

### Hash Verification

```rust
hash = blake3(move_byte + secret_bytes)

// On reveal:
verify(hash == blake3(revealed_move + secret))
```

### Cross-Chain Messages

- `PlayerJoined`: Notifies opponent that player 2 joined
- `MoveCommitted`: Notifies opponent of committed move
- `MoveRevealed`: Notifies opponent of revealed move
- `RoundResult`: Sends round winner to both chains
- `GameEnded`: Notifies game completion

## 📊 GraphQL API

### Queries

```graphql
query GameState {
  gameStatus
  player1 { owner chain rounds_won }
  player2 { owner chain rounds_won }
  totalRounds
  lastRoundWinner
  lastRoundP1Move
  lastRoundP2Move
  bothCommitted
  bothRevealed
  hasActiveGame
}
```

### Operations (via WASM)

- `createGame()`: Create new game
- `joinGame(opponentChain)`: Join existing game
- `commitMove(move, secret)`: Commit a move
- `revealMove(move, secret)`: Reveal a move
- `playAgain()`: Start new round
- `leaveGame()`: Leave current game

## 🔒 Security Features

1. **Commit-Reveal Pattern**: Prevents move front-running
2. **Hash Verification**: Ensures revealed move matches commitment
3. **Authenticated Signers**: Only game players can make moves
4. **Cross-Chain Verification**: Each chain maintains independent state

## 🐛 Troubleshooting

### Wallet Not Connecting

- Ensure CheCko or MetaMask with Linera support is installed
- Check that `window.linera` is available in console
- Refresh the page and try again

### Application ID Not Found

- Make sure you've set the Application ID in localStorage
- Verify the ID is correct from deployment output

### WASM Module Errors

- Rebuild WASM: `cd webui && npm run build:wasm`
- Clear browser cache and reload

### State Not Updating

- Check that Linera service is running on port 8080
- Verify network connectivity
- Check browser console for errors

## 📝 Testing

```bash
# Run Rust tests
cargo test

# Test game flow
linera query-application <APP_ID> "{ gameStatus }"
```

## 🤝 Contributing

This is a learning project demonstrating Linera blockchain development patterns:
- Multi-chain application architecture
- Cross-chain messaging
- Commit-reveal schemes
- GraphQL service integration
- WASM frontend integration

## 📚 Resources

- [Linera Documentation](https://linera.dev/developers)
- [Linera Protocol GitHub](https://github.com/linera-io/linera-protocol)
- [Cross-Chain Messages Guide](https://linera.dev/developers/backend/messages.html)
- [Microchains Concepts](https://linera.dev/developers/core_concepts/microchains.html)

## 📄 License

MIT License - See LICENSE file for details

## 🎉 Acknowledgments

Built following patterns from the Linera Developer Lessons repository, specifically Lesson 4's CheCko integration approach.

---

**Enjoy playing Rock Paper Scissors on the blockchain! 🚀**
