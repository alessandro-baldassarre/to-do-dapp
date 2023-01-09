use boot_core::{
    prelude::{boot_contract, BootInstantiate},
    BootEnvironment, BootError, Contract, TxResponse,
};
use cosmwasm_std::Empty;
use to_do_dapp_package::contract::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[boot_contract(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct ToDoContract;

impl<Chain: BootEnvironment> ToDoContract<Chain> {
    pub fn new(id: &str, chain: Chain) -> Self {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let file_path = &format!("{}{}", crate_path, "/artifacts/to_do_dapp-aarch64.wasm");
        Self(Contract::new(id, chain).with_wasm_path(file_path))
    }

    pub fn create_new(&self) -> Result<TxResponse<Chain>, BootError> {
        let msg = InstantiateMsg { admin: None };

        self.instantiate(&msg, None, None)
    }
}
