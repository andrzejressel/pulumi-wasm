pub mod mssql {
    include!("resources/mssql/database.rs");
    include!("resources/mssql/database_extended_auditing_policy.rs");
    include!("resources/mssql/database_vulnerability_assessment_rule_baseline.rs");
    include!("resources/mssql/elastic_pool.rs");
    include!("resources/mssql/failover_group.rs");
    include!("resources/mssql/firewall_rule.rs");
    include!("resources/mssql/job_agent.rs");
    include!("resources/mssql/job_credential.rs");
    include!("resources/mssql/managed_database.rs");
    include!("resources/mssql/managed_instance.rs");
    include!("resources/mssql/managed_instance_active_directory_administrator.rs");
    include!("resources/mssql/managed_instance_failover_group.rs");
    include!("resources/mssql/managed_instance_security_alert_policy.rs");
    include!("resources/mssql/managed_instance_transparent_data_encryption.rs");
    include!("resources/mssql/managed_instance_vulnerability_assessment.rs");
    include!("resources/mssql/outbound_firewall_rule.rs");
    include!("resources/mssql/server.rs");
    include!("resources/mssql/server_dns_alias.rs");
    include!("resources/mssql/server_extended_auditing_policy.rs");
    include!("resources/mssql/server_microsoft_support_auditing_policy.rs");
    include!("resources/mssql/server_security_alert_policy.rs");
    include!("resources/mssql/server_transparent_data_encryption.rs");
    include!("resources/mssql/server_vulnerability_assessment.rs");
    include!("resources/mssql/virtual_machine.rs");
    include!("resources/mssql/virtual_machine_availability_group_listener.rs");
    include!("resources/mssql/virtual_machine_group.rs");
    include!("resources/mssql/virtual_network_rule.rs");
}
pub mod functions {
    pub mod mssql {
        include!("functions/mssql/get_database.rs");
        include!("functions/mssql/get_elastic_pool.rs");
        include!("functions/mssql/get_managed_database.rs");
        include!("functions/mssql/get_managed_instance.rs");
        include!("functions/mssql/get_server.rs");
    }
}
pub mod types {
    pub mod mssql {
        include!("types/mssql/database_identity.rs");
        include!("types/mssql/database_import.rs");
        include!("types/mssql/database_long_term_retention_policy.rs");
        include!("types/mssql/database_short_term_retention_policy.rs");
        include!("types/mssql/database_threat_detection_policy.rs");
        include!(
            "types/mssql/database_vulnerability_assessment_rule_baseline_baseline_result.rs"
        );
        include!("types/mssql/elastic_pool_per_database_settings.rs");
        include!("types/mssql/elastic_pool_sku.rs");
        include!("types/mssql/failover_group_partner_server.rs");
        include!("types/mssql/failover_group_read_write_endpoint_failover_policy.rs");
        include!("types/mssql/managed_database_long_term_retention_policy.rs");
        include!("types/mssql/managed_database_point_in_time_restore.rs");
        include!("types/mssql/managed_instance_failover_group_partner_region.rs");
        include!(
            "types/mssql/managed_instance_failover_group_read_write_endpoint_failover_policy.rs"
        );
        include!("types/mssql/managed_instance_identity.rs");
        include!(
            "types/mssql/managed_instance_vulnerability_assessment_recurring_scans.rs"
        );
        include!("types/mssql/server_azuread_administrator.rs");
        include!("types/mssql/server_identity.rs");
        include!("types/mssql/server_vulnerability_assessment_recurring_scans.rs");
        include!("types/mssql/virtual_machine_assessment.rs");
        include!("types/mssql/virtual_machine_assessment_schedule.rs");
        include!("types/mssql/virtual_machine_auto_backup.rs");
        include!("types/mssql/virtual_machine_auto_backup_manual_schedule.rs");
        include!("types/mssql/virtual_machine_auto_patching.rs");
        include!(
            "types/mssql/virtual_machine_availability_group_listener_load_balancer_configuration.rs"
        );
        include!(
            "types/mssql/virtual_machine_availability_group_listener_multi_subnet_ip_configuration.rs"
        );
        include!("types/mssql/virtual_machine_availability_group_listener_replica.rs");
        include!("types/mssql/virtual_machine_group_wsfc_domain_profile.rs");
        include!("types/mssql/virtual_machine_key_vault_credential.rs");
        include!("types/mssql/virtual_machine_sql_instance.rs");
        include!("types/mssql/virtual_machine_storage_configuration.rs");
        include!("types/mssql/virtual_machine_storage_configuration_data_settings.rs");
        include!("types/mssql/virtual_machine_storage_configuration_log_settings.rs");
        include!(
            "types/mssql/virtual_machine_storage_configuration_temp_db_settings.rs"
        );
        include!("types/mssql/virtual_machine_wsfc_domain_credential.rs");
        include!("types/mssql/get_database_identity.rs");
        include!("types/mssql/get_elastic_pool_skus.rs");
        include!("types/mssql/get_managed_database_long_term_retention_policy.rs");
        include!("types/mssql/get_managed_database_point_in_time_restore.rs");
        include!("types/mssql/get_managed_instance_identity.rs");
        include!("types/mssql/get_server_identity.rs");
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
