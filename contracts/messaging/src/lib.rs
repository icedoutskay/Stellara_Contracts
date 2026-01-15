#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, log, symbol_short, vec, Address, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Message {
    pub id: u64,
    pub sender: Address,
    pub recipient: Address,
    pub content_hash: Symbol,
    pub timestamp: u64,
    pub is_read: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct MessageStats {
    pub total_messages: u64,
    pub next_message_id: u64,
}

#[contract]
pub struct MessagingContract;

#[contractimpl]
impl MessagingContract {
    /// Initialize the messaging contract
    pub fn init(env: Env) -> Result<(), Symbol> {
        let stats = MessageStats {
            total_messages: 0,
            next_message_id: 1,
        };
        env.storage().instance().set(&symbol_short!("stats"), &stats);
        log!(&env, "Messaging contract initialized");
        Ok(())
    }

    /// Send a message
    pub fn send_message(
        env: Env,
        sender: Address,
        recipient: Address,
        content_hash: Symbol,
    ) -> Result<u64, Symbol> {
        sender.require_auth();

        let mut stats: MessageStats = env
            .storage()
            .instance()
            .get(&symbol_short!("stats"))
            .unwrap();

        let message_id = stats.next_message_id;
        let message = Message {
            id: message_id,
            sender: sender.clone(),
            recipient: recipient.clone(),
            content_hash,
            timestamp: env.ledger().timestamp(),
            is_read: false,
        };

        // Store message
        let messages_key = symbol_short!("msgs");
        let mut messages: Vec<Message> = env
            .storage()
            .instance()
            .get(&messages_key)
            .unwrap_or(vec![&env]);
        messages.push_back(message);
        env.storage().instance().set(&messages_key, &messages);

        // Update stats
        stats.total_messages += 1;
        stats.next_message_id += 1;
        env.storage().instance().set(&symbol_short!("stats"), &stats);

        log!(&env, "Message sent: {}", message_id);
        Ok(message_id)
    }

    /// Mark message as read
    pub fn mark_as_read(env: Env, user: Address, message_id: u64) -> Result<(), Symbol> {
        user.require_auth();

        let messages_key = symbol_short!("msgs");
        let mut messages: Vec<Message> = env
            .storage()
            .instance()
            .get(&messages_key)
            .ok_or(symbol_short!("notfnd"))?;

        for i in 0..messages.len() {
            if messages.get_unchecked(i).id == message_id {
                let mut msg = messages.get_unchecked(i);
                if msg.recipient == user {
                    msg.is_read = true;
                    messages.set(i, msg);
                    env.storage().instance().set(&messages_key, &messages);
                    return Ok(());
                }
            }
        }
        Err(symbol_short!("invld"))
    }

    /// Get user's messages
    pub fn get_messages(env: Env, user: Address, limit: u32) -> Vec<Message> {
        let messages: Vec<Message> = env
            .storage()
            .instance()
            .get(&symbol_short!("msgs"))
            .unwrap_or(vec![&env]);

        let mut user_messages = vec![&env];
        let mut count = 0;
        for i in (0..messages.len()).rev() {
            if count >= limit {
                break;
            }
            let msg = messages.get_unchecked(i);
            if msg.recipient == user || msg.sender == user {
                user_messages.push_back(msg);
                count += 1;
            }
        }
        user_messages
    }

    /// Get unread message count
    pub fn get_unread_count(env: Env, user: Address) -> u32 {
        let messages: Vec<Message> = env
            .storage()
            .instance()
            .get(&symbol_short!("msgs"))
            .unwrap_or(vec![&env]);

        let mut count = 0;
        for i in 0..messages.len() {
            let msg = messages.get_unchecked(i);
            if msg.recipient == user && !msg.is_read {
                count += 1;
            }
        }
        count
    }

    /// Get messaging statistics
    pub fn get_stats(env: Env) -> MessageStats {
        env.storage()
            .instance()
            .get(&symbol_short!("stats"))
            .unwrap()
    }
}
