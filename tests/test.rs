#[cfg(test)]
mod tests {
	use evm::backend::{ApplyBackend, MemoryAccount, MemoryBackend, MemoryVicinity};
	use evm::executor::stack::{MemoryStackState, StackExecutor, StackSubstateMetadata};
	use evm::Config;
	use primitive_types::{H160, H256, U256};
	use std::{collections::BTreeMap, str::FromStr};

	#[test]
	fn test_transfer_eth() {
		let config = Config::istanbul();

		let vicinity = MemoryVicinity {
			gas_price: U256::zero(),
			origin: H160::default(),
			block_hashes: Vec::new(),
			block_number: Default::default(),
			block_coinbase: Default::default(),
			block_timestamp: Default::default(),
			block_difficulty: Default::default(),
			block_gas_limit: Default::default(),
			chain_id: U256::one(),
			block_base_fee_per_gas: U256::zero(),
		};

		let mut state = BTreeMap::new();

		state.insert(
			H160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
			MemoryAccount {
				nonce: U256::one(),
				balance: U256::from(10000000),
				storage: BTreeMap::new(),
				code: Vec::new(),
			},
		);
		state.insert(
			H160::from_str("0x0000000000000000000000000000000000000002").unwrap(),
			MemoryAccount {
				nonce: U256::one(),
				balance: U256::from(10000000),
				storage: BTreeMap::new(),
				code: Vec::new(),
			},
		);

		let mut backend = MemoryBackend::new(&vicinity, state);
		let metadata = StackSubstateMetadata::new(u64::MAX, &config);
		let state = MemoryStackState::new(metadata, &backend);
		let precompiles = BTreeMap::new();
		let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);

		// Transfer 100 wie from 0x0000000000000000000000000000000000000001 to 0x0000000000000000000000000000000000000002
		let _reason = executor.transact_call(
			H160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
			H160::from_str("0x0000000000000000000000000000000000000002").unwrap(),
			U256::from_dec_str("100").unwrap(),
			hex::decode("00").unwrap(),
			u64::MAX,
			Vec::new(),
		);

		let (values, logs) = executor.into_state().deconstruct();

		backend.apply(values, logs, true);

		assert_eq!(
			backend
				.state()
				.get(&H160::from_str("0x0000000000000000000000000000000000000001").unwrap())
				.unwrap(),
			&MemoryAccount {
				nonce: U256::from(2),
				balance: U256::from(9999900),
				storage: BTreeMap::new(),
				code: Vec::new(),
			},
		);

		assert_eq!(
			backend
				.state()
				.get(&H160::from_str("0x0000000000000000000000000000000000000002").unwrap())
				.unwrap(),
			&MemoryAccount {
				nonce: U256::one(),
				balance: U256::from(10000100),
				storage: BTreeMap::new(),
				code: Vec::new(),
			},
		);
	}

	#[test]
	//TestContract.sol
	// 0x535b3d7a252fa034ed71f0c53ec0c6f784cb64e1: Contract was created by 0x0000000000000000000000000000000000000001 with nonce 1
	fn test_create_contract() {
		let config = Config::istanbul();

		let vicinity = MemoryVicinity {
			gas_price: U256::zero(),
			origin: H160::default(),
			block_hashes: Vec::new(),
			block_number: Default::default(),
			block_coinbase: Default::default(),
			block_timestamp: Default::default(),
			block_difficulty: Default::default(),
			block_gas_limit: Default::default(),
			chain_id: U256::one(),
			block_base_fee_per_gas: U256::zero(),
		};

		let mut state = BTreeMap::new();

		state.insert(
			H160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
			MemoryAccount {
				nonce: U256::one(),
				balance: U256::from(10000000),
				storage: BTreeMap::new(),
				code: Vec::new(),
			},
		);

		let mut backend = MemoryBackend::new(&vicinity, state);
		let metadata = StackSubstateMetadata::new(u64::MAX, &config);
		let state = MemoryStackState::new(metadata, &backend);
		let precompiles = BTreeMap::new();
		let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);

		let _reason = executor.transact_create(
			H160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
			U256::from(100),
			hex::decode("60806040526000805561014d806100176000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c806361bc221a1461003b578063d09de08a14610059575b600080fd5b610043610063565b6040516100509190610099565b60405180910390f35b610061610069565b005b60005481565b600160005461007891906100e3565b600081905550565b6000819050919050565b61009381610080565b82525050565b60006020820190506100ae600083018461008a565b92915050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60006100ee82610080565b91506100f983610080565b9250828201905080821115610111576101106100b4565b5b9291505056fea2646970667358221220497c014d5a3c457cfbf88e3a19c1e3646bc65fc0381bf3f8ab53e978857b44fc64736f6c63430008110033").unwrap(),
			100000000,
			Vec::new(),
		);

		let (values, logs) = executor.into_state().deconstruct();

		backend.apply(values, logs, true);

		assert_eq!(
			backend
				.state()
				.get(&H160::from_str("0x535b3d7a252fa034ed71f0c53ec0c6f784cb64e1").unwrap())
				.unwrap()
				.nonce,
			U256::from(1)
		);

		assert_eq!(
			backend
				.state()
				.get(&H160::from_str("0x535b3d7a252fa034ed71f0c53ec0c6f784cb64e1").unwrap())
				.unwrap()
				.balance,
			U256::from(100)
		);
	}

	#[test]
	//TestContract.sol
	// 0x535b3d7a252fa034ed71f0c53ec0c6f784cb64e1: Contract was created by 0x0000000000000000000000000000000000000001 with nonce 1
	fn test_call_contract() {
		let config = Config::istanbul();

		let vicinity = MemoryVicinity {
			gas_price: U256::zero(),
			origin: H160::default(),
			block_hashes: Vec::new(),
			block_number: Default::default(),
			block_coinbase: Default::default(),
			block_timestamp: Default::default(),
			block_difficulty: Default::default(),
			block_gas_limit: Default::default(),
			chain_id: U256::one(),
			block_base_fee_per_gas: U256::zero(),
		};

		let mut state = BTreeMap::new();

		state.insert(
			H160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
			MemoryAccount {
				nonce: U256::one(),
				balance: U256::from(10000000),
				storage: BTreeMap::new(),
				code: Vec::new(),
			},
		);

		let mut backend = MemoryBackend::new(&vicinity, state);
		let metadata = StackSubstateMetadata::new(u64::MAX, &config);
		let state = MemoryStackState::new(metadata, &backend);
		let precompiles = BTreeMap::new();
		let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);

		let _reason = executor.transact_create(
			H160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
			U256::from(100),
			hex::decode("60806040526000805561014d806100176000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c806361bc221a1461003b578063d09de08a14610059575b600080fd5b610043610063565b6040516100509190610099565b60405180910390f35b610061610069565b005b60005481565b600160005461007891906100e3565b600081905550565b6000819050919050565b61009381610080565b82525050565b60006020820190506100ae600083018461008a565b92915050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60006100ee82610080565b91506100f983610080565b9250828201905080821115610111576101106100b4565b5b9291505056fea2646970667358221220497c014d5a3c457cfbf88e3a19c1e3646bc65fc0381bf3f8ab53e978857b44fc64736f6c63430008110033").unwrap(),
			100000000,
			Vec::new(),
		);

		let (values, logs) = executor.into_state().deconstruct();

		backend.apply(values, logs, true);

		let metadata = StackSubstateMetadata::new(u64::MAX, &config);
		let state = MemoryStackState::new(metadata, &backend);
		let precompiles = BTreeMap::new();
		let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);

		// call increment() function
		let _reason = executor.transact_call(
			H160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
			H160::from_str("0x535b3d7a252fa034ed71f0c53ec0c6f784cb64e1").unwrap(),
			U256::zero(),
			hex::decode("d09de08a").unwrap(),
			u64::MAX,
			Vec::new(),
		);

		let (values, logs) = executor.into_state().deconstruct();

		backend.apply(values, logs, true);

		// expect counter == 1
		assert_eq!(
			backend
				.state()
				.get(&H160::from_str("0x535b3d7a252fa034ed71f0c53ec0c6f784cb64e1").unwrap())
				.unwrap()
				.storage
				.get(
					&H256::from_str(
						"0x0000000000000000000000000000000000000000000000000000000000000000"
					)
					.unwrap()
				)
				.unwrap(),
			&H256::from_str("0x0000000000000000000000000000000000000000000000000000000000000001")
				.unwrap()
		);
	}
}
