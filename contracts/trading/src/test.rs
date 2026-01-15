#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_trading_contract_init() {
        let env = Env::default();
        let contract = TradingContract;
        
        let result = contract.init(&env);
        assert!(result.is_ok());
        
        let stats = contract.get_stats(&env);
        assert_eq!(stats.total_trades, 0);
        assert_eq!(stats.total_volume, 0);
        assert_eq!(stats.last_trade_id, 0);
    }

    #[test]
    fn test_execute_trade() {
        let env = Env::default();
        let contract = TradingContract;
        
        contract.init(&env).unwrap();
        
        let trader = soroban_sdk::Address::random(&env);
        let pair = soroban_sdk::symbol_short!("USDT");
        let amount = 1000i128;
        let price = 100i128;
        
        let trade_id = contract
            .execute_trade(&env, trader.clone(), pair, amount, price, true)
            .unwrap();
        
        assert_eq!(trade_id, 1);
        
        let stats = contract.get_stats(&env);
        assert_eq!(stats.total_trades, 1);
        assert_eq!(stats.total_volume, 1000);
    }

    #[test]
    fn test_get_user_trades() {
        let env = Env::default();
        let contract = TradingContract;
        
        contract.init(&env).unwrap();
        
        let trader = soroban_sdk::Address::random(&env);
        let pair = soroban_sdk::symbol_short!("USDT");
        
        // Execute multiple trades
        for i in 0..3 {
            contract
                .execute_trade(&env, trader.clone(), pair, (i + 1) as i128 * 100, 100, true)
                .unwrap();
        }
        
        let trades = contract.get_user_trades(&env, trader, 10);
        assert_eq!(trades.len(), 3);
    }
}
