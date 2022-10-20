#[cfg(test)]
pub mod tests {
    use near_sdk::Timestamp;
    use near_units::parse_gas;
    use serde_json::json;
    use workspaces::{network::Sandbox, Account, Contract, Worker};

    pub(crate) const INITIAL_GREETING: &str = "Initial message";
    pub(crate) const GREETING1: &str = "Greeting 1";

    pub struct IntegrationTestContext<T> {
        pub worker: Worker<T>,
        pub stake_contract: Contract,
        pub rewards_contract: Contract,
        pub user1: Account,
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
                .call("init")
                .args_json((stake_contract.id(),))
                .max_gas()
                .transact()
                .await?;
            assert!(res.is_success());
            println!("Initialized the rewards contract!");

            let user1 = worker.dev_create_account().await?;

            Ok(IntegrationTestContext {
                worker,
                stake_contract,
                rewards_contract,
                user1,
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

        pub async fn set_greeting(&self) -> workspaces::Result<()> {
            let res = self
                .user1
                .call(self.stake_contract.id(), "set_greeting")
                .args_json((GREETING1,))
                .gas(parse_gas!("1 T") as u64)
                .transact()
                .await?
                .into_result()?;
            println!("{:?}", res);

            Ok(())
        }

        pub async fn set_greeting_as_view(&self) -> workspaces::Result<()> {
            let res = self
                .user1
                .call(self.stake_contract.id(), "set_greeting")
                .args_json((GREETING1,))
                .gas(parse_gas!("1 T") as u64)
                .view()
                .await?;
            println!("{:?}", res);

            Ok(())
        }

        pub async fn cross_get_greeting(&self) -> workspaces::Result<String> {
            let res = self
                .user1
                .call(self.rewards_contract.id(), "query_greeting")
                .max_gas()
                .transact()
                .await?;
            if res.is_failure() {
                println!("{:?}", res);
            }
            res.json()
        }
    }
}
