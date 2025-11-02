#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env, Map, Symbol, Vec};

#[contract]
pub struct Contract;

const KEY_OWNER: Symbol = Symbol::short("OWNER");
const KEY_ENVELOPES: Symbol = Symbol::short("ENVS");
const KEY_GUARDIANS: Symbol = Symbol::short("GUARD");
const KEY_RECOVERY_THRESHOLD: Symbol = Symbol::short("R_TH");
const KEY_RECOVERY_DELAY: Symbol = Symbol::short("R_DL");

#[contracttype]
#[derive(Clone)]
pub struct VestSlice {
    pub ts: i64,
    pub bp: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct Envelope {
    pub beneficiary: Address,
    pub amount: i128,
    pub secret_hash: BytesN<32>,
    pub unlock_ts: Option<i64>,
    pub vesting: Vec<VestSlice>,
    pub claimed: i128,
    pub expiry_ts: Option<i64>,
    pub revoked: bool,
}

impl Contract {
    fn envelopes_map(env: &Env) -> Map<BytesN<32>, Envelope> {
        env.storage()
            .instance()
            .get(&KEY_ENVELOPES)
            .unwrap_or(Map::new(env))
    }

    fn save_envelopes(env: &Env, m: &Map<BytesN<32>, Envelope>) {
        env.storage().instance().set(&KEY_ENVELOPES, m);
    }

    fn owner_address(env: &Env) -> Address {
        env.storage()
            .instance()
            .get(&KEY_OWNER)
            .expect("owner not set")
    }

    fn now(env: &Env) -> u64 {
        env.ledger().timestamp()
    }
}

#[contractimpl]
impl Contract {
    pub fn initialize(
        env: Env,
        owner: Address,
        guardians: Vec<Address>,
        recovery_threshold: u32,
        recovery_delay: u64,
    ) {
        if env.storage().instance().has(&KEY_OWNER) {
            panic!("already initialized");
        }
        env.storage().instance().set(&KEY_OWNER, &owner);
        env.storage().instance().set(&KEY_GUARDIANS, &guardians);
        env.storage().instance().set(&KEY_RECOVERY_THRESHOLD, &recovery_threshold);
        env.storage().instance().set(&KEY_RECOVERY_DELAY, &recovery_delay);
        env.storage().instance().set(&KEY_ENVELOPES, &Map::<BytesN<32>, Envelope>::new(&env));
    }

    pub fn create_envelope(
        env: Env,
        envelope_id: BytesN<32>,
        beneficiary: Address,
        amount: i128,
        secret_hash: BytesN<32>,
        unlock_ts: Option<u64>,
        vesting: Vec<VestSlice>,
        expiry_ts: Option<u64>,
    ) {
        let owner = Contract::owner_address(&env);
        owner.require_auth();
        
        if amount <= 0 {
            panic!("amount must be > 0");
        }

        let mut m = Contract::envelopes_map(&env);
        if m.get(envelope_id.clone()).is_some() {
            panic!("envelope id exists");
        }

        let env_rec = Envelope {
            beneficiary,
            amount,
            secret_hash,
            unlock_ts: unlock_ts.map(|ts| ts as i64),
            vesting,
            claimed: 0,
            expiry_ts: expiry_ts.map(|ts| ts as i64),
            revoked: false,
        };
        m.set(envelope_id, env_rec);
        Contract::save_envelopes(&env, &m);
    }

    pub fn claim(env: Env, envelope_id: BytesN<32>, provided_secret_hash: BytesN<32>) -> i128 {
        let mut m = Contract::envelopes_map(&env);
        let env_rec = m.get(envelope_id.clone()).expect("envelope not found");

        if env_rec.revoked {
            panic!("envelope revoked");
        }

        env_rec.beneficiary.require_auth();

        if provided_secret_hash != env_rec.secret_hash {
            panic!("invalid secret");
        }

        if let Some(unlock_ts) = env_rec.unlock_ts {
            if (Contract::now(&env) as i64) < unlock_ts {
                panic!("locked until unlock_ts");
            }
        }

        let now_ts = Contract::now(&env) as i64;
        let mut sum_bp: u32 = 0;
        
        if env_rec.vesting.is_empty() {
            sum_bp = 10_000;
        } else {
            for vs in env_rec.vesting.iter() {
                if vs.ts <= now_ts {
                    sum_bp = sum_bp.saturating_add(vs.bp);
                }
            }
            if sum_bp > 10_000 {
                sum_bp = 10_000;
            }
        }

        let vested_amount = (env_rec.amount * sum_bp as i128) / 10_000i128;
        if vested_amount <= env_rec.claimed {
            return 0;
        }
        let delta = vested_amount - env_rec.claimed;

        let mut updated_env = env_rec.clone();
        updated_env.claimed = vested_amount;
        m.set(envelope_id, updated_env);
        Contract::save_envelopes(&env, &m);

        delta
    }

    pub fn revoke_envelope(env: Env, envelope_id: BytesN<32>) {
        let owner = Contract::owner_address(&env);
        owner.require_auth();

        let mut m = Contract::envelopes_map(&env);
        let env_rec = m.get(envelope_id.clone()).expect("envelope not found");
        
        if env_rec.revoked {
            panic!("already revoked");
        }
        if env_rec.claimed >= env_rec.amount {
            panic!("already fully claimed");
        }
        
        let mut updated_env = env_rec.clone();
        updated_env.revoked = true;
        m.set(envelope_id, updated_env);
        Contract::save_envelopes(&env, &m);
    }

    pub fn refund_owner(env: Env, envelope_id: BytesN<32>) -> i128 {
        let owner = Contract::owner_address(&env);
        owner.require_auth();

        let mut m = Contract::envelopes_map(&env);
        let env_rec = m.get(envelope_id.clone()).expect("envelope not found");

        if let Some(exp) = env_rec.expiry_ts {
            if (Contract::now(&env) as i64) < exp {
                panic!("not yet expired");
            }
        } else {
            panic!("no expiry set");
        }

        if env_rec.revoked {
            panic!("already revoked");
        }

        let unclaimed = env_rec.amount - env_rec.claimed;
        if unclaimed <= 0 {
            return 0;
        }

        let mut updated_env = env_rec.clone();
        updated_env.revoked = true;
        updated_env.claimed = env_rec.amount;
        m.set(envelope_id, updated_env);
        Contract::save_envelopes(&env, &m);

        unclaimed
    }

    pub fn get_envelope(env: Env, envelope_id: BytesN<32>) -> Envelope {
        let m = Contract::envelopes_map(&env);
        m.get(envelope_id).expect("envelope not found")
    }
}