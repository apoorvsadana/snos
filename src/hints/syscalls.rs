use std::collections::HashMap;

use cairo_vm::hint_processor::builtin_hint_processor::dict_manager::Dictionary;
use cairo_vm::hint_processor::builtin_hint_processor::hint_utils::{
    get_integer_from_var_name, get_ptr_from_var_name, insert_value_from_var_name,
};
use cairo_vm::hint_processor::hint_processor_definition::HintReference;
use cairo_vm::hint_processor::hint_processor_utils::felt_to_usize;
use cairo_vm::serde::deserialize_program::ApTracking;
use cairo_vm::types::exec_scope::ExecutionScopes;
use cairo_vm::types::relocatable::{MaybeRelocatable, Relocatable};
use cairo_vm::vm::errors::hint_errors::HintError;
use cairo_vm::vm::vm_core::VirtualMachine;
use cairo_vm::Felt252;
use indoc::indoc;

use crate::cairo_types::syscalls::{
    NewDeployResponse, NewSyscallContractResponse, StorageRead, StorageReadRequest, SyscallContractResponse,
};
use crate::execution::deprecated_syscall_handler::DeprecatedOsSyscallHandlerWrapper;
use crate::execution::helper::ExecutionHelperWrapper;
use crate::execution::syscall_handler::OsSyscallHandlerWrapper;
use crate::hints::vars;

pub const CALL_CONTRACT: &str = "syscall_handler.call_contract(segments=segments, syscall_ptr=ids.syscall_ptr)";

pub fn call_contract(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let syscall_handler = exec_scopes.get::<DeprecatedOsSyscallHandlerWrapper>("syscall_handler")?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;

    syscall_handler.call_contract(syscall_ptr, vm)?;

    Ok(())
}

pub const DELEGATE_CALL: &str = "syscall_handler.delegate_call(segments=segments, syscall_ptr=ids.syscall_ptr)";

pub fn delegate_call(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let syscall_handler = exec_scopes.get::<DeprecatedOsSyscallHandlerWrapper>("syscall_handler")?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;

    syscall_handler.storage_write(syscall_ptr);

    Ok(())
}

pub const DELEGATE_L1_HANDLER: &str =
    "syscall_handler.delegate_l1_handler(segments=segments, syscall_ptr=ids.syscall_ptr)";

pub fn delegate_l1_handler(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let syscall_handler = exec_scopes.get::<DeprecatedOsSyscallHandlerWrapper>("syscall_handler")?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;

    syscall_handler.delegate_l1_handler(syscall_ptr);

    Ok(())
}

pub const DEPLOY: &str = "syscall_handler.deploy(segments=segments, syscall_ptr=ids.syscall_ptr)";

pub fn deploy(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let syscall_handler = exec_scopes.get::<DeprecatedOsSyscallHandlerWrapper>("syscall_handler")?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;

    syscall_handler.deploy(syscall_ptr);

    Ok(())
}

pub const EMIT_EVENT: &str = "syscall_handler.emit_event(segments=segments, syscall_ptr=ids.syscall_ptr)";

pub fn emit_event(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let syscall_handler = exec_scopes.get::<DeprecatedOsSyscallHandlerWrapper>("syscall_handler")?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;

    syscall_handler.emit_event(syscall_ptr);

    Ok(())
}

pub const GET_BLOCK_NUMBER: &str = "syscall_handler.get_block_number(segments=segments, syscall_ptr=ids.syscall_ptr)";

pub fn get_block_number(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let syscall_handler = exec_scopes.get::<DeprecatedOsSyscallHandlerWrapper>("syscall_handler")?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;

    syscall_handler.get_block_number(syscall_ptr);

    Ok(())
}

pub const GET_BLOCK_TIMESTAMP: &str =
    "syscall_handler.get_block_timestamp(segments=segments, syscall_ptr=ids.syscall_ptr)";

pub fn get_block_timestamp(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let syscall_handler = exec_scopes.get::<DeprecatedOsSyscallHandlerWrapper>("syscall_handler")?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;

    syscall_handler.get_block_timestamp(syscall_ptr);

    Ok(())
}

pub const GET_CALLER_ADDRESS: &str =
    "syscall_handler.get_caller_address(segments=segments, syscall_ptr=ids.syscall_ptr)";

pub fn get_caller_address(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let syscall_handler = exec_scopes.get::<DeprecatedOsSyscallHandlerWrapper>("syscall_handler")?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;

    syscall_handler.get_caller_address(syscall_ptr, vm);

    Ok(())
}

pub const GET_CONTRACT_ADDRESS: &str =
    "syscall_handler.get_contract_address(segments=segments, syscall_ptr=ids.syscall_ptr)";

pub fn get_contract_address(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let syscall_handler = exec_scopes.get::<DeprecatedOsSyscallHandlerWrapper>("syscall_handler")?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;

    syscall_handler.get_contract_address(syscall_ptr);

    Ok(())
}

pub const GET_SEQUENCER_ADDRESS: &str =
    "syscall_handler.get_sequencer_address(segments=segments, syscall_ptr=ids.syscall_ptr)";

pub fn get_sequencer_address(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let syscall_handler = exec_scopes.get::<DeprecatedOsSyscallHandlerWrapper>("syscall_handler")?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;

    syscall_handler.get_sequencer_address(syscall_ptr);

    Ok(())
}

pub const GET_TX_INFO: &str = "syscall_handler.get_tx_info(segments=segments, syscall_ptr=ids.syscall_ptr)";

pub fn get_tx_info(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let syscall_handler = exec_scopes.get::<DeprecatedOsSyscallHandlerWrapper>("syscall_handler")?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;

    syscall_handler.get_tx_info(syscall_ptr);

    Ok(())
}

pub const GET_TX_SIGNATURE: &str = "syscall_handler.get_tx_signature(segments=segments, syscall_ptr=ids.syscall_ptr)";

pub fn get_tx_signature(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let syscall_handler = exec_scopes.get::<DeprecatedOsSyscallHandlerWrapper>("syscall_handler")?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;

    syscall_handler.get_tx_signature(syscall_ptr);

    Ok(())
}

pub const LIBRARY: &str = "syscall_handler.library_call(segments=segments, syscall_ptr=ids.syscall_ptr)";

pub fn library_call(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let syscall_handler = exec_scopes.get::<DeprecatedOsSyscallHandlerWrapper>("syscall_handler")?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;

    syscall_handler.library_call(syscall_ptr);

    Ok(())
}

pub const LIBRARY_CALL_L1_HANDLER: &str =
    "syscall_handler.library_call_l1_handler(segments=segments, syscall_ptr=ids.syscall_ptr)";

pub fn library_call_l1_handler(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let syscall_handler = exec_scopes.get::<DeprecatedOsSyscallHandlerWrapper>("syscall_handler")?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;

    syscall_handler.library_call_l1_handler(syscall_ptr);

    Ok(())
}

pub const REPLACE_CLASS: &str = "syscall_handler.replace_class(segments=segments, syscall_ptr=ids.syscall_ptr)";

pub fn replace_class(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let syscall_handler = exec_scopes.get::<DeprecatedOsSyscallHandlerWrapper>("syscall_handler")?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;

    syscall_handler.replace_class(syscall_ptr);

    Ok(())
}

pub const SEND_MESSAGE_TO_L1: &str =
    "syscall_handler.send_message_to_l1(segments=segments, syscall_ptr=ids.syscall_ptr)";

pub fn send_message_to_l1(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let syscall_handler = exec_scopes.get::<DeprecatedOsSyscallHandlerWrapper>("syscall_handler")?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;

    syscall_handler.send_message_to_l1(syscall_ptr);

    Ok(())
}

pub const STORAGE_READ: &str = "syscall_handler.storage_read(segments=segments, syscall_ptr=ids.syscall_ptr)";

pub fn storage_read(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let syscall_handler = exec_scopes.get::<DeprecatedOsSyscallHandlerWrapper>("syscall_handler")?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;

    syscall_handler.storage_read(syscall_ptr, vm)?;

    Ok(())
}

pub const STORAGE_WRITE: &str = "syscall_handler.storage_write(segments=segments, syscall_ptr=ids.syscall_ptr)";

pub fn storage_write(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let syscall_handler = exec_scopes.get::<DeprecatedOsSyscallHandlerWrapper>("syscall_handler")?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;

    syscall_handler.storage_write(syscall_ptr);

    Ok(())
}

pub const SET_SYSCALL_PTR: &str = indoc! {r#"
	ids.os_context = segments.add()
	ids.syscall_ptr = segments.add()

	syscall_handler.set_syscall_ptr(syscall_ptr=ids.syscall_ptr)"#
};

pub fn set_syscall_ptr(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let os_context = vm.add_memory_segment();
    let syscall_ptr = vm.add_memory_segment();

    insert_value_from_var_name(vars::ids::OS_CONTEXT, os_context, vm, ids_data, ap_tracking)?;
    insert_value_from_var_name(vars::ids::SYSCALL_PTR, syscall_ptr, vm, ids_data, ap_tracking)?;

    let syscall_handler: OsSyscallHandlerWrapper = exec_scopes.get(vars::scopes::SYSCALL_HANDLER)?;
    syscall_handler.set_syscall_ptr(syscall_ptr);

    Ok(())
}

pub const OS_LOGGER_ENTER_SYSCALL_PREPRARE_EXIT_SYSCALL: &str = indoc! {r#"
        execution_helper.os_logger.enter_syscall(
            n_steps=current_step,
            builtin_ptrs=ids.builtin_ptrs,
            deprecated=True,
            selector=ids.selector,
            range_check_ptr=ids.range_check_ptr,
        )

        # Prepare a short callable to save code duplication.
        exit_syscall = lambda selector: execution_helper.os_logger.exit_syscall(
            n_steps=current_step,
            builtin_ptrs=ids.builtin_ptrs,
            range_check_ptr=ids.range_check_ptr,
            selector=selector,
        )"#
};
pub fn os_logger_enter_syscall_preprare_exit_syscall(
    _vm: &mut VirtualMachine,
    _exec_scopes: &mut ExecutionScopes,
    _ids_data: &HashMap<String, HintReference>,
    _ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    println!("TODO: os_logger enter/exit calls");

    Ok(())
}

pub const FETCH_STATE_ENTRY_5: &str = indoc! {r#"
	# Fetch a state_entry in this hint and validate it in the update that comes next.
	ids.state_entry = __dict_manager.get_dict(ids.contract_state_changes)[
	    ids.contract_address
	]

	ids.new_state_entry = segments.add()"#
};
pub fn fetch_state_entry_5(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let key = get_integer_from_var_name("contract_address", vm, ids_data, ap_tracking)?;
    let dict_ptr = get_ptr_from_var_name("contract_state_changes", vm, ids_data, ap_tracking)?;
    let val = match exec_scopes.get_dict_manager()?.borrow().get_tracker(dict_ptr)?.data.clone() {
        Dictionary::SimpleDictionary(dict) => dict
            .get(&MaybeRelocatable::Int(key.into_owned()))
            .expect("State changes dictionnary shouldn't be None")
            .clone(),
        Dictionary::DefaultDictionary { dict: _d, default_value: _v } => {
            panic!("State changes dict shouldn't be a default dict")
        }
    };
    insert_value_from_var_name("state_entry", val, vm, ids_data, ap_tracking)?;
    insert_value_from_var_name("new_state_entry", vm.add_memory_segment(), vm, ids_data, ap_tracking)?;

    Ok(())
}

pub const CACHE_CONTRACT_STORAGE_2: &str = indoc! {r#"
	# Make sure the value is cached (by reading it), to be used later on for the
	# commitment computation.
	value = execution_helper.storage_by_address[ids.contract_address].read(
	    key=ids.syscall_ptr.request.address
	)
	assert ids.value == value, "Inconsistent storage value.""#
};
pub fn cache_contract_storage_2(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let contract_address = get_integer_from_var_name("contract_address", vm, ids_data, ap_tracking)?;
    let syscall_ptr = get_ptr_from_var_name("syscall_ptr", vm, ids_data, ap_tracking)?;
    let key =
        vm.get_integer((syscall_ptr + (StorageRead::request_offset() + StorageReadRequest::address_offset()))?)?;

    let mut execution_helper: ExecutionHelperWrapper = exec_scopes.get(vars::scopes::EXECUTION_HELPER)?;
    let value = execution_helper.read_storage_for_address(*contract_address, *key).map_err(|_| {
        HintError::CustomHint(
            format!("Storage read error, contract: {}, key: {}", contract_address, key).into_boxed_str(),
        )
    })?;

    let ids_value = *get_integer_from_var_name("value", vm, ids_data, ap_tracking)?;
    if ids_value != value {
        return Err(HintError::AssertionFailed(
            format!("Inconsistent storage value: {} <> {}.", ids_value, value).into(),
        ));
    }

    Ok(())
}

fn assert_memory_ranges_equal(
    vm: &VirtualMachine,
    expected_ptr: Relocatable,
    expected_size: usize,
    actual_ptr: Relocatable,
    actual_size: usize,
) -> Result<(), HintError> {
    let expected = vm.get_range(expected_ptr, expected_size);
    let actual = vm.get_range(actual_ptr, actual_size);

    if expected != actual {
        return Err(HintError::AssertionFailed(
            format!("Return value mismatch expected={expected:?}, actual={actual:?}.").into_boxed_str(),
        ));
    }

    Ok(())
}

pub const CHECK_SYSCALL_RESPONSE: &str = indoc! {r#"
	# Check that the actual return value matches the expected one.
	expected = memory.get_range(
	    addr=ids.call_response.retdata, size=ids.call_response.retdata_size
	)
	actual = memory.get_range(addr=ids.retdata, size=ids.retdata_size)

	assert expected == actual, f'Return value mismatch expected={expected}, actual={actual}.'"#
};

pub fn check_syscall_response(
    vm: &mut VirtualMachine,
    _exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let call_response_ptr = get_ptr_from_var_name(vars::ids::CALL_RESPONSE, vm, ids_data, ap_tracking)?;
    let call_response_retdata = vm.get_relocatable((call_response_ptr + SyscallContractResponse::retdata_offset())?)?;
    let call_response_retdata_size =
        felt_to_usize(vm.get_integer((call_response_ptr + SyscallContractResponse::retdata_size_offset())?)?.as_ref())?;

    let retdata = get_ptr_from_var_name(vars::ids::RETDATA, vm, ids_data, ap_tracking)?;
    let retdata_size =
        felt_to_usize(get_integer_from_var_name(vars::ids::RETDATA_SIZE, vm, ids_data, ap_tracking)?.as_ref())?;

    assert_memory_ranges_equal(vm, call_response_retdata, call_response_retdata_size, retdata, retdata_size)?;

    Ok(())
}

pub const CHECK_NEW_SYSCALL_RESPONSE: &str = indoc! {r#"
	# Check that the actual return value matches the expected one.
	expected = memory.get_range(
	    addr=ids.response.retdata_start,
	    size=ids.response.retdata_end - ids.response.retdata_start,
	)
	actual = memory.get_range(addr=ids.retdata, size=ids.retdata_size)

	assert expected == actual, f'Return value mismatch; expected={expected}, actual={actual}.'"#
};

pub fn check_new_syscall_response(
    vm: &mut VirtualMachine,
    _exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let response_ptr = get_ptr_from_var_name(vars::ids::RESPONSE, vm, ids_data, ap_tracking)?;
    let response_retdata_start =
        vm.get_relocatable((response_ptr + NewSyscallContractResponse::retdata_start_offset())?)?;
    let response_retdata_end =
        vm.get_relocatable((response_ptr + NewSyscallContractResponse::retdata_end_offset())?)?;
    let response_retdata_size = (response_retdata_end - response_retdata_start)?;

    let retdata = get_ptr_from_var_name(vars::ids::RETDATA, vm, ids_data, ap_tracking)?;
    let retdata_size =
        felt_to_usize(get_integer_from_var_name(vars::ids::RETDATA_SIZE, vm, ids_data, ap_tracking)?.as_ref())?;

    assert_memory_ranges_equal(vm, response_retdata_start, response_retdata_size, retdata, retdata_size)?;

    Ok(())
}

pub const CHECK_NEW_DEPLOY_RESPONSE: &str = indoc! {r#"
	# Check that the actual return value matches the expected one.
	expected = memory.get_range(
	    addr=ids.response.constructor_retdata_start,
	    size=ids.response.constructor_retdata_end - ids.response.constructor_retdata_start,
	)
	actual = memory.get_range(addr=ids.retdata, size=ids.retdata_size)
	assert expected == actual, f'Return value mismatch; expected={expected}, actual={actual}.'"#
};

pub fn check_new_deploy_response(
    vm: &mut VirtualMachine,
    _exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let response_ptr = get_ptr_from_var_name(vars::ids::RESPONSE, vm, ids_data, ap_tracking)?;
    let constructor_retdata_start =
        vm.get_relocatable((response_ptr + NewDeployResponse::constructor_retdata_start_offset())?)?;
    let constructor_retdata_end =
        vm.get_relocatable((response_ptr + NewDeployResponse::constructor_retdata_end_offset())?)?;
    let response_retdata_size = (constructor_retdata_end - constructor_retdata_start)?;

    let retdata = get_ptr_from_var_name(vars::ids::RETDATA, vm, ids_data, ap_tracking)?;
    let retdata_size =
        felt_to_usize(get_integer_from_var_name(vars::ids::RETDATA_SIZE, vm, ids_data, ap_tracking)?.as_ref())?;

    assert_memory_ranges_equal(vm, constructor_retdata_start, response_retdata_size, retdata, retdata_size)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use blockifier::block_context::BlockContext;
    use blockifier::state::cached_state::CachedState;
    use cairo_vm::types::relocatable::Relocatable;
    use rstest::{fixture, rstest};

    use super::*;
    use crate::hints::tests::tests::block_context;
    use crate::ExecutionHelperWrapper;

    #[fixture]
    fn exec_scopes(block_context: BlockContext) -> ExecutionScopes {
        let execution_infos = vec![];
        let exec_helper = ExecutionHelperWrapper::new(CachedState::default(), execution_infos, &block_context);
        let syscall_handler = OsSyscallHandlerWrapper::new(exec_helper);

        let mut exec_scopes = ExecutionScopes::new();
        exec_scopes.insert_value(vars::scopes::SYSCALL_HANDLER, syscall_handler);

        exec_scopes
    }

    #[rstest]
    fn test_set_syscall_ptr(mut exec_scopes: ExecutionScopes) {
        let mut vm = VirtualMachine::new(false);

        let ids_data = HashMap::from([
            (vars::ids::OS_CONTEXT.to_string(), HintReference::new_simple(-2)),
            (vars::ids::SYSCALL_PTR.to_string(), HintReference::new_simple(-1)),
        ]);
        vm.add_memory_segment();
        vm.add_memory_segment();
        vm.set_fp(2);

        let ap_tracking = ApTracking::new();
        let constants = HashMap::new();

        set_syscall_ptr(&mut vm, &mut exec_scopes, &ids_data, &ap_tracking, &constants).unwrap();

        let os_context = get_ptr_from_var_name(vars::ids::OS_CONTEXT, &vm, &ids_data, &ap_tracking).unwrap();
        let syscall_ptr = get_ptr_from_var_name(vars::ids::SYSCALL_PTR, &vm, &ids_data, &ap_tracking).unwrap();

        assert_eq!(os_context, Relocatable::from((2, 0)));
        assert_eq!(syscall_ptr, Relocatable::from((3, 0)));

        let syscall_handler: OsSyscallHandlerWrapper = exec_scopes.get(vars::scopes::SYSCALL_HANDLER).unwrap();
        assert_eq!(syscall_handler.syscall_ptr(), Some(syscall_ptr));
    }
}

pub fn exit_syscall(
    selector_name: &str,
    _vm: &mut VirtualMachine,
    _exec_scopes: &mut ExecutionScopes,
    _ids_data: &HashMap<String, HintReference>,
    _ap_tracking: &ApTracking,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    // TODO: add logging
    println!("exiting syscall {}", selector_name);
    Ok(())
}
pub const EXIT_CALL_CONTRACT_SYSCALL: &str = "exit_syscall(selector=ids.CALL_CONTRACT_SELECTOR)";
pub fn exit_call_contract_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("CALL_CONTRACT_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}

pub const EXIT_DELEGATE_CALL_SYSCALL: &str = "exit_syscall(selector=ids.DELEGATE_CALL_SELECTOR)";
pub fn exit_delegate_call_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("DELEGATE_CALL_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_DELEGATE_L1_HANDLER_SYSCALL: &str = "exit_syscall(selector=ids.DELEGATE_L1_HANDLER_SELECTOR)";
pub fn exit_delegate_l1_handler_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("DELEGATE_L1_HANDLER_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_DEPLOY_SYSCALL: &str = "exit_syscall(selector=ids.DEPLOY_SELECTOR)";
pub fn exit_deploy_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("DEPLOY_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_EMIT_EVENT_SYSCALL: &str = "exit_syscall(selector=ids.EMIT_EVENT_SELECTOR)";

pub fn exit_emit_event_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("EMIT_EVENT_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_GET_BLOCK_HASH_SYSCALL: &str = "exit_syscall(selector=ids.GET_BLOCK_HASH_SELECTOR)";

pub fn exit_get_block_hash_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("GET_BLOCK_HASH_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_GET_BLOCK_TIMESTAMP_SYSCALL: &str = "exit_syscall(selector=ids.GET_BLOCK_TIMESTAMP_SELECTOR)";

pub fn exit_get_block_timestamp_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("GET_BLOCK_TIMESTAMP_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_GET_CALLER_ADDRESS_SYSCALL: &str = "exit_syscall(selector=ids.GET_CALLER_ADDRESS_SELECTOR)";

pub fn exit_get_caller_address_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("GET_CALLER_ADDRESS_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_GET_CONTRACT_ADDRESS_SYSCALL: &str = "exit_syscall(selector=ids.GET_CONTRACT_ADDRESS_SELECTOR)";

pub fn exit_get_contract_address_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("GET_CONTRACT_ADDRESS_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_GET_EXECUTION_INFO_SYSCALL: &str = "exit_syscall(selector=ids.GET_EXECUTION_INFO_SELECTOR)";

pub fn exit_get_execution_info_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("GET_EXECUTION_INFO_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_GET_SEQUENCER_ADDRESS_SYSCALL: &str = "exit_syscall(selector=ids.GET_SEQUENCER_ADDRESS_SELECTOR)";

pub fn exit_get_sequencer_address_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("GET_SEQUENCER_ADDRESS_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_GET_TX_INFO_SYSCALL: &str = "exit_syscall(selector=ids.GET_TX_INFO_SELECTOR)";

pub fn exit_get_tx_info_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("GET_TX_INFO_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_GET_TX_SIGNATURE_SYSCALL: &str = "exit_syscall(selector=ids.GET_TX_SIGNATURE_SELECTOR)";

pub fn exit_get_tx_signature_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("GET_TX_SIGNATURE_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_KECCAK_SYSCALL: &str = "exit_syscall(selector=ids.KECCAK_SELECTOR)";

pub fn exit_keccak_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("KECCAK_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_LIBRARY_CALL_L1_HANDLER_SYSCALL: &str = "exit_syscall(selector=ids.LIBRARY_CALL_L1_HANDLER_SELECTOR)";

pub fn exit_library_call_l1_handler_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("LIBRARY_CALL_L1_HANDLER_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_LIBRARY_CALL_SYSCALL: &str = "exit_syscall(selector=ids.LIBRARY_CALL_SELECTOR)";

pub fn exit_library_call_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("LIBRARY_CALL_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_REPLACE_CLASS_SYSCALL: &str = "exit_syscall(selector=ids.REPLACE_CLASS_SELECTOR)";

pub fn exit_replace_class_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("REPLACE_CLASS_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_SECP256K1_ADD_SYSCALL: &str = "exit_syscall(selector=ids.SECP256K1_ADD_SELECTOR)";

pub fn exit_secp256k1_add_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("SECP256K1_ADD_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_SECP256K1_GET_POINT_FROM_X_SYSCALL: &str =
    "exit_syscall(selector=ids.SECP256K1_GET_POINT_FROM_X_SELECTOR)";

pub fn exit_secp256k1_get_point_from_x_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("SECP256K1_GET_POINT_FROM_X_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_SECP256K1_GET_XY_SYSCALL: &str = "exit_syscall(selector=ids.SECP256K1_GET_XY_SELECTOR)";

pub fn exit_secp256k1_get_xy_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("SECP256K1_GET_XY_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_SECP256K1_MUL_SYSCALL: &str = "exit_syscall(selector=ids.SECP256K1_MUL_SELECTOR)";

pub fn exit_secp256k1_mul_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("SECP256K1_MUL_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_SECP256K1_NEW_SYSCALL: &str = "exit_syscall(selector=ids.SECP256K1_NEW_SELECTOR)";

pub fn exit_secp256k1_new_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("SECP256K1_NEW_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_SECP256R1_ADD_SYSCALL: &str = "exit_syscall(selector=ids.SECP256R1_ADD_SELECTOR)";

pub fn exit_secp256r1_add_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("SECP256R1_ADD_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_SECP256R1_GET_POINT_FROM_X_SYSCALL: &str =
    "exit_syscall(selector=ids.SECP256R1_GET_POINT_FROM_X_SELECTOR)";

pub fn exit_secp256r1_get_point_from_x_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("SECP256R1_GET_POINT_FROM_X_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_SECP256R1_GET_XY_SYSCALL: &str = "exit_syscall(selector=ids.SECP256R1_GET_XY_SELECTOR)";

pub fn exit_secp256r1_get_xy_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("SECP256R1_GET_XY_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_SECP256R1_MUL_SYSCALL: &str = "exit_syscall(selector=ids.SECP256R1_MUL_SELECTOR)";

pub fn exit_secp256r1_mul_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("SECP256R1_MUL_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_SECP256R1_NEW_SYSCALL: &str = "exit_syscall(selector=ids.SECP256R1_NEW_SELECTOR)";

pub fn exit_secp256r1_new_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("SECP256R1_NEW_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_SEND_MESSAGE_TO_L1_SYSCALL: &str = "exit_syscall(selector=ids.SEND_MESSAGE_TO_L1_SELECTOR)";

pub fn exit_send_message_to_l1_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("SEND_MESSAGE_TO_L1_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_STORAGE_READ_SYSCALL: &str = "exit_syscall(selector=ids.STORAGE_READ_SELECTOR)";

pub fn exit_storage_read_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("STORAGE_READ_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
pub const EXIT_STORAGE_WRITE_SYSCALL: &str = "exit_syscall(selector=ids.STORAGE_WRITE_SELECTOR)";

pub fn exit_storage_write_syscall(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    exit_syscall("STORAGE_WRITE_SELECTOR", vm, exec_scopes, ids_data, ap_tracking, constants)
}
