# Satsuma: Cross-Chain Yield-to-Bitcoin Conversion Platform

A sophisticated DeFi platform that enables users to deposit yield-bearing stablecoins and automatically convert the generated yield to Bitcoin across multiple blockchain ecosystems.

## 🏗️ Architecture Overview

Satsuma operates across two major blockchain ecosystems:

1. **Injective Protocol** - For yield generation and yield skimming
2. **Internet Computer (ICP)** - For yield accumulation and Bitcoin conversion

### Cross-Chain Flow

```
User Deposit (USDC) → Injective Vault → nUSDC (yield-bearing)
                                                    ↓
Yield Generation → Yield Skimming → USDC → Axelar GMP → ICP Canister
                                                    ↓
                                            Bitcoin Conversion
```

## 🔧 Technical Components

### 1. Injective Smart Contract (`contracts/injective/src/lib.rs`)

**Purpose**: Yield-skimming vault that manages USDC deposits and converts yield to Bitcoin

**Key Features**:
- Accepts USDC deposits and swaps them to nUSDC (yield-bearing token)
- Tracks principal balances per user
- Implements yield skimming mechanism
- Bridges yield to ICP via Axelar GMP (General Message Passing)
- **Complete Astroport Integration**: Full swap implementation for USDC ↔ nUSDC

**Core Functions**:
```rust
// Deposit USDC → swap to nUSDC → track principal
fn execute_deposit(amount: Uint128) -> Response

// Skim yield: nUSDC → USDC → bridge to ICP
fn execute_skim() -> Response

// ICP-managed operations
fn execute_from_icp(action: IcpAction) -> Response
```

**Astroport Integration**:
- Complete swap message structures for USDC ↔ nUSDC
- Proper approval flow for Astroport router
- Configurable pool IDs for different trading pairs
- Slippage protection and minimum receive amounts

### 2. ICP Canister (`contracts/icp/src/lib.rs`)

**Purpose**: Receives and accumulates yield from Injective, manages Bitcoin conversion

**Key Features**:
- Simple balance tracking per principal
- Receives yield via cross-chain messages
- **Bitcoin Conversion Engine**: Automatic yield-to-Bitcoin conversion
- **Cross-Chain Management**: Controls Injective operations from ICP
- Provides query interface for balance checking

**Core Functions**:
```rust
// Receive yield from Injective
fn deposit_yield(amount: u128)

// Convert yield to Bitcoin
fn convert_yield_to_bitcoin()

// Manual Bitcoin conversion
fn manual_bitcoin_conversion(principal: String, usdc_amount: u128)

// Manage Injective operations
fn execute_injective_deposit(user: String, amount: u128)
fn trigger_injective_yield_skim(recipient: &str)
```

### 3. Frontend Application (`frontend/`)

**Technology Stack**: Next.js 15, React 19, TypeScript, Tailwind CSS

**Key Features**:
- **Dual Wallet Support**: ICP (Internet Identity) + Injective wallets
- **Cross-Chain Service**: Unified interface for both chains
- Real-time portfolio tracking with yield visualization
- Bitcoin accumulation charts and analytics
- Transaction history and status tracking

## 🚀 Complete Astroport Implementation

### Swap Message Structures

```rust
// USDC → nUSDC Swap
pub struct AstroportSwapMsg {
    pub swap: AstroportSwap,
}

pub struct AstroportSwap {
    pub offer_asset: OfferAsset,
    pub ask_asset_info: AssetInfo,
    pub minimum_receive: Option<Uint128>,
}

pub struct OfferAsset {
    pub info: AssetInfo,
    pub amount: Uint128,
}

pub enum AssetInfo {
    Token { contract_addr: String },
    NativeToken { denom: String },
}
```

### Complete Swap Flow

1. **User deposits USDC** → Contract receives USDC
2. **Approve Astroport router** → Allow router to spend USDC
3. **Execute swap** → USDC → nUSDC via Astroport
4. **Track principal** → Update user's principal balance
5. **Yield generation** → nUSDC generates yield over time
6. **Yield skimming** → Convert excess nUSDC back to USDC
7. **Bridge to ICP** → Send yield to ICP via Axelar GMP

## 🔄 ICP Management of Injective Operations

### Cross-Chain Control Flow

The ICP canister acts as the central controller for Injective operations:

```rust
// ICP can trigger Injective operations
pub enum IcpAction {
    Deposit { user: String, amount: Uint128 },
    SkimYield { recipient: String },
    UpdateConfig { config: Config },
}

// Injective contract accepts ICP-managed operations
fn execute_from_icp(action: IcpAction) -> Response {
    match action {
        IcpAction::Deposit { user, amount } => {
            // Execute deposit on Injective from ICP
        },
        IcpAction::SkimYield { recipient } => {
            // Trigger yield skimming with custom recipient
        },
        IcpAction::UpdateConfig { config } => {
            // Update Injective contract configuration
        },
    }
}
```

### Benefits of ICP Management

1. **Centralized Control**: All operations managed from ICP
2. **Cross-Chain Coordination**: Seamless integration between chains
3. **Security**: ICP canister controls critical operations
4. **Flexibility**: Easy to add new cross-chain operations

## 📊 User Experience Flow

### 1. Deposit Process
```
User → Frontend → ICP Canister → Injective Contract → nUSDC
```

### 2. Yield Generation
```
nUSDC → Yield Generation → Automatic Yield Tracking
```

### 3. Yield Skimming
```
Yield Detection → nUSDC → USDC → Axelar Bridge → ICP
```

### 4. Bitcoin Conversion
```
ICP Canister → Yield Accumulation → Bitcoin Conversion → User Balance
```

### 5. Portfolio Tracking
```
Real-time Updates → Cross-Chain Balances → Bitcoin Accumulation Charts
```

## 🛠️ Development Setup

### Prerequisites
- Node.js 18+
- Rust 1.70+
- DFX (Internet Computer SDK)
- Injective CLI

### Installation

1. **Clone Repository**
```bash
git clone https://github.com/your-org/satsuma.git
cd satsuma
```

2. **Install Dependencies**
```bash
npm install
cd frontend && npm install
```

3. **Build Contracts**
```bash
# Build Injective contract
npm run build:injective

# Build ICP canister
npm run build:icp
```

4. **Deploy Contracts**
```bash
# Deploy ICP canister
dfx start --background
dfx deploy

# Deploy Injective contract (requires Injective network)
# Follow Injective deployment guide
```

5. **Start Frontend**
```bash
cd frontend
npm run dev
```

## 🔧 Configuration

### Environment Variables

```bash
# Frontend (.env.local)
NEXT_PUBLIC_ICP_CANISTER_ID=icp_yield_vault
NEXT_PUBLIC_INJECTIVE_CONTRACT_ADDRESS=inj1...
NEXT_PUBLIC_AXELAR_GATEWAY=axelar1...
NEXT_PUBLIC_BITCOIN_PRICE_ORACLE=oracle1...
```

### Contract Configuration

**Injective Contract**:
```rust
pub struct Config {
    pub token_usdc: String,
    pub token_nusdc: String,
    pub astroport_router: String,
    pub axelar_gateway: String,
    pub icp_canister_id: String,
    pub yield_collector: Addr,
    pub pool_id_usdc_nusdc: u64,
    pub pool_id_nusdc_usdc: u64,
}
```

**ICP Canister**:
```rust
pub struct InjectiveConfig {
    pub contract_address: String,
    pub axelar_gateway: String,
    pub yield_collector: String,
    pub bitcoin_price_oracle: String,
}
```

## 🧪 Testing

### Contract Testing
```bash
# Test Injective contract
cd contracts/injective
cargo test

# Test ICP canister
cd contracts/icp
dfx canister call icp_yield_vault my_balance
```

### Frontend Testing
```bash
cd frontend
npm run test
npm run lint
```

## 🔒 Security Features

1. **Access Control**: Only authorized principals can manage operations
2. **Cross-Chain Verification**: Validates cross-chain message sources
3. **Slippage Protection**: Configurable minimum receive amounts
4. **Emergency Withdrawals**: Users can withdraw funds in emergencies
5. **Principal Protection**: Only yield is converted, principal remains safe

## 📈 Performance Optimizations

1. **Batch Operations**: Multiple operations in single transaction
2. **Gas Optimization**: Efficient contract calls and storage
3. **Cross-Chain Efficiency**: Minimal bridge operations
4. **Real-time Updates**: WebSocket connections for live data

## 🚀 Deployment

### Production Checklist

- [ ] Security audit completed
- [ ] All tests passing
- [ ] Configuration validated
- [ ] Cross-chain bridges tested
- [ ] Monitoring and alerting configured
- [ ] Documentation updated

### Network-Specific Deployment

**Injective Mainnet**:
```bash
# Deploy to Injective mainnet
injectived tx wasm store satsuma_injective_yield_vault.wasm \
  --from <key> --chain-id injective-1 --gas-prices 500000000inj
```

**ICP Mainnet**:
```bash
# Deploy to ICP mainnet
dfx deploy --network ic
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

MIT License - see LICENSE file for details

## 🆘 Support

- Documentation: [docs.satsuma.com](https://docs.satsuma.com)
- Discord: [discord.gg/satsuma](https://discord.gg/satsuma)
- GitHub Issues: [github.com/your-org/satsuma/issues](https://github.com/your-org/satsuma/issues)

---

**Satsuma**: Building the future of cross-chain yield optimization and Bitcoin accumulation. 