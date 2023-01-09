#[cfg(test)]
mod tests {
    use crate::contract;
    use boot_core::prelude::*;
    use cosmwasm_std::Addr;
    use cw_multi_test::ContractWrapper;
    use interfaces::contract::ToDoContract;
    // use semver::Version;

    use to_do_dapp_package::contract::{ExecuteMsg, QueryMsg, ToDoResponse};

    // const TEST_VERSION: Version = Version::new(0, 0, 0);

    fn setup() -> anyhow::Result<ToDoContract<Mock>> {
        let sender = Addr::unchecked("sender");

        let (_, mock_chain) = instantiate_default_mock_env(&sender)?;

        let mut to_do_contract = ToDoContract::new("testing_contract", mock_chain);

        to_do_contract
            .as_instance_mut()
            .set_mock(Box::new(ContractWrapper::new_with_empty(
                contract::execute,
                contract::instantiate,
                contract::query,
            )));
        to_do_contract.upload()?;
        to_do_contract.create_new()?;
        Ok(to_do_contract)
    }

    #[test]
    fn test_add_to_do() {
        let to_do_contract = setup().unwrap();

        to_do_contract
            .execute(
                &ExecuteMsg::AddToDo {
                    task: "test_add_to_do".to_owned(),
                    expiration: None,
                },
                None,
            )
            .unwrap();
        let res: ToDoResponse = to_do_contract
            .query(&QueryMsg::GetToDo { task_id: 1 })
            .unwrap();
        assert_eq!(
            res,
            ToDoResponse {
                task_id: 1,
                task: "test_add_to_do".to_owned(),
                expiration: cw_utils::Expiration::Never {}
            }
        )
    }
}
