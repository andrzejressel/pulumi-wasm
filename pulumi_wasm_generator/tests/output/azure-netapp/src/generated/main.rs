pub mod netapp {
    include!("resources/netapp/account.rs");
    include!("resources/netapp/account_encryption.rs");
    include!("resources/netapp/backup_policy.rs");
    include!("resources/netapp/backup_vault.rs");
    include!("resources/netapp/pool.rs");
    include!("resources/netapp/snapshot.rs");
    include!("resources/netapp/snapshot_policy.rs");
    include!("resources/netapp/volume.rs");
    include!("resources/netapp/volume_group_sap_hana.rs");
    include!("resources/netapp/volume_quota_rule.rs");
}
pub mod functions {
    pub mod netapp {
        include!("functions/netapp/get_account.rs");
        include!("functions/netapp/get_account_encryption.rs");
        include!("functions/netapp/get_backup_policy.rs");
        include!("functions/netapp/get_backup_vault.rs");
        include!("functions/netapp/get_pool.rs");
        include!("functions/netapp/get_snapshot.rs");
        include!("functions/netapp/get_snapshot_policy.rs");
        include!("functions/netapp/get_volume.rs");
        include!("functions/netapp/get_volume_group_sap_hana.rs");
        include!("functions/netapp/get_volume_quota_rule.rs");
    }
}
pub mod types {
    pub mod netapp {
        include!("types/netapp/account_active_directory.rs");
        include!("types/netapp/account_identity.rs");
        include!("types/netapp/snapshot_policy_daily_schedule.rs");
        include!("types/netapp/snapshot_policy_hourly_schedule.rs");
        include!("types/netapp/snapshot_policy_monthly_schedule.rs");
        include!("types/netapp/snapshot_policy_weekly_schedule.rs");
        include!("types/netapp/volume_data_protection_backup_policy.rs");
        include!("types/netapp/volume_data_protection_replication.rs");
        include!("types/netapp/volume_data_protection_snapshot_policy.rs");
        include!("types/netapp/volume_export_policy_rule.rs");
        include!("types/netapp/volume_group_sap_hana_volume.rs");
        include!(
            "types/netapp/volume_group_sap_hana_volume_data_protection_replication.rs"
        );
        include!(
            "types/netapp/volume_group_sap_hana_volume_data_protection_snapshot_policy.rs"
        );
        include!("types/netapp/volume_group_sap_hana_volume_export_policy_rule.rs");
        include!("types/netapp/get_account_identity.rs");
        include!("types/netapp/get_snapshot_policy_daily_schedule.rs");
        include!("types/netapp/get_snapshot_policy_hourly_schedule.rs");
        include!("types/netapp/get_snapshot_policy_monthly_schedule.rs");
        include!("types/netapp/get_snapshot_policy_weekly_schedule.rs");
        include!("types/netapp/get_volume_data_protection_backup_policy.rs");
        include!("types/netapp/get_volume_data_protection_replication.rs");
        include!("types/netapp/get_volume_group_sap_hana_volume.rs");
        include!(
            "types/netapp/get_volume_group_sap_hana_volume_data_protection_replication.rs"
        );
        include!(
            "types/netapp/get_volume_group_sap_hana_volume_data_protection_snapshot_policy.rs"
        );
        include!("types/netapp/get_volume_group_sap_hana_volume_export_policy_rule.rs");
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
