pub mod redis {
    include!("resources/redis/cluster.rs");
    include!("resources/redis/instance.rs");
}
pub mod resourcemanager {
    include!("resources/resourcemanager/lien.rs");
}
pub mod runtimeconfig {
    include!("resources/runtimeconfig/config.rs");
    include!("resources/runtimeconfig/config_iam_binding.rs");
    include!("resources/runtimeconfig/config_iam_member.rs");
    include!("resources/runtimeconfig/config_iam_policy.rs");
    include!("resources/runtimeconfig/variable.rs");
}
pub mod secretmanager {
    include!("resources/secretmanager/regional_secret.rs");
    include!("resources/secretmanager/regional_secret_iam_binding.rs");
    include!("resources/secretmanager/regional_secret_iam_member.rs");
    include!("resources/secretmanager/regional_secret_iam_policy.rs");
    include!("resources/secretmanager/regional_secret_version.rs");
    include!("resources/secretmanager/secret.rs");
    include!("resources/secretmanager/secret_iam_binding.rs");
    include!("resources/secretmanager/secret_iam_member.rs");
    include!("resources/secretmanager/secret_iam_policy.rs");
    include!("resources/secretmanager/secret_version.rs");
}
pub mod securesourcemanager {
    include!("resources/securesourcemanager/branch_rule.rs");
    include!("resources/securesourcemanager/instance.rs");
    include!("resources/securesourcemanager/instance_iam_binding.rs");
    include!("resources/securesourcemanager/instance_iam_member.rs");
    include!("resources/securesourcemanager/instance_iam_policy.rs");
    include!("resources/securesourcemanager/repository.rs");
    include!("resources/securesourcemanager/repository_iam_binding.rs");
    include!("resources/securesourcemanager/repository_iam_member.rs");
    include!("resources/securesourcemanager/repository_iam_policy.rs");
}
pub mod securitycenter {
    include!("resources/securitycenter/event_threat_detection_custom_module.rs");
    include!("resources/securitycenter/folder_custom_module.rs");
    include!("resources/securitycenter/folder_notification_config.rs");
    include!("resources/securitycenter/folder_scc_big_query_export.rs");
    include!("resources/securitycenter/instance_iam_binding.rs");
    include!("resources/securitycenter/instance_iam_member.rs");
    include!("resources/securitycenter/instance_iam_policy.rs");
    include!(
        "resources/securitycenter/management_folder_security_health_analytics_custom_module.rs"
    );
    include!(
        "resources/securitycenter/management_organization_event_threat_detection_custom_module.rs"
    );
    include!(
        "resources/securitycenter/management_organization_security_health_analytics_custom_module.rs"
    );
    include!(
        "resources/securitycenter/management_project_security_health_analytics_custom_module.rs"
    );
    include!("resources/securitycenter/mute_config.rs");
    include!("resources/securitycenter/notification_config.rs");
    include!("resources/securitycenter/organization_custom_module.rs");
    include!("resources/securitycenter/organization_scc_big_query_export.rs");
    include!("resources/securitycenter/project_custom_module.rs");
    include!("resources/securitycenter/project_notification_config.rs");
    include!("resources/securitycenter/project_scc_big_query_export.rs");
    include!("resources/securitycenter/source.rs");
    include!("resources/securitycenter/source_iam_binding.rs");
    include!("resources/securitycenter/source_iam_member.rs");
    include!("resources/securitycenter/source_iam_policy.rs");
    include!("resources/securitycenter/v_2_folder_mute_config.rs");
    include!("resources/securitycenter/v_2_folder_notification_config.rs");
    include!("resources/securitycenter/v_2_folder_scc_big_query_export.rs");
    include!("resources/securitycenter/v_2_organization_mute_config.rs");
    include!("resources/securitycenter/v_2_organization_notification_config.rs");
    include!("resources/securitycenter/v_2_organization_scc_big_query_export.rs");
    include!("resources/securitycenter/v_2_organization_scc_big_query_exports.rs");
    include!("resources/securitycenter/v_2_organization_source.rs");
    include!("resources/securitycenter/v_2_organization_source_iam_binding.rs");
    include!("resources/securitycenter/v_2_organization_source_iam_member.rs");
    include!("resources/securitycenter/v_2_organization_source_iam_policy.rs");
    include!("resources/securitycenter/v_2_project_mute_config.rs");
    include!("resources/securitycenter/v_2_project_notification_config.rs");
    include!("resources/securitycenter/v_2_project_scc_big_query_export.rs");
}
pub mod securityposture {
    include!("resources/securityposture/posture.rs");
    include!("resources/securityposture/posture_deployment.rs");
}
pub mod serviceaccount {
    include!("resources/serviceaccount/account.rs");
    include!("resources/serviceaccount/iam_binding.rs");
    include!("resources/serviceaccount/iam_member.rs");
    include!("resources/serviceaccount/iam_policy.rs");
    include!("resources/serviceaccount/key.rs");
}
pub mod servicedirectory {
    include!("resources/servicedirectory/endpoint.rs");
    include!("resources/servicedirectory/namespace.rs");
    include!("resources/servicedirectory/namespace_iam_binding.rs");
    include!("resources/servicedirectory/namespace_iam_member.rs");
    include!("resources/servicedirectory/namespace_iam_policy.rs");
    include!("resources/servicedirectory/service.rs");
    include!("resources/servicedirectory/service_iam_binding.rs");
    include!("resources/servicedirectory/service_iam_member.rs");
    include!("resources/servicedirectory/service_iam_policy.rs");
}
pub mod servicenetworking {
    include!("resources/servicenetworking/connection.rs");
    include!("resources/servicenetworking/peered_dns_domain.rs");
    include!("resources/servicenetworking/vpc_service_controls.rs");
}
pub mod functions {
    pub mod redis {
        include!("functions/redis/get_instance.rs");
    }
    pub mod runtimeconfig {
        include!("functions/runtimeconfig/get_config.rs");
        include!("functions/runtimeconfig/get_config_iam_policy.rs");
        include!("functions/runtimeconfig/get_variable.rs");
    }
    pub mod secretmanager {
        include!("functions/secretmanager/get_regional_secret.rs");
        include!("functions/secretmanager/get_regional_secret_iam_policy.rs");
        include!("functions/secretmanager/get_regional_secret_version.rs");
        include!("functions/secretmanager/get_regional_secret_version_access.rs");
        include!("functions/secretmanager/get_regional_secrets.rs");
        include!("functions/secretmanager/get_secret.rs");
        include!("functions/secretmanager/get_secret_iam_policy.rs");
        include!("functions/secretmanager/get_secret_version.rs");
        include!("functions/secretmanager/get_secret_version_access.rs");
        include!("functions/secretmanager/get_secrets.rs");
    }
    pub mod securesourcemanager {
        include!("functions/securesourcemanager/get_instance_iam_policy.rs");
        include!("functions/securesourcemanager/get_repository_iam_policy.rs");
    }
    pub mod securitycenter {
        include!("functions/securitycenter/get_source_iam_policy.rs");
        include!("functions/securitycenter/get_v_2_organization_source_iam_policy.rs");
    }
    pub mod serviceaccount {
        include!("functions/serviceaccount/get_account.rs");
        include!("functions/serviceaccount/get_account_access_token.rs");
        include!("functions/serviceaccount/get_account_id_token.rs");
        include!("functions/serviceaccount/get_account_jwt.rs");
        include!("functions/serviceaccount/get_account_key.rs");
        include!("functions/serviceaccount/get_iam_policy.rs");
        include!("functions/serviceaccount/get_s.rs");
    }
    pub mod servicedirectory {
        include!("functions/servicedirectory/get_namespace_iam_policy.rs");
        include!("functions/servicedirectory/get_service_iam_policy.rs");
    }
    pub mod servicenetworking {
        include!("functions/servicenetworking/get_peered_dns_domain.rs");
    }
}
pub mod types {
    pub mod redis {
        include!("types/redis/cluster_cross_cluster_replication_config.rs");
        include!("types/redis/cluster_cross_cluster_replication_config_membership.rs");
        include!(
            "types/redis/cluster_cross_cluster_replication_config_membership_primary_cluster.rs"
        );
        include!(
            "types/redis/cluster_cross_cluster_replication_config_membership_secondary_cluster.rs"
        );
        include!(
            "types/redis/cluster_cross_cluster_replication_config_primary_cluster.rs"
        );
        include!(
            "types/redis/cluster_cross_cluster_replication_config_secondary_cluster.rs"
        );
        include!("types/redis/cluster_discovery_endpoint.rs");
        include!("types/redis/cluster_discovery_endpoint_psc_config.rs");
        include!("types/redis/cluster_maintenance_policy.rs");
        include!("types/redis/cluster_maintenance_policy_weekly_maintenance_window.rs");
        include!(
            "types/redis/cluster_maintenance_policy_weekly_maintenance_window_start_time.rs"
        );
        include!("types/redis/cluster_maintenance_schedule.rs");
        include!("types/redis/cluster_persistence_config.rs");
        include!("types/redis/cluster_persistence_config_aof_config.rs");
        include!("types/redis/cluster_persistence_config_rdb_config.rs");
        include!("types/redis/cluster_psc_config.rs");
        include!("types/redis/cluster_psc_connection.rs");
        include!("types/redis/cluster_state_info.rs");
        include!("types/redis/cluster_state_info_update_info.rs");
        include!("types/redis/cluster_zone_distribution_config.rs");
        include!("types/redis/instance_maintenance_policy.rs");
        include!("types/redis/instance_maintenance_policy_weekly_maintenance_window.rs");
        include!(
            "types/redis/instance_maintenance_policy_weekly_maintenance_window_start_time.rs"
        );
        include!("types/redis/instance_maintenance_schedule.rs");
        include!("types/redis/instance_node.rs");
        include!("types/redis/instance_persistence_config.rs");
        include!("types/redis/instance_server_ca_cert.rs");
        include!("types/redis/get_instance_maintenance_policy.rs");
        include!(
            "types/redis/get_instance_maintenance_policy_weekly_maintenance_window.rs"
        );
        include!(
            "types/redis/get_instance_maintenance_policy_weekly_maintenance_window_start_time.rs"
        );
        include!("types/redis/get_instance_maintenance_schedule.rs");
        include!("types/redis/get_instance_node.rs");
        include!("types/redis/get_instance_persistence_config.rs");
        include!("types/redis/get_instance_server_ca_cert.rs");
    }
    pub mod runtimeconfig {
        include!("types/runtimeconfig/config_iam_binding_condition.rs");
        include!("types/runtimeconfig/config_iam_member_condition.rs");
    }
    pub mod secretmanager {
        include!("types/secretmanager/regional_secret_customer_managed_encryption.rs");
        include!("types/secretmanager/regional_secret_iam_binding_condition.rs");
        include!("types/secretmanager/regional_secret_iam_member_condition.rs");
        include!("types/secretmanager/regional_secret_rotation.rs");
        include!("types/secretmanager/regional_secret_topic.rs");
        include!(
            "types/secretmanager/regional_secret_version_customer_managed_encryption.rs"
        );
        include!("types/secretmanager/secret_iam_binding_condition.rs");
        include!("types/secretmanager/secret_iam_member_condition.rs");
        include!("types/secretmanager/secret_replication.rs");
        include!("types/secretmanager/secret_replication_auto.rs");
        include!(
            "types/secretmanager/secret_replication_auto_customer_managed_encryption.rs"
        );
        include!("types/secretmanager/secret_replication_user_managed.rs");
        include!("types/secretmanager/secret_replication_user_managed_replica.rs");
        include!(
            "types/secretmanager/secret_replication_user_managed_replica_customer_managed_encryption.rs"
        );
        include!("types/secretmanager/secret_rotation.rs");
        include!("types/secretmanager/secret_topic.rs");
        include!(
            "types/secretmanager/get_regional_secret_customer_managed_encryption.rs"
        );
        include!("types/secretmanager/get_regional_secret_rotation.rs");
        include!("types/secretmanager/get_regional_secret_topic.rs");
        include!(
            "types/secretmanager/get_regional_secret_version_customer_managed_encryption.rs"
        );
        include!("types/secretmanager/get_regional_secrets_secret.rs");
        include!(
            "types/secretmanager/get_regional_secrets_secret_customer_managed_encryption.rs"
        );
        include!("types/secretmanager/get_regional_secrets_secret_rotation.rs");
        include!("types/secretmanager/get_regional_secrets_secret_topic.rs");
        include!("types/secretmanager/get_secret_replication.rs");
        include!("types/secretmanager/get_secret_replication_auto.rs");
        include!(
            "types/secretmanager/get_secret_replication_auto_customer_managed_encryption.rs"
        );
        include!("types/secretmanager/get_secret_replication_user_managed.rs");
        include!("types/secretmanager/get_secret_replication_user_managed_replica.rs");
        include!(
            "types/secretmanager/get_secret_replication_user_managed_replica_customer_managed_encryption.rs"
        );
        include!("types/secretmanager/get_secret_rotation.rs");
        include!("types/secretmanager/get_secret_topic.rs");
        include!("types/secretmanager/get_secrets_secret.rs");
        include!("types/secretmanager/get_secrets_secret_replication.rs");
        include!("types/secretmanager/get_secrets_secret_replication_auto.rs");
        include!(
            "types/secretmanager/get_secrets_secret_replication_auto_customer_managed_encryption.rs"
        );
        include!("types/secretmanager/get_secrets_secret_replication_user_managed.rs");
        include!(
            "types/secretmanager/get_secrets_secret_replication_user_managed_replica.rs"
        );
        include!(
            "types/secretmanager/get_secrets_secret_replication_user_managed_replica_customer_managed_encryption.rs"
        );
        include!("types/secretmanager/get_secrets_secret_rotation.rs");
        include!("types/secretmanager/get_secrets_secret_topic.rs");
    }
    pub mod securesourcemanager {
        include!("types/securesourcemanager/instance_host_config.rs");
        include!("types/securesourcemanager/instance_iam_binding_condition.rs");
        include!("types/securesourcemanager/instance_iam_member_condition.rs");
        include!("types/securesourcemanager/instance_private_config.rs");
        include!(
            "types/securesourcemanager/instance_workforce_identity_federation_config.rs"
        );
        include!("types/securesourcemanager/repository_iam_binding_condition.rs");
        include!("types/securesourcemanager/repository_iam_member_condition.rs");
        include!("types/securesourcemanager/repository_initial_config.rs");
        include!("types/securesourcemanager/repository_uri.rs");
    }
    pub mod securitycenter {
        include!("types/securitycenter/folder_custom_module_custom_config.rs");
        include!(
            "types/securitycenter/folder_custom_module_custom_config_custom_output.rs"
        );
        include!(
            "types/securitycenter/folder_custom_module_custom_config_custom_output_property.rs"
        );
        include!(
            "types/securitycenter/folder_custom_module_custom_config_custom_output_property_value_expression.rs"
        );
        include!("types/securitycenter/folder_custom_module_custom_config_predicate.rs");
        include!(
            "types/securitycenter/folder_custom_module_custom_config_resource_selector.rs"
        );
        include!("types/securitycenter/folder_notification_config_streaming_config.rs");
        include!("types/securitycenter/instance_iam_binding_condition.rs");
        include!("types/securitycenter/instance_iam_member_condition.rs");
        include!(
            "types/securitycenter/management_folder_security_health_analytics_custom_module_custom_config.rs"
        );
        include!(
            "types/securitycenter/management_folder_security_health_analytics_custom_module_custom_config_custom_output.rs"
        );
        include!(
            "types/securitycenter/management_folder_security_health_analytics_custom_module_custom_config_custom_output_property.rs"
        );
        include!(
            "types/securitycenter/management_folder_security_health_analytics_custom_module_custom_config_custom_output_property_value_expression.rs"
        );
        include!(
            "types/securitycenter/management_folder_security_health_analytics_custom_module_custom_config_predicate.rs"
        );
        include!(
            "types/securitycenter/management_folder_security_health_analytics_custom_module_custom_config_resource_selector.rs"
        );
        include!(
            "types/securitycenter/management_organization_security_health_analytics_custom_module_custom_config.rs"
        );
        include!(
            "types/securitycenter/management_organization_security_health_analytics_custom_module_custom_config_custom_output.rs"
        );
        include!(
            "types/securitycenter/management_organization_security_health_analytics_custom_module_custom_config_custom_output_property.rs"
        );
        include!(
            "types/securitycenter/management_organization_security_health_analytics_custom_module_custom_config_custom_output_property_value_expression.rs"
        );
        include!(
            "types/securitycenter/management_organization_security_health_analytics_custom_module_custom_config_predicate.rs"
        );
        include!(
            "types/securitycenter/management_organization_security_health_analytics_custom_module_custom_config_resource_selector.rs"
        );
        include!(
            "types/securitycenter/management_project_security_health_analytics_custom_module_custom_config.rs"
        );
        include!(
            "types/securitycenter/management_project_security_health_analytics_custom_module_custom_config_custom_output.rs"
        );
        include!(
            "types/securitycenter/management_project_security_health_analytics_custom_module_custom_config_custom_output_property.rs"
        );
        include!(
            "types/securitycenter/management_project_security_health_analytics_custom_module_custom_config_custom_output_property_value_expression.rs"
        );
        include!(
            "types/securitycenter/management_project_security_health_analytics_custom_module_custom_config_predicate.rs"
        );
        include!(
            "types/securitycenter/management_project_security_health_analytics_custom_module_custom_config_resource_selector.rs"
        );
        include!("types/securitycenter/notification_config_streaming_config.rs");
        include!("types/securitycenter/organization_custom_module_custom_config.rs");
        include!(
            "types/securitycenter/organization_custom_module_custom_config_custom_output.rs"
        );
        include!(
            "types/securitycenter/organization_custom_module_custom_config_custom_output_property.rs"
        );
        include!(
            "types/securitycenter/organization_custom_module_custom_config_custom_output_property_value_expression.rs"
        );
        include!(
            "types/securitycenter/organization_custom_module_custom_config_predicate.rs"
        );
        include!(
            "types/securitycenter/organization_custom_module_custom_config_resource_selector.rs"
        );
        include!("types/securitycenter/project_custom_module_custom_config.rs");
        include!(
            "types/securitycenter/project_custom_module_custom_config_custom_output.rs"
        );
        include!(
            "types/securitycenter/project_custom_module_custom_config_custom_output_property.rs"
        );
        include!(
            "types/securitycenter/project_custom_module_custom_config_custom_output_property_value_expression.rs"
        );
        include!(
            "types/securitycenter/project_custom_module_custom_config_predicate.rs"
        );
        include!(
            "types/securitycenter/project_custom_module_custom_config_resource_selector.rs"
        );
        include!("types/securitycenter/project_notification_config_streaming_config.rs");
        include!("types/securitycenter/source_iam_binding_condition.rs");
        include!("types/securitycenter/source_iam_member_condition.rs");
        include!(
            "types/securitycenter/v_2_folder_notification_config_streaming_config.rs"
        );
        include!(
            "types/securitycenter/v_2_organization_notification_config_streaming_config.rs"
        );
        include!(
            "types/securitycenter/v_2_organization_source_iam_binding_condition.rs"
        );
        include!("types/securitycenter/v_2_organization_source_iam_member_condition.rs");
        include!(
            "types/securitycenter/v_2_project_notification_config_streaming_config.rs"
        );
    }
    pub mod securityposture {
        include!("types/securityposture/posture_policy_set.rs");
        include!("types/securityposture/posture_policy_set_policy.rs");
        include!(
            "types/securityposture/posture_policy_set_policy_compliance_standard.rs"
        );
        include!("types/securityposture/posture_policy_set_policy_constraint.rs");
        include!(
            "types/securityposture/posture_policy_set_policy_constraint_org_policy_constraint.rs"
        );
        include!(
            "types/securityposture/posture_policy_set_policy_constraint_org_policy_constraint_custom.rs"
        );
        include!(
            "types/securityposture/posture_policy_set_policy_constraint_org_policy_constraint_custom_custom_constraint.rs"
        );
        include!(
            "types/securityposture/posture_policy_set_policy_constraint_org_policy_constraint_custom_policy_rule.rs"
        );
        include!(
            "types/securityposture/posture_policy_set_policy_constraint_org_policy_constraint_custom_policy_rule_condition.rs"
        );
        include!(
            "types/securityposture/posture_policy_set_policy_constraint_org_policy_constraint_custom_policy_rule_values.rs"
        );
        include!(
            "types/securityposture/posture_policy_set_policy_constraint_org_policy_constraint_policy_rule.rs"
        );
        include!(
            "types/securityposture/posture_policy_set_policy_constraint_org_policy_constraint_policy_rule_condition.rs"
        );
        include!(
            "types/securityposture/posture_policy_set_policy_constraint_org_policy_constraint_policy_rule_values.rs"
        );
        include!(
            "types/securityposture/posture_policy_set_policy_constraint_security_health_analytics_custom_module.rs"
        );
        include!(
            "types/securityposture/posture_policy_set_policy_constraint_security_health_analytics_custom_module_config.rs"
        );
        include!(
            "types/securityposture/posture_policy_set_policy_constraint_security_health_analytics_custom_module_config_custom_output.rs"
        );
        include!(
            "types/securityposture/posture_policy_set_policy_constraint_security_health_analytics_custom_module_config_custom_output_property.rs"
        );
        include!(
            "types/securityposture/posture_policy_set_policy_constraint_security_health_analytics_custom_module_config_custom_output_property_value_expression.rs"
        );
        include!(
            "types/securityposture/posture_policy_set_policy_constraint_security_health_analytics_custom_module_config_predicate.rs"
        );
        include!(
            "types/securityposture/posture_policy_set_policy_constraint_security_health_analytics_custom_module_config_resource_selector.rs"
        );
        include!(
            "types/securityposture/posture_policy_set_policy_constraint_security_health_analytics_module.rs"
        );
    }
    pub mod serviceaccount {
        include!("types/serviceaccount/iam_binding_condition.rs");
        include!("types/serviceaccount/iam_member_condition.rs");
        include!("types/serviceaccount/get_s_account.rs");
    }
    pub mod servicedirectory {
        include!("types/servicedirectory/namespace_iam_binding_condition.rs");
        include!("types/servicedirectory/namespace_iam_member_condition.rs");
        include!("types/servicedirectory/service_iam_binding_condition.rs");
        include!("types/servicedirectory/service_iam_member_condition.rs");
    }
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-gcp {
    import output-interface;
}

interface pulumi-engine {
    resource engine {
        constructor(in-preview: bool);
    }
}

interface output-interface {
    use pulumi-engine.{engine};

    resource output {
        constructor(engine: borrow<engine>, value: string, secret: bool);
        map: func(function-name: string) -> output;
    }
    combine: func(outputs: list<borrow<output>>) -> output;

    resource register-output {
        extract-field: func(field-name: string) -> output;
    }
}

interface register-interface {
    use pulumi-engine.{engine};
    use output-interface.{output, register-output};

    record object-field {
        name: string,
        value: borrow<output>
    }

    record register-resource-request {
        %type: string,
        name: string,
        version: string,
        object: list<object-field>,
    }

    register: func(engine: borrow<engine>, request: register-resource-request) -> register-output;

    record resource-invoke-request {
        token: string,
        version: string,
        object: list<object-field>,
    }

    invoke: func(engine: borrow<engine>, request: resource-invoke-request) -> register-output;
}",
        with : { "component:pulumi-wasm/output-interface@0.0.0-DEV" :
        pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface
        } }
    );
}
#[link_section = "pulumi_wasm_provider::gcp"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_GCP: [u8; 45] = *b"{\"version\":\"8.12.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "8.12.1".to_string()
}
