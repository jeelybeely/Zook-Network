use stacks_core::clarity_vm::clarity::ClarityInstance;
use stacks_core::clarity_vm::database::MemoryBackingStore;
use stacks_core::clarity_vm::execution::Executer;
use stacks_core::clarity_vm::types::{QualifiedContractIdentifier, Value};
use stacks_core::clarity_vm::representations::{ClarityBlockConnection, ClarityBlockState};
use stacks_core::clarity_vm::errors::RuntimeError;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::fs;
use std::io::Read;

pub struct ClarityInteractor {
    clarity_instance: ClarityInstance,
    contracts: HashMap<String, QualifiedContractIdentifier>,
}

impl ClarityInteractor {
    pub fn new() -> Self {
        let backing_store = MemoryBackingStore::new();
        let clarity_instance = ClarityInstance::new(Arc::new(backing_store));
        ClarityInteractor {
            clarity_instance,
            contracts: HashMap::new(),
        }
    }

    pub fn load_contracts(&mut self, contracts_dir: PathBuf) -> Result<(), String> {
        if !contracts_dir.is_dir() {
            return Err("Contracts directory does not exist".to_string());
        }

        for entry in fs::read_dir(contracts_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("clar") {
                let mut file = fs::File::open(&path).map_err(|e| e.to_string())?;
                let mut content = String::new();
                file.read_to_string(&mut content).map_err(|e| e.to_string())?;

                let contract_name = path.file_stem().and_then(|os_str| os_str.to_str()).unwrap_or_default().to_string();
                let contract_id = QualifiedContractIdentifier::new("zook-l2", &contract_name).map_err(|e| e.to_string())?;

                self.clarity_instance
                    .initialize_contract(&contract_id, &content, None)
                    .map_err(|e| format!("Failed to initialize contract {}: {:?}", contract_name, e))?;

                self.contracts.insert(contract_name.clone(), contract_id);
            }
        }
        Ok(())
    }

    pub fn execute_function(
        &self,
        contract_name: &str,
        function_name: &str,
        args: Vec<Value>,
        block_height: u64,
    ) -> Result<Value, RuntimeError> {
        let contract_id = self
            .contracts
            .get(contract_name)
            .ok_or_else(|| RuntimeError::UndefinedContract(contract_name.to_string()))?;

        let block_info = ClarityBlockConnection::new(
            ClarityBlockState {
                block_height,
                block_time: 0,
                proposer_address: None,
            },
            None,
        );

        self.clarity_instance
            .execute_function(contract_id, function_name, &args, &block_info)
    }

    pub fn get_variable(
        &self,
        contract_name: &str,
        variable_name: &str,
        block_height: u64,
    ) -> Result<Value, RuntimeError> {
        let contract_id = self
            .contracts
            .get(contract_name)
            .ok_or_else(|| RuntimeError::UndefinedContract(contract_name.to_string()))?;

        let block_info = ClarityBlockConnection::new(
            ClarityBlockState {
                block_height,
                block_time: 0,
                proposer_address: None,
            },
            None,
        );

        self.clarity_instance
            .lookup_variable(contract_id, variable_name, &block_info)
    }
}
