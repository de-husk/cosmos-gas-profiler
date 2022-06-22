# Cosm-Orc


Rust Cosmwasm smart contract orchestration and gas profiling library.

Store, instantiate, execute, and query [Cosmwasm](https://github.com/CosmWasm/cosmwasm) smart contracts against a configured [Cosmos](https://github.com/cosmos/cosmos-sdk) based chain. 

Optionally, profile gas usage of the smart contract operations.

Potential uses:
* Integration tests
* Deployments / Bootstrapping environments
* Gas profiling

## Quick Start
 ```rust
    use anyhow::Result;
    use cosm_orc::{
        config::config::Config,
        orchestrator::cosm_orc::{CosmOrc, WasmMsg},
    };
    use cw20_base::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

    fn main() -> Result<()> {
        // juno_cfg.yaml has the cw20_base code_id already stored
        // If the smart contract has not been stored on the chain yet use: `cosm_orc::store_contracts()`
        let mut cosm_orc = CosmOrc::new(Config::from_yaml("./examples/juno_cfg.yaml")?);

        let msgs: Vec<WasmMsg<InstantiateMsg, ExecuteMsg, QueryMsg>> = vec![
            WasmMsg::InstantiateMsg(InstantiateMsg {
                name: "Meme Token".to_string(),
                symbol: "MEME".to_string(),
                decimals: 6,
                initial_balances: vec![],
                mint: None,
                marketing: None,
            }),
            WasmMsg::QueryMsg(QueryMsg::TokenInfo {}),
        ];

        cosm_orc.process_msgs("cw20_base".to_string(), &msgs)?;

        Ok(())
  }
```


## Configuration

### Yaml File

### Environment Variables

### Manual Struct Config

## Usage

### Github Action

### Local Usage

