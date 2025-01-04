pub mod datafactory {
    include!("resources/datafactory/credential_service_principal.rs");
    include!("resources/datafactory/credential_user_managed_identity.rs");
    include!("resources/datafactory/custom_dataset.rs");
    include!("resources/datafactory/data_flow.rs");
    include!("resources/datafactory/dataset_azure_blob.rs");
    include!("resources/datafactory/dataset_azure_sql_table.rs");
    include!("resources/datafactory/dataset_binary.rs");
    include!("resources/datafactory/dataset_cosmos_db_api.rs");
    include!("resources/datafactory/dataset_delimited_text.rs");
    include!("resources/datafactory/dataset_http.rs");
    include!("resources/datafactory/dataset_json.rs");
    include!("resources/datafactory/dataset_mysql.rs");
    include!("resources/datafactory/dataset_parquet.rs");
    include!("resources/datafactory/dataset_postgresql.rs");
    include!("resources/datafactory/dataset_snowflake.rs");
    include!("resources/datafactory/dataset_sql_server_table.rs");
    include!("resources/datafactory/factory.rs");
    include!("resources/datafactory/flowlet_data_flow.rs");
    include!("resources/datafactory/integration_runtime_rule.rs");
    include!("resources/datafactory/integration_runtime_self_hosted.rs");
    include!("resources/datafactory/integration_runtime_ssis.rs");
    include!("resources/datafactory/linked_custom_service.rs");
    include!("resources/datafactory/linked_service_azure_blob_storage.rs");
    include!("resources/datafactory/linked_service_azure_databricks.rs");
    include!("resources/datafactory/linked_service_azure_file_storage.rs");
    include!("resources/datafactory/linked_service_azure_function.rs");
    include!("resources/datafactory/linked_service_azure_search.rs");
    include!("resources/datafactory/linked_service_azure_sql_database.rs");
    include!("resources/datafactory/linked_service_azure_table_storage.rs");
    include!("resources/datafactory/linked_service_cosmos_db.rs");
    include!("resources/datafactory/linked_service_cosmos_db_mongo_api.rs");
    include!("resources/datafactory/linked_service_data_lake_storage_gen_2.rs");
    include!("resources/datafactory/linked_service_key_vault.rs");
    include!("resources/datafactory/linked_service_kusto.rs");
    include!("resources/datafactory/linked_service_mysql.rs");
    include!("resources/datafactory/linked_service_odata.rs");
    include!("resources/datafactory/linked_service_odbc.rs");
    include!("resources/datafactory/linked_service_postgresql.rs");
    include!("resources/datafactory/linked_service_sftp.rs");
    include!("resources/datafactory/linked_service_snowflake.rs");
    include!("resources/datafactory/linked_service_sql_server.rs");
    include!("resources/datafactory/linked_service_synapse.rs");
    include!("resources/datafactory/linked_service_web.rs");
    include!("resources/datafactory/managed_private_endpoint.rs");
    include!("resources/datafactory/pipeline.rs");
    include!("resources/datafactory/trigger_blob_event.rs");
    include!("resources/datafactory/trigger_custom_event.rs");
    include!("resources/datafactory/trigger_schedule.rs");
    include!("resources/datafactory/trigger_tumbling_window.rs");
}
pub mod functions {
    pub mod datafactory {
        include!("functions/datafactory/get_factory.rs");
        include!("functions/datafactory/get_trigger_schedule.rs");
        include!("functions/datafactory/get_trigger_schedules.rs");
    }
}
pub mod types {
    pub mod datafactory {
        include!(
            "types/datafactory/credential_service_principal_service_principal_key.rs"
        );
        include!("types/datafactory/custom_dataset_linked_service.rs");
        include!("types/datafactory/data_flow_sink.rs");
        include!("types/datafactory/data_flow_sink_dataset.rs");
        include!("types/datafactory/data_flow_sink_flowlet.rs");
        include!("types/datafactory/data_flow_sink_linked_service.rs");
        include!("types/datafactory/data_flow_sink_rejected_linked_service.rs");
        include!("types/datafactory/data_flow_sink_schema_linked_service.rs");
        include!("types/datafactory/data_flow_source.rs");
        include!("types/datafactory/data_flow_source_dataset.rs");
        include!("types/datafactory/data_flow_source_flowlet.rs");
        include!("types/datafactory/data_flow_source_linked_service.rs");
        include!("types/datafactory/data_flow_source_rejected_linked_service.rs");
        include!("types/datafactory/data_flow_source_schema_linked_service.rs");
        include!("types/datafactory/data_flow_transformation.rs");
        include!("types/datafactory/data_flow_transformation_dataset.rs");
        include!("types/datafactory/data_flow_transformation_flowlet.rs");
        include!("types/datafactory/data_flow_transformation_linked_service.rs");
        include!("types/datafactory/dataset_azure_blob_schema_column.rs");
        include!("types/datafactory/dataset_azure_sql_table_schema_column.rs");
        include!("types/datafactory/dataset_binary_azure_blob_storage_location.rs");
        include!("types/datafactory/dataset_binary_compression.rs");
        include!("types/datafactory/dataset_binary_http_server_location.rs");
        include!("types/datafactory/dataset_binary_sftp_server_location.rs");
        include!("types/datafactory/dataset_cosmos_db_api_schema_column.rs");
        include!("types/datafactory/dataset_delimited_text_azure_blob_fs_location.rs");
        include!(
            "types/datafactory/dataset_delimited_text_azure_blob_storage_location.rs"
        );
        include!("types/datafactory/dataset_delimited_text_http_server_location.rs");
        include!("types/datafactory/dataset_delimited_text_schema_column.rs");
        include!("types/datafactory/dataset_http_schema_column.rs");
        include!("types/datafactory/dataset_json_azure_blob_storage_location.rs");
        include!("types/datafactory/dataset_json_http_server_location.rs");
        include!("types/datafactory/dataset_json_schema_column.rs");
        include!("types/datafactory/dataset_mysql_schema_column.rs");
        include!("types/datafactory/dataset_parquet_azure_blob_fs_location.rs");
        include!("types/datafactory/dataset_parquet_azure_blob_storage_location.rs");
        include!("types/datafactory/dataset_parquet_http_server_location.rs");
        include!("types/datafactory/dataset_parquet_schema_column.rs");
        include!("types/datafactory/dataset_postgresql_schema_column.rs");
        include!("types/datafactory/dataset_snowflake_schema_column.rs");
        include!("types/datafactory/dataset_sql_server_table_schema_column.rs");
        include!("types/datafactory/factory_github_configuration.rs");
        include!("types/datafactory/factory_global_parameter.rs");
        include!("types/datafactory/factory_identity.rs");
        include!("types/datafactory/factory_vsts_configuration.rs");
        include!("types/datafactory/flowlet_data_flow_sink.rs");
        include!("types/datafactory/flowlet_data_flow_sink_dataset.rs");
        include!("types/datafactory/flowlet_data_flow_sink_flowlet.rs");
        include!("types/datafactory/flowlet_data_flow_sink_linked_service.rs");
        include!("types/datafactory/flowlet_data_flow_sink_rejected_linked_service.rs");
        include!("types/datafactory/flowlet_data_flow_sink_schema_linked_service.rs");
        include!("types/datafactory/flowlet_data_flow_source.rs");
        include!("types/datafactory/flowlet_data_flow_source_dataset.rs");
        include!("types/datafactory/flowlet_data_flow_source_flowlet.rs");
        include!("types/datafactory/flowlet_data_flow_source_linked_service.rs");
        include!(
            "types/datafactory/flowlet_data_flow_source_rejected_linked_service.rs"
        );
        include!("types/datafactory/flowlet_data_flow_source_schema_linked_service.rs");
        include!("types/datafactory/flowlet_data_flow_transformation.rs");
        include!("types/datafactory/flowlet_data_flow_transformation_dataset.rs");
        include!("types/datafactory/flowlet_data_flow_transformation_flowlet.rs");
        include!("types/datafactory/flowlet_data_flow_transformation_linked_service.rs");
        include!(
            "types/datafactory/integration_runtime_self_hosted_rbac_authorization.rs"
        );
        include!("types/datafactory/integration_runtime_ssis_catalog_info.rs");
        include!("types/datafactory/integration_runtime_ssis_copy_compute_scale.rs");
        include!("types/datafactory/integration_runtime_ssis_custom_setup_script.rs");
        include!("types/datafactory/integration_runtime_ssis_express_custom_setup.rs");
        include!(
            "types/datafactory/integration_runtime_ssis_express_custom_setup_command_key.rs"
        );
        include!(
            "types/datafactory/integration_runtime_ssis_express_custom_setup_command_key_key_vault_password.rs"
        );
        include!(
            "types/datafactory/integration_runtime_ssis_express_custom_setup_component.rs"
        );
        include!(
            "types/datafactory/integration_runtime_ssis_express_custom_setup_component_key_vault_license.rs"
        );
        include!(
            "types/datafactory/integration_runtime_ssis_express_vnet_integration.rs"
        );
        include!("types/datafactory/integration_runtime_ssis_package_store.rs");
        include!(
            "types/datafactory/integration_runtime_ssis_pipeline_external_compute_scale.rs"
        );
        include!("types/datafactory/integration_runtime_ssis_proxy.rs");
        include!("types/datafactory/integration_runtime_ssis_vnet_integration.rs");
        include!("types/datafactory/linked_custom_service_integration_runtime.rs");
        include!(
            "types/datafactory/linked_service_azure_blob_storage_key_vault_sas_token.rs"
        );
        include!(
            "types/datafactory/linked_service_azure_blob_storage_service_principal_linked_key_vault_key.rs"
        );
        include!("types/datafactory/linked_service_azure_databricks_instance_pool.rs");
        include!(
            "types/datafactory/linked_service_azure_databricks_key_vault_password.rs"
        );
        include!(
            "types/datafactory/linked_service_azure_databricks_new_cluster_config.rs"
        );
        include!(
            "types/datafactory/linked_service_azure_file_storage_key_vault_password.rs"
        );
        include!("types/datafactory/linked_service_azure_function_key_vault_key.rs");
        include!(
            "types/datafactory/linked_service_azure_sql_database_key_vault_connection_string.rs"
        );
        include!(
            "types/datafactory/linked_service_azure_sql_database_key_vault_password.rs"
        );
        include!("types/datafactory/linked_service_odata_basic_authentication.rs");
        include!("types/datafactory/linked_service_odbc_basic_authentication.rs");
        include!("types/datafactory/linked_service_snowflake_key_vault_password.rs");
        include!(
            "types/datafactory/linked_service_sql_server_key_vault_connection_string.rs"
        );
        include!("types/datafactory/linked_service_sql_server_key_vault_password.rs");
        include!("types/datafactory/linked_service_synapse_key_vault_password.rs");
        include!("types/datafactory/trigger_blob_event_pipeline.rs");
        include!("types/datafactory/trigger_custom_event_pipeline.rs");
        include!("types/datafactory/trigger_schedule_pipeline.rs");
        include!("types/datafactory/trigger_schedule_schedule.rs");
        include!("types/datafactory/trigger_schedule_schedule_monthly.rs");
        include!("types/datafactory/trigger_tumbling_window_pipeline.rs");
        include!("types/datafactory/trigger_tumbling_window_retry.rs");
        include!("types/datafactory/trigger_tumbling_window_trigger_dependency.rs");
        include!("types/datafactory/get_factory_github_configuration.rs");
        include!("types/datafactory/get_factory_identity.rs");
        include!("types/datafactory/get_factory_vsts_configuration.rs");
        include!("types/datafactory/get_trigger_schedule_schedule.rs");
        include!("types/datafactory/get_trigger_schedule_schedule_monthly.rs");
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
