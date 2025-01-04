pub mod backup {
    include!("resources/backup/container_storage_account.rs");
    include!("resources/backup/policy_file_share.rs");
    include!("resources/backup/policy_vm.rs");
    include!("resources/backup/policy_vm_workload.rs");
    include!("resources/backup/protected_file_share.rs");
    include!("resources/backup/protected_vm.rs");
}
pub mod functions {
    pub mod backup {
        include!("functions/backup/get_policy_fileshare.rs");
        include!("functions/backup/get_policy_vm.rs");
    }
}
pub mod types {
    pub mod backup {
        include!("types/backup/policy_file_share_backup.rs");
        include!("types/backup/policy_file_share_backup_hourly.rs");
        include!("types/backup/policy_file_share_retention_daily.rs");
        include!("types/backup/policy_file_share_retention_monthly.rs");
        include!("types/backup/policy_file_share_retention_weekly.rs");
        include!("types/backup/policy_file_share_retention_yearly.rs");
        include!("types/backup/policy_vm_backup.rs");
        include!("types/backup/policy_vm_instant_restore_resource_group.rs");
        include!("types/backup/policy_vm_retention_daily.rs");
        include!("types/backup/policy_vm_retention_monthly.rs");
        include!("types/backup/policy_vm_retention_weekly.rs");
        include!("types/backup/policy_vm_retention_yearly.rs");
        include!("types/backup/policy_vm_tiering_policy.rs");
        include!("types/backup/policy_vm_tiering_policy_archived_restore_point.rs");
        include!("types/backup/policy_vm_workload_protection_policy.rs");
        include!("types/backup/policy_vm_workload_protection_policy_backup.rs");
        include!("types/backup/policy_vm_workload_protection_policy_retention_daily.rs");
        include!(
            "types/backup/policy_vm_workload_protection_policy_retention_monthly.rs"
        );
        include!(
            "types/backup/policy_vm_workload_protection_policy_retention_weekly.rs"
        );
        include!(
            "types/backup/policy_vm_workload_protection_policy_retention_yearly.rs"
        );
        include!(
            "types/backup/policy_vm_workload_protection_policy_simple_retention.rs"
        );
        include!("types/backup/policy_vm_workload_settings.rs");
    }
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-azure {
    import output-interface;
}

interface output-interface {

    resource output {
        constructor(value: string);
        map: func(function-name: string) -> output;
    }
    combine: func(outputs: list<borrow<output>>) -> output;
}


interface register-interface {
    use output-interface.{output};

    record object-field {
        name: string,
        value: borrow<output>
    }

    record result-field {
        name: string
    }

    record register-resource-result-field {
        name: string,
        output: output
    }

    record register-resource-request {
        %type: string,
        name: string,
        object: list<object-field>,
        results: list<result-field>
    }

    record register-resource-result {
        fields: list<register-resource-result-field>
    }

    register: func(request: register-resource-request) -> register-resource-result;

    record resource-invoke-result-field {
        name: string,
        output: output
    }

    record resource-invoke-request {
        token: string,
        object: list<object-field>,
        results: list<result-field>
    }

    record resource-invoke-result {
        fields: list<resource-invoke-result-field>
    }

    invoke: func(request: resource-invoke-request) -> resource-invoke-result;
}",
        with : { "component:pulumi-wasm/output-interface@0.0.0-DEV" :
        pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface
        } }
    );
}
