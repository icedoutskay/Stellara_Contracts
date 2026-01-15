#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, log, symbol_short, vec, Address, Env, Map, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Credential {
    pub id: u64,
    pub holder: Address,
    pub course_id: Symbol,
    pub level: u32,
    pub issued_at: u64,
    pub metadata_uri: Symbol,
}

#[contracttype]
#[derive(Clone)]
pub struct CredentialStats {
    pub total_issued: u64,
    pub next_id: u64,
}

#[contract]
pub struct AcademyContract;

#[contractimpl]
impl AcademyContract {
    /// Initialize the academy contract
    pub fn init(env: Env) -> Result<(), Symbol> {
        let stats = CredentialStats {
            total_issued: 0,
            next_id: 1,
        };
        env.storage().instance().set(&symbol_short!("stats"), &stats);
        log!(&env, "Academy contract initialized");
        Ok(())
    }

    /// Issue a credential to a user
    pub fn issue_credential(
        env: Env,
        admin: Address,
        holder: Address,
        course_id: Symbol,
        level: u32,
        metadata_uri: Symbol,
    ) -> Result<u64, Symbol> {
        admin.require_auth();

        let mut stats: CredentialStats = env
            .storage()
            .instance()
            .get(&symbol_short!("stats"))
            .unwrap();

        let credential_id = stats.next_id;
        let credential = Credential {
            id: credential_id,
            holder: holder.clone(),
            course_id,
            level,
            issued_at: env.ledger().timestamp(),
            metadata_uri,
        };

        // Store credential
        let creds_key = symbol_short!("creds");
        let mut credentials: Vec<Credential> = env
            .storage()
            .instance()
            .get(&creds_key)
            .unwrap_or(vec![&env]);
        credentials.push_back(credential);
        env.storage().instance().set(&creds_key, &credentials);

        // Update stats
        stats.total_issued += 1;
        stats.next_id += 1;
        env.storage().instance().set(&symbol_short!("stats"), &stats);

        log!(&env, "Credential issued: {}", credential_id);
        Ok(credential_id)
    }

    /// Get user's credentials
    pub fn get_user_credentials(env: Env, holder: Address) -> Vec<Credential> {
        let credentials: Vec<Credential> = env
            .storage()
            .instance()
            .get(&symbol_short!("creds"))
            .unwrap_or(vec![&env]);

        let mut user_creds = vec![&env];
        for i in 0..credentials.len() {
            if credentials.get_unchecked(i).holder == holder {
                user_creds.push_back(credentials.get_unchecked(i));
            }
        }
        user_creds
    }

    /// Verify a credential exists
    pub fn verify_credential(env: Env, credential_id: u64) -> bool {
        let credentials: Vec<Credential> = env
            .storage()
            .instance()
            .get(&symbol_short!("creds"))
            .unwrap_or(vec![&env]);

        for i in 0..credentials.len() {
            if credentials.get_unchecked(i).id == credential_id {
                return true;
            }
        }
        false
    }

    /// Get credential statistics
    pub fn get_stats(env: Env) -> CredentialStats {
        env.storage()
            .instance()
            .get(&symbol_short!("stats"))
            .unwrap()
    }
}
