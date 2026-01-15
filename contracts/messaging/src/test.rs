#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_messaging_contract_init() {
        let env = Env::default();
        let contract = MessagingContract;
        
        let result = contract.init(&env);
        assert!(result.is_ok());
        
        let stats = contract.get_stats(&env);
        assert_eq!(stats.total_messages, 0);
        assert_eq!(stats.next_message_id, 1);
    }

    #[test]
    fn test_send_message() {
        let env = Env::default();
        let contract = MessagingContract;
        
        contract.init(&env).unwrap();
        
        let sender = soroban_sdk::Address::random(&env);
        let recipient = soroban_sdk::Address::random(&env);
        let content_hash = soroban_sdk::symbol_short!("hash");
        
        let msg_id = contract
            .send_message(&env, sender, recipient, content_hash)
            .unwrap();
        
        assert_eq!(msg_id, 1);
        
        let stats = contract.get_stats(&env);
        assert_eq!(stats.total_messages, 1);
    }

    #[test]
    fn test_mark_as_read() {
        let env = Env::default();
        let contract = MessagingContract;
        
        contract.init(&env).unwrap();
        
        let sender = soroban_sdk::Address::random(&env);
        let recipient = soroban_sdk::Address::random(&env);
        
        let msg_id = contract
            .send_message(&env, sender, recipient.clone(), soroban_sdk::symbol_short!("hash"))
            .unwrap();
        
        let result = contract.mark_as_read(&env, recipient, msg_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unread_count() {
        let env = Env::default();
        let contract = MessagingContract;
        
        contract.init(&env).unwrap();
        
        let sender = soroban_sdk::Address::random(&env);
        let recipient = soroban_sdk::Address::random(&env);
        
        // Send 3 messages
        for _ in 0..3 {
            contract
                .send_message(&env, sender.clone(), recipient.clone(), soroban_sdk::symbol_short!("hash"))
                .unwrap();
        }
        
        let unread = contract.get_unread_count(&env, recipient);
        assert_eq!(unread, 3);
    }

    #[test]
    fn test_get_messages() {
        let env = Env::default();
        let contract = MessagingContract;
        
        contract.init(&env).unwrap();
        
        let sender = soroban_sdk::Address::random(&env);
        let recipient = soroban_sdk::Address::random(&env);
        
        for _ in 0..5 {
            contract
                .send_message(&env, sender.clone(), recipient.clone(), soroban_sdk::symbol_short!("hash"))
                .unwrap();
        }
        
        let messages = contract.get_messages(&env, recipient.clone(), 10);
        assert_eq!(messages.len(), 5);
    }
}
