# Stellara Smart Contracts - Detailed Documentation

## Contract Architecture

All contracts follow Soroban best practices and are optimized for the Testnet environment.

### Design Patterns

1. **Contract Initialization**: All contracts require explicit initialization before use
2. **Authentication**: Functions requiring authorization use `require_auth()` for security
3. **Data Storage**: Persistent state stored in contract instance storage
4. **Error Handling**: Using Symbol-based error codes for gas efficiency

## Trading Contract

### Purpose
Enables decentralized exchange of cryptocurrency pairs with trade history tracking.

### State Variables
- `stats`: TradeStats - Global trading statistics
- `trades`: Vec<Trade> - Complete trade history

### Key Structs

```rust
pub struct Trade {
    pub id: u64,
    pub trader: Address,
    pub pair: Symbol,          // e.g., "USDT" 
    pub amount: i128,          // Amount being traded
    pub price: i128,           // Price per unit
    pub timestamp: u64,        // Ledger timestamp
    pub is_buy: bool,          // Buy vs Sell order
}

pub struct TradeStats {
    pub total_trades: u64,
    pub total_volume: i128,
    pub last_trade_id: u64,
}
