use icgeek_candid_gen::*;

#[allow(deprecated)]
fn main() {
    generate_init_candid_method!(common_contract_api, init_contract);

    generate_query_candid_method!(common_canister_api, get_canister_metrics);

    generate_update_candid_method!(common_contract_api, get_contract_owner);
    generate_update_candid_method!(common_canister_api, get_canister_status);
    generate_update_candid_method!(common_contract_api, activate_contract);
    generate_update_candid_method!(common_contract_api, get_contract_certificate);
    generate_update_candid_method!(common_contract_api, add_contract_controller);

    candid::export_service!();
    std::print!("{}", __export_service());
}
