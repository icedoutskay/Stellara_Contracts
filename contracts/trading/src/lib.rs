#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, log, symbol_short, vec, Address, Env, Map, Symbol, Val};

#[contracttype]
#[derive(Clone)]
pub struct Trade {
    pub id: u64,
    pub trader: Address,
    pub pair: Symbol,
    pub amount: i128,
    pub price: i128,
    pub timestamp: u64,
    pub is_buy: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct TradeStats {
    pub total_trades: u64,
    pub total_volume: i128,
    pub last_trade_id: u64,
}

#[contract]
pub struct TradingContract;

#[contractimpl]
impl TradingContract {
    /// Initialize the trading contract
    pub fn init(env: Env) -> Result<(), Symbol> {
        let stats = TradeStats {
            total_trades: 0,
            total_volume: 0,
            last_trade_id: 0,
        };
        env.storage().instance().set(&symbol_short!("stats"), &stats);
        log!(&env, "Trading contract initialized");
        Ok(())
    }

    /// Execute a trade on the DEX
    pub fn execute_trade(
        env: Env,
        trader: Address,
        pair: Symbol,
        amount: i128,
        price: i128,
        is_buy: bool,
    ) -> Result<u64, Symbol> {
        trader.require_auth();

        let mut stats: TradeStats = env
            .storage()
            .instance()
            .get(&symbol_short!("stats"))
            .unwrap_or(TradeStats {
                total_trades: 0,
                total_volume: 0,
                last_trade_id: 0,
            });

        let trade_id = stats.last_trade_id + 1;
        let trade = Trade {
            id: trade_id,
            trader: trader.clone(),
            pair,
            amount,
            price,
            timestamp: env.ledger().timestamp(),
            is_buy,
        };

        // Store trade in persistent storage
        let trades_key = symbol_short!("trades");
        let mut trades: Vec<Trade> = env
            .storage()
            .instance()
            .get(&trades_key)
            .unwrap_or(vec![&env]);
        trades.push_back(trade);
        env.storage().instance().set(&trades_key, &trades);

        // Update stats
        stats.total_trades += 1;
        stats.total_volume += amount;
        stats.last_trade_id = trade_id;
        env.storage().instance().set(&symbol_short!("stats"), &stats);

        log!(&env, "Trade executed: {}", trade_id);
        Ok(trade_id)
    }

    /// Get trade statistics
    pub fn get_stats(env: Env) -> TradeStats {
        env.storage()
            .instance()
            .get(&symbol_short!("stats"))
            .unwrap_or(TradeStats {
                total_trades: 0,
                total_volume: 0,
                last_trade_id: 0,
            })
    }

    /// Get user's recent trades
    pub fn get_user_trades(env: Env, trader: Address, limit: u32) -> Vec<Trade> {
        let trades: Vec<Trade> = env
            .storage()
            .instance()
            .get(&symbol_short!("trades"))
            .unwrap_or(vec![&env]);

        let mut user_trades = vec![&env];
        let mut count = 0;
        for i in (0..trades.len()).rev() {
            if count >= limit {
                break;
            }
            if trades.get_unchecked(i).trader == trader {
                user_trades.push_back(trades.get_unchecked(i));
                count += 1;
            }
        }
        user_trades
    }
}
