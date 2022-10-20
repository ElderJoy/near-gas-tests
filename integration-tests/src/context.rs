#[cfg(test)]
pub mod tests {
    use near_sdk::Timestamp;
    use serde_json::json;
    use workspaces::{network::Sandbox, Contract, Worker};

    pub(crate) const INITIAL_GREETING: &str = "Initial message";

    pub struct IntegrationTestContext<T> {
        pub worker: Worker<T>,
        pub stake_contract: Contract,
        pub rewards_contract: Contract,
    }

    impl IntegrationTestContext<Sandbox> {
        // Return type is the worker, veorder contract
        pub async fn new() -> anyhow::Result<IntegrationTestContext<Sandbox>> {
            println!("Connecting to sandbox!");
            let worker = workspaces::sandbox().await?;
            let stake_contract = worker
                .dev_deploy(include_bytes!(
                    "../../target/wasm32-unknown-unknown/release/stake.wasm"
                ))
                .await?;

            // init the near pool contract
            println!("Initializing the staking contract!");
            stake_contract
                .call("new")
                .args_json(json!({ "message": INITIAL_GREETING }))
                .transact()
                .await?
                .into_result()?;

            println!("Initialized the staking contract!");

            println!("Initializing the rewards contract!");
            let rewards_contract = worker
                .dev_deploy(include_bytes!(
                    "../../target/wasm32-unknown-unknown/release/rewards.wasm"
                ))
                .await?;
            let res = rewards_contract
                .call("new")
                .args_json((stake_contract.id(),))
                .max_gas()
                .transact()
                .await?;
            assert!(res.is_success());
            println!("Initialized the rewards contract!");

            Ok(IntegrationTestContext {
                worker,
                stake_contract,
                rewards_contract,
            })
        }

        pub async fn _get_block_timestamp(&self) -> Timestamp {
            self.worker.view_latest_block().await.unwrap().timestamp()
        }

        pub async fn get_greeting(&self) -> workspaces::Result<String> {
            self.stake_contract
                .call("get_greeting")
                .view()
                .await?
                .json()
        }
    }
}
