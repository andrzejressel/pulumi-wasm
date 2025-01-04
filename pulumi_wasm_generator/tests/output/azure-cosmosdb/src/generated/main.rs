pub mod cosmosdb {
    include!("resources/cosmosdb/account.rs");
    include!("resources/cosmosdb/cassandra_cluster.rs");
    include!("resources/cosmosdb/cassandra_datacenter.rs");
    include!("resources/cosmosdb/cassandra_keyspace.rs");
    include!("resources/cosmosdb/cassandra_table.rs");
    include!("resources/cosmosdb/gremlin_database.rs");
    include!("resources/cosmosdb/gremlin_graph.rs");
    include!("resources/cosmosdb/mongo_cluster.rs");
    include!("resources/cosmosdb/mongo_collection.rs");
    include!("resources/cosmosdb/mongo_database.rs");
    include!("resources/cosmosdb/mongo_role_definition.rs");
    include!("resources/cosmosdb/mongo_user_definition.rs");
    include!("resources/cosmosdb/postgresql_cluster.rs");
    include!("resources/cosmosdb/postgresql_coordinator_configuration.rs");
    include!("resources/cosmosdb/postgresql_firewall_rule.rs");
    include!("resources/cosmosdb/postgresql_node_configuration.rs");
    include!("resources/cosmosdb/postgresql_role.rs");
    include!("resources/cosmosdb/sql_container.rs");
    include!("resources/cosmosdb/sql_database.rs");
    include!("resources/cosmosdb/sql_dedicated_gateway.rs");
    include!("resources/cosmosdb/sql_function.rs");
    include!("resources/cosmosdb/sql_role_assignment.rs");
    include!("resources/cosmosdb/sql_role_definition.rs");
    include!("resources/cosmosdb/sql_stored_procedure.rs");
    include!("resources/cosmosdb/sql_trigger.rs");
    include!("resources/cosmosdb/table.rs");
}
pub mod functions {
    pub mod cosmosdb {
        include!("functions/cosmosdb/get_account.rs");
        include!("functions/cosmosdb/get_mongo_database.rs");
        include!("functions/cosmosdb/get_restorable_database_accounts.rs");
        include!("functions/cosmosdb/get_sql_database.rs");
        include!("functions/cosmosdb/get_sql_role_definition.rs");
    }
}
pub mod types {
    pub mod cosmosdb {
        include!("types/cosmosdb/account_analytical_storage.rs");
        include!("types/cosmosdb/account_backup.rs");
        include!("types/cosmosdb/account_capability.rs");
        include!("types/cosmosdb/account_capacity.rs");
        include!("types/cosmosdb/account_consistency_policy.rs");
        include!("types/cosmosdb/account_cors_rule.rs");
        include!("types/cosmosdb/account_geo_location.rs");
        include!("types/cosmosdb/account_identity.rs");
        include!("types/cosmosdb/account_restore.rs");
        include!("types/cosmosdb/account_restore_database.rs");
        include!("types/cosmosdb/account_restore_gremlin_database.rs");
        include!("types/cosmosdb/account_virtual_network_rule.rs");
        include!("types/cosmosdb/cassandra_cluster_identity.rs");
        include!("types/cosmosdb/cassandra_keyspace_autoscale_settings.rs");
        include!("types/cosmosdb/cassandra_table_autoscale_settings.rs");
        include!("types/cosmosdb/cassandra_table_schema.rs");
        include!("types/cosmosdb/cassandra_table_schema_cluster_key.rs");
        include!("types/cosmosdb/cassandra_table_schema_column.rs");
        include!("types/cosmosdb/cassandra_table_schema_partition_key.rs");
        include!("types/cosmosdb/gremlin_database_autoscale_settings.rs");
        include!("types/cosmosdb/gremlin_graph_autoscale_settings.rs");
        include!("types/cosmosdb/gremlin_graph_conflict_resolution_policy.rs");
        include!("types/cosmosdb/gremlin_graph_index_policy.rs");
        include!("types/cosmosdb/gremlin_graph_index_policy_composite_index.rs");
        include!("types/cosmosdb/gremlin_graph_index_policy_composite_index_index.rs");
        include!("types/cosmosdb/gremlin_graph_index_policy_spatial_index.rs");
        include!("types/cosmosdb/gremlin_graph_unique_key.rs");
        include!("types/cosmosdb/mongo_collection_autoscale_settings.rs");
        include!("types/cosmosdb/mongo_collection_index.rs");
        include!("types/cosmosdb/mongo_collection_system_index.rs");
        include!("types/cosmosdb/mongo_database_autoscale_settings.rs");
        include!("types/cosmosdb/mongo_role_definition_privilege.rs");
        include!("types/cosmosdb/mongo_role_definition_privilege_resource.rs");
        include!("types/cosmosdb/postgresql_cluster_maintenance_window.rs");
        include!("types/cosmosdb/postgresql_cluster_server.rs");
        include!("types/cosmosdb/sql_container_autoscale_settings.rs");
        include!("types/cosmosdb/sql_container_conflict_resolution_policy.rs");
        include!("types/cosmosdb/sql_container_indexing_policy.rs");
        include!("types/cosmosdb/sql_container_indexing_policy_composite_index.rs");
        include!(
            "types/cosmosdb/sql_container_indexing_policy_composite_index_index.rs"
        );
        include!("types/cosmosdb/sql_container_indexing_policy_excluded_path.rs");
        include!("types/cosmosdb/sql_container_indexing_policy_included_path.rs");
        include!("types/cosmosdb/sql_container_indexing_policy_spatial_index.rs");
        include!("types/cosmosdb/sql_container_unique_key.rs");
        include!("types/cosmosdb/sql_database_autoscale_settings.rs");
        include!("types/cosmosdb/sql_role_definition_permission.rs");
        include!("types/cosmosdb/table_autoscale_settings.rs");
        include!("types/cosmosdb/get_account_capability.rs");
        include!("types/cosmosdb/get_account_consistency_policy.rs");
        include!("types/cosmosdb/get_account_geo_location.rs");
        include!("types/cosmosdb/get_account_virtual_network_rule.rs");
        include!("types/cosmosdb/get_restorable_database_accounts_account.rs");
        include!(
            "types/cosmosdb/get_restorable_database_accounts_account_restorable_location.rs"
        );
        include!("types/cosmosdb/get_sql_database_autoscale_setting.rs");
        include!("types/cosmosdb/get_sql_role_definition_permission.rs");
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
