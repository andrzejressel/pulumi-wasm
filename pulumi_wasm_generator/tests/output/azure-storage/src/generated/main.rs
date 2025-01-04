pub mod storage {
    include!("resources/storage/account.rs");
    include!("resources/storage/account_network_rules.rs");
    include!("resources/storage/account_queue_properties.rs");
    include!("resources/storage/account_static_website.rs");
    include!("resources/storage/blob.rs");
    include!("resources/storage/blob_inventory_policy.rs");
    include!("resources/storage/container.rs");
    include!("resources/storage/container_immutability_policy.rs");
    include!("resources/storage/customer_managed_key.rs");
    include!("resources/storage/data_lake_gen_2_filesystem.rs");
    include!("resources/storage/data_lake_gen_2_path.rs");
    include!("resources/storage/encryption_scope.rs");
    include!("resources/storage/local_user.rs");
    include!("resources/storage/management_policy.rs");
    include!("resources/storage/mover.rs");
    include!("resources/storage/mover_agent.rs");
    include!("resources/storage/mover_job_definition.rs");
    include!("resources/storage/mover_project.rs");
    include!("resources/storage/mover_source_endpoint.rs");
    include!("resources/storage/mover_target_endpoint.rs");
    include!("resources/storage/object_replication.rs");
    include!("resources/storage/queue.rs");
    include!("resources/storage/share.rs");
    include!("resources/storage/share_directory.rs");
    include!("resources/storage/share_file.rs");
    include!("resources/storage/sync.rs");
    include!("resources/storage/sync_cloud_endpoint.rs");
    include!("resources/storage/sync_group.rs");
    include!("resources/storage/sync_server_endpoint.rs");
    include!("resources/storage/table.rs");
    include!("resources/storage/table_entity.rs");
    include!("resources/storage/zip_blob.rs");
}
pub mod functions {
    pub mod storage {
        include!("functions/storage/get_account.rs");
        include!("functions/storage/get_account_blob_container_sas.rs");
        include!("functions/storage/get_account_sas.rs");
        include!("functions/storage/get_blob.rs");
        include!("functions/storage/get_containers.rs");
        include!("functions/storage/get_encryption_scope.rs");
        include!("functions/storage/get_policy.rs");
        include!("functions/storage/get_queue.rs");
        include!("functions/storage/get_share.rs");
        include!("functions/storage/get_storage_container.rs");
        include!("functions/storage/get_sync.rs");
        include!("functions/storage/get_sync_group.rs");
        include!("functions/storage/get_table.rs");
        include!("functions/storage/get_table_entities.rs");
        include!("functions/storage/get_table_entity.rs");
    }
}
pub mod types {
    pub mod storage {
        include!("types/storage/account_azure_files_authentication.rs");
        include!("types/storage/account_azure_files_authentication_active_directory.rs");
        include!("types/storage/account_blob_properties.rs");
        include!(
            "types/storage/account_blob_properties_container_delete_retention_policy.rs"
        );
        include!("types/storage/account_blob_properties_cors_rule.rs");
        include!("types/storage/account_blob_properties_delete_retention_policy.rs");
        include!("types/storage/account_blob_properties_restore_policy.rs");
        include!("types/storage/account_custom_domain.rs");
        include!("types/storage/account_customer_managed_key.rs");
        include!("types/storage/account_identity.rs");
        include!("types/storage/account_immutability_policy.rs");
        include!("types/storage/account_network_rules.rs");
        include!("types/storage/account_network_rules_private_link_access.rs");
        include!("types/storage/account_network_rules_private_link_access_rule.rs");
        include!("types/storage/account_queue_properties.rs");
        include!("types/storage/account_queue_properties_cors_rule.rs");
        include!("types/storage/account_queue_properties_hour_metrics.rs");
        include!("types/storage/account_queue_properties_logging.rs");
        include!("types/storage/account_queue_properties_minute_metrics.rs");
        include!("types/storage/account_routing.rs");
        include!("types/storage/account_sas_policy.rs");
        include!("types/storage/account_share_properties.rs");
        include!("types/storage/account_share_properties_cors_rule.rs");
        include!("types/storage/account_share_properties_retention_policy.rs");
        include!("types/storage/account_share_properties_smb.rs");
        include!("types/storage/account_static_website.rs");
        include!("types/storage/blob_inventory_policy_rule.rs");
        include!("types/storage/blob_inventory_policy_rule_filter.rs");
        include!("types/storage/data_lake_gen_2_filesystem_ace.rs");
        include!("types/storage/data_lake_gen_2_path_ace.rs");
        include!("types/storage/local_user_permission_scope.rs");
        include!("types/storage/local_user_permission_scope_permissions.rs");
        include!("types/storage/local_user_ssh_authorized_key.rs");
        include!("types/storage/management_policy_rule.rs");
        include!("types/storage/management_policy_rule_actions.rs");
        include!("types/storage/management_policy_rule_actions_base_blob.rs");
        include!("types/storage/management_policy_rule_actions_snapshot.rs");
        include!("types/storage/management_policy_rule_actions_version.rs");
        include!("types/storage/management_policy_rule_filters.rs");
        include!("types/storage/management_policy_rule_filters_match_blob_index_tag.rs");
        include!("types/storage/object_replication_rule.rs");
        include!("types/storage/share_acl.rs");
        include!("types/storage/share_acl_access_policy.rs");
        include!("types/storage/table_acl.rs");
        include!("types/storage/table_acl_access_policy.rs");
        include!("types/storage/get_account_azure_files_authentication.rs");
        include!(
            "types/storage/get_account_azure_files_authentication_active_directory.rs"
        );
        include!("types/storage/get_account_blob_container_sas_permissions.rs");
        include!("types/storage/get_account_custom_domain.rs");
        include!("types/storage/get_account_identity.rs");
        include!("types/storage/get_account_sas_permissions.rs");
        include!("types/storage/get_account_sas_resource_types.rs");
        include!("types/storage/get_account_sas_services.rs");
        include!("types/storage/get_containers_container.rs");
        include!("types/storage/get_policy_rule.rs");
        include!("types/storage/get_policy_rule_action.rs");
        include!("types/storage/get_policy_rule_action_base_blob.rs");
        include!("types/storage/get_policy_rule_action_snapshot.rs");
        include!("types/storage/get_policy_rule_action_version.rs");
        include!("types/storage/get_policy_rule_filter.rs");
        include!("types/storage/get_policy_rule_filter_match_blob_index_tag.rs");
        include!("types/storage/get_share_acl.rs");
        include!("types/storage/get_share_acl_access_policy.rs");
        include!("types/storage/get_table_acl.rs");
        include!("types/storage/get_table_acl_access_policy.rs");
        include!("types/storage/get_table_entities_item.rs");
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
