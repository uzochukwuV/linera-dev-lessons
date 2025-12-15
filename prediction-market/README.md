# 🎯 Prediction Market on Linera

A fully decentralized prediction market platform built on Linera blockchain with HTTP oracle integration. Features a stunning Vercel-inspired dark UI.

![Prediction Market](https://img.shields.io/badge/Linera-Blockchain-blue)
![Status](https://img.shields.io/badge/Status-Active-success)

## 🌟 Features

### Core Functionality
- **Create Markets**: Anyone can create prediction markets with custom questions and outcomes
- **Place Bets**: Users can bet on different outcomes with their tokens
- **Automated Resolution**: Markets resolve automatically after deadline
- **Claim Winnings**: Winners can claim their proportional share of the pool
- **Oracle Integration**: Ready for HTTP oracle data fetching (following Linera patterns)

### Technical Features
- **Cross-Chain Ready**: Built with Linera's microchain architecture
- **State Management**: Efficient storage using MapView and RegisterView
- **GraphQL API**: Full query interface for market data
- **Real-time Updates**: Polling-based UI updates
- **Wallet Integration**: Compatible with Linera wallets

## 🎨 UI Design

Modern Vercel-style interface with:
- Dark theme with gradient accents
- Smooth animations and transitions
- Responsive grid layouts
- Glass-morphism effects
- Professional typography
- Intuitive market cards
- Real-time statistics

## 🏗️ Architecture

### Smart Contract

```
prediction-market/
├── src/
│   ├── lib.rs        # ABI, types, operations, messages
│   ├── state.rs      # State management with views
│   ├── contract.rs   # Core market logic
│   └── service.rs    # GraphQL query service
```

**Key Components:**

- **Market Creation**: Users can create markets with multiple outcomes
- **Betting System**: Proportional payout based on pool distribution
- **Resolution Mechanism**: Time-based with oracle support
- **Winnings Distribution**: Automatic calculation and claiming

### State Structure

```rust
pub struct PredictionMarketState {
    next_market_id: RegisterView<u64>,
    markets: MapView<u64, Market>,
    bets: MapView<String, Bet>,
    balances: MapView<Owner, u64>,
    total_markets: RegisterView<u64>,
}
```

### Market Flow

1. **Creation** → Creator sets question, outcomes, resolution time
2. **Betting** → Users place bets on outcomes before deadline
3. **Resolution** → After deadline, creator resolves with winning outcome
4. **Claiming** → Winners claim proportional share of total pool

## 🚀 Quick Start

### Prerequisites

- Rust toolchain (1.83.0)
- Linera CLI tools
- Node.js (>= 20)
- npm (>= 8)

### 1. Build & Deploy Contract

```bash
cd prediction-market

# Build contract
cargo build --release --target wasm32-unknown-unknown

# Deploy
./run.sh
```

The script will:
- Initialize Linera wallet
- Request test chains
- Publish the application
- Start service on port 8080

**Save the Application ID** printed at the end!

### 2. Set Up UI

```bash
cd webui

# Install dependencies
npm install

# Start development server
npm run dev
```

UI will open at `http://localhost:3000`

### 3. Configure Application

In browser console:

```javascript
localStorage.setItem('applicationId', 'YOUR_APPLICATION_ID');
```

Reload the page and start using the app!

## 📖 How to Use

### Creating a Market

1. Click "Connect Wallet"
2. Click "Create Market"
3. Fill in:
   - Question (e.g., "Will BTC reach $100k in 2025?")
   - Outcomes (e.g., "Yes, No")
   - Resolution date
   - Optional: Oracle URL
4. Click "Create Market"

### Placing a Bet

1. Browse active markets
2. Click on a market card
3. Select an outcome
4. Enter bet amount
5. Click "Place Bet"

### Claiming Winnings

1. Wait for market resolution
2. Open resolved market
3. Click "Claim Winnings"
4. Tokens are added to your balance

## 💡 Smart Contract Details

### Operations

```rust
pub enum Operation {
    CreateMarket {
        question: String,
        outcomes: Vec<String>,
        resolution_time: Timestamp,
        oracle_url: Option<String>,
    },
    PlaceBet {
        market_id: u64,
        outcome_index: usize,
        amount: u64,
    },
    ResolveMarket {
        market_id: u64,
        winning_outcome_index: usize,
    },
    ClaimWinnings {
        market_id: u64,
    },
}
```

### GraphQL Queries

```graphql
# Get all markets
query {
  markets {
    id
    question
    outcomes
    status
    total_pool
    outcome_pools
  }
}

# Get active markets
query {
  active_markets {
    id
    question
  }
}

# Get user balance
query {
  balance(owner: "OWNER_ADDRESS")
}

# Get market summary
query {
  market_summary(market_id: 1, owner: "OWNER_ADDRESS") {
    market { ... }
    total_bets
    user_bets { ... }
  }
}
```

## 🔒 Payout Formula

Winners receive proportional payouts:

```
Payout = (Your Bet / Winning Pool) × Total Pool
```

**Example:**
- Total Pool: 1000 tokens
- Winning Pool (Yes): 400 tokens
- Your Bet on Yes: 100 tokens
- Your Payout: (100 / 400) × 1000 = **250 tokens**

## 🌐 Oracle Integration

The contract supports HTTP oracle integration for automated resolution:

```rust
// In contract.rs (ready for implementation)
if let Some(oracle_url) = &market.oracle_url {
    // Fetch result from oracle
    let response = self.runtime.http_request(
        http::Request::get(oracle_url)
    );
    // Validate and use response
}
```

Follows [Linera HTTP request patterns](https://github.com/linera-io/linera-protocol/blob/main/examples/how-to/perform-http-requests/src/contract.rs).

## 🎨 UI Components

### Market Card
- Question display
- Status badge (Active/Resolved)
- Pool information
- Outcome buttons with amounts
- Hover animations

### Stats Dashboard
- Total markets
- User balance
- Active markets count

### Dialogs
- Create market form
- Market detail with betting
- Winnings claim interface

## 🧪 Testing

```bash
# Run contract tests
cargo test

# Test GraphQL queries
linera query-application <APP_ID> "{ markets { id question } }"
```

## 🛠️ Development

### Adding New Features

1. **Market Types**: Extend `MarketStatus` enum
2. **Betting Options**: Modify `Operation::PlaceBet`
3. **Oracle Sources**: Implement HTTP fetching in resolution
4. **UI Themes**: Edit `webui/src/css/app.css`

### Environment Setup

```bash
# Contract development
cargo watch -x "build --release --target wasm32-unknown-unknown"

# UI development
cd webui && npm run dev
```

## 📊 Future Enhancements

- [ ] Multi-option markets (>2 outcomes)
- [ ] Automated oracle resolution
- [ ] Market categories and filtering
- [ ] Historical market data
- [ ] User profiles and statistics
- [ ] Market liquidity pools
- [ ] Social features (comments, shares)
- [ ] Mobile responsive improvements
- [ ] Trading charts and analytics

## 🐛 Troubleshooting

**Wallet Not Connecting**
- Ensure Linera wallet is installed
- Check `window.linera` is available
- Try refreshing the page

**Application ID Not Found**
- Verify localStorage has correct ID
- Check contract deployment completed
- Ensure service is running on port 8080

**Bets Not Showing**
- Wait for blockchain confirmation
- Check you're on the correct chain
- Refresh data manually

## 📚 Resources

- [Linera Documentation](https://linera.dev/developers)
- [HTTP Requests Example](https://github.com/linera-io/linera-protocol/blob/main/examples/how-to/perform-http-requests)
- [GraphQL Integration](https://linera.dev/developers/backend/service.html)
- [Linera SDK Reference](https://docs.rs/linera-sdk)

## 📄 License

MIT License - See LICENSE file

## 🤝 Contributing

Contributions welcome! This is a demonstration project for Linera blockchain development.

## 🎉 Acknowledgments

- Built with Linera SDK 0.15.7
- UI inspired by Vercel design system
- Quasar Framework for Vue components
- GraphQL for query interface

---

**Happy Predicting! 🚀**
