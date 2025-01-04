pub mod siterecovery {
    include!("resources/siterecovery/fabric.rs");
    include!("resources/siterecovery/hyper_v_replication_policy.rs");
    include!("resources/siterecovery/hyper_v_replication_policy_association.rs");
    include!("resources/siterecovery/hyper_v_site.rs");
    include!("resources/siterecovery/hyperv_network_mapping.rs");
    include!("resources/siterecovery/network_mapping.rs");
    include!("resources/siterecovery/protection_container.rs");
    include!("resources/siterecovery/protection_container_mapping.rs");
    include!("resources/siterecovery/replicated_vm.rs");
    include!("resources/siterecovery/replication_policy.rs");
    include!("resources/siterecovery/replication_recovery_plan.rs");
    include!("resources/siterecovery/vm_ware_replication_policy.rs");
    include!("resources/siterecovery/vmware_replicated_vm.rs");
    include!("resources/siterecovery/vmware_replication_policy_association.rs");
}
pub mod functions {
    pub mod siterecovery {
        include!("functions/siterecovery/get_fabric.rs");
        include!("functions/siterecovery/get_protection_container.rs");
        include!("functions/siterecovery/get_replication_policy.rs");
        include!("functions/siterecovery/get_replication_recovery_plan.rs");
    }
}
pub mod types {
    pub mod siterecovery {
        include!("types/siterecovery/protection_container_mapping_automatic_update.rs");
        include!("types/siterecovery/replicated_vm_managed_disk.rs");
        include!(
            "types/siterecovery/replicated_vm_managed_disk_target_disk_encryption.rs"
        );
        include!(
            "types/siterecovery/replicated_vm_managed_disk_target_disk_encryption_disk_encryption_key.rs"
        );
        include!(
            "types/siterecovery/replicated_vm_managed_disk_target_disk_encryption_key_encryption_key.rs"
        );
        include!("types/siterecovery/replicated_vm_network_interface.rs");
        include!("types/siterecovery/replicated_vm_unmanaged_disk.rs");
        include!(
            "types/siterecovery/replication_recovery_plan_azure_to_azure_settings.rs"
        );
        include!("types/siterecovery/replication_recovery_plan_boot_recovery_group.rs");
        include!(
            "types/siterecovery/replication_recovery_plan_boot_recovery_group_post_action.rs"
        );
        include!(
            "types/siterecovery/replication_recovery_plan_boot_recovery_group_pre_action.rs"
        );
        include!(
            "types/siterecovery/replication_recovery_plan_failover_recovery_group.rs"
        );
        include!(
            "types/siterecovery/replication_recovery_plan_failover_recovery_group_post_action.rs"
        );
        include!(
            "types/siterecovery/replication_recovery_plan_failover_recovery_group_pre_action.rs"
        );
        include!(
            "types/siterecovery/replication_recovery_plan_shutdown_recovery_group.rs"
        );
        include!(
            "types/siterecovery/replication_recovery_plan_shutdown_recovery_group_post_action.rs"
        );
        include!(
            "types/siterecovery/replication_recovery_plan_shutdown_recovery_group_pre_action.rs"
        );
        include!("types/siterecovery/vmware_replicated_vm_managed_disk.rs");
        include!("types/siterecovery/vmware_replicated_vm_network_interface.rs");
        include!(
            "types/siterecovery/get_replication_recovery_plan_azure_to_azure_setting.rs"
        );
        include!("types/siterecovery/get_replication_recovery_plan_recovery_group.rs");
        include!(
            "types/siterecovery/get_replication_recovery_plan_recovery_group_post_action.rs"
        );
        include!(
            "types/siterecovery/get_replication_recovery_plan_recovery_group_pre_action.rs"
        );
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
