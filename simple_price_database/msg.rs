pub struct InitMsg {

    pub initial_oracle_ref: HumanAddr,

}

pub enum HandleMsg {
    // new_oracle_ref: a new oracle address to be set
    SetOracleRef { new_oracle_ref: HumanAddr },

    // symbol: a symbol that will be used to ask the oracle to get the price
    SavePrice { symbol: String }
}

pub enum QueryMsg {
    // query owner address
    Owner {},

    // query oracle address
    OracleRef {},

    // query price that has been saved
    GetPrice { symbol: String }
}


pub enum QueryExtMsg {
    GetReferenceData {
        base_symbol: String,
        quote_symbol: String,
    }
}