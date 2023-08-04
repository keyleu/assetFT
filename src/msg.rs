use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub symbol: String,          // "MYTOKEN"
    pub subunit: String,         // "umytoken"
    pub precision: u32,          // 10   1 MY TOKEN = 10000000000 umytoken
    pub initial_amount: Uint128, // 150
    pub description: Option<String>,
    pub features: Option<Vec<u32>>,           // [0, 1, 4]
    pub burn_rate: Option<String>,            // 0.1
    pub send_commission_rate: Option<String>, // 0.2
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint {
        amount: u128,
    },
    Burn {
        amount: u128,
    },
    Freeze {
        account: String,
        amount: u128,
    },
    Unfreeze {
        account: String,
        amount: u128,
    },
    GloballyFreeze {},
    GloballyUnfreeze {},
    SetWhitelistedLimit {
        account: String,
        amount: u128,
    },
    // custom message we use to show the submission of multiple messages
    MintAndSend {
        account: String,
        amount: u128,
    },
    UpgradeTokenV1 {
        ibc_enabled: bool,
    },
    // mint and ibc
    MintAndIbc {
        amount: u128,
        
        channel_id: String,
        /// address on the remote chain to receive these tokens
        to_address: String,
        /// when packet times out, measured on remote chain
        timeout: u64,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Params {},
    Token {},
    Tokens { issuer: String },
    Balance { account: String },
    FrozenBalances { account: String },
    FrozenBalance { account: String },
    WhitelistedBalances { account: String },
    WhitelistedBalance { account: String },
}
