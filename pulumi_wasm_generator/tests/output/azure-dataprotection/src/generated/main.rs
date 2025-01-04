pub mod dataprotection {
    include!("resources/dataprotection/backup_instance_blog_storage.rs");
    include!("resources/dataprotection/backup_instance_disk.rs");
    include!("resources/dataprotection/backup_instance_kubernetes_cluster.rs");
    include!("resources/dataprotection/backup_instance_mysql_flexible_server.rs");
    include!("resources/dataprotection/backup_instance_postgresql.rs");
    include!("resources/dataprotection/backup_instance_postgresql_flexible_server.rs");
    include!("resources/dataprotection/backup_policy_blob_storage.rs");
    include!("resources/dataprotection/backup_policy_disk.rs");
    include!("resources/dataprotection/backup_policy_kubernetes_cluster.rs");
    include!("resources/dataprotection/backup_policy_mysql_flexible_server.rs");
    include!("resources/dataprotection/backup_policy_postgresql.rs");
    include!("resources/dataprotection/backup_policy_postgresql_flexible_server.rs");
    include!("resources/dataprotection/backup_vault.rs");
    include!("resources/dataprotection/resource_guard.rs");
}
pub mod functions {
    pub mod dataprotection {
        include!("functions/dataprotection/get_backup_vault.rs");
    }
}
pub mod types {
    pub mod dataprotection {
        include!(
            "types/dataprotection/backup_instance_kubernetes_cluster_backup_datasource_parameters.rs"
        );
        include!("types/dataprotection/backup_policy_blob_storage_retention_rule.rs");
        include!(
            "types/dataprotection/backup_policy_blob_storage_retention_rule_criteria.rs"
        );
        include!(
            "types/dataprotection/backup_policy_blob_storage_retention_rule_life_cycle.rs"
        );
        include!("types/dataprotection/backup_policy_disk_retention_rule.rs");
        include!("types/dataprotection/backup_policy_disk_retention_rule_criteria.rs");
        include!(
            "types/dataprotection/backup_policy_kubernetes_cluster_default_retention_rule.rs"
        );
        include!(
            "types/dataprotection/backup_policy_kubernetes_cluster_default_retention_rule_life_cycle.rs"
        );
        include!(
            "types/dataprotection/backup_policy_kubernetes_cluster_retention_rule.rs"
        );
        include!(
            "types/dataprotection/backup_policy_kubernetes_cluster_retention_rule_criteria.rs"
        );
        include!(
            "types/dataprotection/backup_policy_kubernetes_cluster_retention_rule_life_cycle.rs"
        );
        include!(
            "types/dataprotection/backup_policy_mysql_flexible_server_default_retention_rule.rs"
        );
        include!(
            "types/dataprotection/backup_policy_mysql_flexible_server_default_retention_rule_life_cycle.rs"
        );
        include!(
            "types/dataprotection/backup_policy_mysql_flexible_server_retention_rule.rs"
        );
        include!(
            "types/dataprotection/backup_policy_mysql_flexible_server_retention_rule_criteria.rs"
        );
        include!(
            "types/dataprotection/backup_policy_mysql_flexible_server_retention_rule_life_cycle.rs"
        );
        include!(
            "types/dataprotection/backup_policy_postgresql_flexible_server_default_retention_rule.rs"
        );
        include!(
            "types/dataprotection/backup_policy_postgresql_flexible_server_default_retention_rule_life_cycle.rs"
        );
        include!(
            "types/dataprotection/backup_policy_postgresql_flexible_server_retention_rule.rs"
        );
        include!(
            "types/dataprotection/backup_policy_postgresql_flexible_server_retention_rule_criteria.rs"
        );
        include!(
            "types/dataprotection/backup_policy_postgresql_flexible_server_retention_rule_life_cycle.rs"
        );
        include!("types/dataprotection/backup_policy_postgresql_retention_rule.rs");
        include!(
            "types/dataprotection/backup_policy_postgresql_retention_rule_criteria.rs"
        );
        include!("types/dataprotection/backup_vault_identity.rs");
        include!("types/dataprotection/get_backup_vault_identity.rs");
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
