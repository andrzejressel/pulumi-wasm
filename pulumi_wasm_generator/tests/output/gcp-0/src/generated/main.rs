pub mod accesscontextmanager {
    include!("resources/accesscontextmanager/access_level.rs");
    include!("resources/accesscontextmanager/access_level_condition.rs");
    include!("resources/accesscontextmanager/access_levels.rs");
    include!("resources/accesscontextmanager/access_policy.rs");
    include!("resources/accesscontextmanager/access_policy_iam_binding.rs");
    include!("resources/accesscontextmanager/access_policy_iam_member.rs");
    include!("resources/accesscontextmanager/access_policy_iam_policy.rs");
    include!("resources/accesscontextmanager/authorized_orgs_desc.rs");
    include!("resources/accesscontextmanager/egress_policy.rs");
    include!("resources/accesscontextmanager/gcp_user_access_binding.rs");
    include!("resources/accesscontextmanager/ingress_policy.rs");
    include!("resources/accesscontextmanager/service_perimeter.rs");
    include!(
        "resources/accesscontextmanager/service_perimeter_dry_run_egress_policy.rs"
    );
    include!(
        "resources/accesscontextmanager/service_perimeter_dry_run_ingress_policy.rs"
    );
    include!("resources/accesscontextmanager/service_perimeter_dry_run_resource.rs");
    include!("resources/accesscontextmanager/service_perimeter_egress_policy.rs");
    include!("resources/accesscontextmanager/service_perimeter_ingress_policy.rs");
    include!("resources/accesscontextmanager/service_perimeter_resource.rs");
    include!("resources/accesscontextmanager/service_perimeters.rs");
}
pub mod activedirectory {
    include!("resources/activedirectory/domain.rs");
    include!("resources/activedirectory/domain_trust.rs");
    include!("resources/activedirectory/peering.rs");
}
pub mod alloydb {
    include!("resources/alloydb/backup.rs");
    include!("resources/alloydb/cluster.rs");
    include!("resources/alloydb/instance.rs");
    include!("resources/alloydb/user.rs");
}
pub mod apigateway {
    include!("resources/apigateway/api.rs");
    include!("resources/apigateway/api_config.rs");
    include!("resources/apigateway/api_config_iam_binding.rs");
    include!("resources/apigateway/api_config_iam_member.rs");
    include!("resources/apigateway/api_config_iam_policy.rs");
    include!("resources/apigateway/api_iam_binding.rs");
    include!("resources/apigateway/api_iam_member.rs");
    include!("resources/apigateway/api_iam_policy.rs");
    include!("resources/apigateway/gateway.rs");
    include!("resources/apigateway/gateway_iam_binding.rs");
    include!("resources/apigateway/gateway_iam_member.rs");
    include!("resources/apigateway/gateway_iam_policy.rs");
}
pub mod apigee {
    include!("resources/apigee/addons_config.rs");
    include!("resources/apigee/api.rs");
    include!("resources/apigee/app_group.rs");
    include!("resources/apigee/developer.rs");
    include!("resources/apigee/endpoint_attachment.rs");
    include!("resources/apigee/env_group.rs");
    include!("resources/apigee/env_group_attachment.rs");
    include!("resources/apigee/env_keystore.rs");
    include!("resources/apigee/env_references.rs");
    include!("resources/apigee/environment.rs");
    include!("resources/apigee/environment_iam_binding.rs");
    include!("resources/apigee/environment_iam_member.rs");
    include!("resources/apigee/environment_iam_policy.rs");
    include!("resources/apigee/environment_keyvaluemaps.rs");
    include!("resources/apigee/environment_keyvaluemaps_entries.rs");
    include!("resources/apigee/flowhook.rs");
    include!("resources/apigee/instance.rs");
    include!("resources/apigee/instance_attachment.rs");
    include!("resources/apigee/keystores_aliases_key_cert_file.rs");
    include!("resources/apigee/keystores_aliases_pkcs_12.rs");
    include!("resources/apigee/keystores_aliases_self_signed_cert.rs");
    include!("resources/apigee/nat_address.rs");
    include!("resources/apigee/organization.rs");
    include!("resources/apigee/sharedflow.rs");
    include!("resources/apigee/sharedflow_deployment.rs");
    include!("resources/apigee/sync_authorization.rs");
    include!("resources/apigee/target_server.rs");
}
pub mod appengine {
    include!("resources/appengine/application.rs");
    include!("resources/appengine/application_url_dispatch_rules.rs");
    include!("resources/appengine/domain_mapping.rs");
    include!("resources/appengine/engine_split_traffic.rs");
    include!("resources/appengine/firewall_rule.rs");
    include!("resources/appengine/flexible_app_version.rs");
    include!("resources/appengine/service_network_settings.rs");
    include!("resources/appengine/standard_app_version.rs");
}
pub mod apphub {
    include!("resources/apphub/application.rs");
    include!("resources/apphub/service.rs");
    include!("resources/apphub/service_project_attachment.rs");
    include!("resources/apphub/workload.rs");
}
pub mod applicationintegration {
    include!("resources/applicationintegration/auth_config.rs");
    include!("resources/applicationintegration/client.rs");
}
pub mod artifactregistry {
    include!("resources/artifactregistry/repository.rs");
    include!("resources/artifactregistry/repository_iam_binding.rs");
    include!("resources/artifactregistry/repository_iam_member.rs");
    include!("resources/artifactregistry/repository_iam_policy.rs");
    include!("resources/artifactregistry/vpcsc_config.rs");
}
pub mod functions {
    pub mod accessapproval {
        include!("functions/accessapproval/get_folder_service_account.rs");
        include!("functions/accessapproval/get_organization_service_account.rs");
        include!("functions/accessapproval/get_project_service_account.rs");
    }
    pub mod accesscontextmanager {
        include!("functions/accesscontextmanager/get_access_policy.rs");
        include!("functions/accesscontextmanager/get_access_policy_iam_policy.rs");
    }
    pub mod alloydb {
        include!("functions/alloydb/get_locations.rs");
        include!("functions/alloydb/get_supported_database_flags.rs");
    }
    pub mod apigateway {
        include!("functions/apigateway/get_api_config_iam_policy.rs");
        include!("functions/apigateway/get_api_iam_policy.rs");
        include!("functions/apigateway/get_gateway_iam_policy.rs");
    }
    pub mod apigee {
        include!("functions/apigee/get_environment_iam_policy.rs");
    }
    pub mod appengine {
        include!("functions/appengine/get_default_service_account.rs");
    }
    pub mod apphub {
        include!("functions/apphub/get_application.rs");
        include!("functions/apphub/get_discovered_service.rs");
        include!("functions/apphub/get_discovered_workload.rs");
    }
    pub mod artifactregistry {
        include!("functions/artifactregistry/get_docker_image.rs");
        include!("functions/artifactregistry/get_locations.rs");
        include!("functions/artifactregistry/get_repository.rs");
        include!("functions/artifactregistry/get_repository_iam_policy.rs");
    }
}
pub mod types {
    pub mod accesscontextmanager {
        include!("types/accesscontextmanager/access_level_basic.rs");
        include!("types/accesscontextmanager/access_level_basic_condition.rs");
        include!(
            "types/accesscontextmanager/access_level_basic_condition_device_policy.rs"
        );
        include!(
            "types/accesscontextmanager/access_level_basic_condition_device_policy_os_constraint.rs"
        );
        include!(
            "types/accesscontextmanager/access_level_basic_condition_vpc_network_source.rs"
        );
        include!(
            "types/accesscontextmanager/access_level_basic_condition_vpc_network_source_vpc_subnetwork.rs"
        );
        include!("types/accesscontextmanager/access_level_condition_device_policy.rs");
        include!(
            "types/accesscontextmanager/access_level_condition_device_policy_os_constraint.rs"
        );
        include!(
            "types/accesscontextmanager/access_level_condition_vpc_network_source.rs"
        );
        include!(
            "types/accesscontextmanager/access_level_condition_vpc_network_source_vpc_subnetwork.rs"
        );
        include!("types/accesscontextmanager/access_level_custom.rs");
        include!("types/accesscontextmanager/access_level_custom_expr.rs");
        include!("types/accesscontextmanager/access_levels_access_level.rs");
        include!("types/accesscontextmanager/access_levels_access_level_basic.rs");
        include!(
            "types/accesscontextmanager/access_levels_access_level_basic_condition.rs"
        );
        include!(
            "types/accesscontextmanager/access_levels_access_level_basic_condition_device_policy.rs"
        );
        include!(
            "types/accesscontextmanager/access_levels_access_level_basic_condition_device_policy_os_constraint.rs"
        );
        include!(
            "types/accesscontextmanager/access_levels_access_level_basic_condition_vpc_network_source.rs"
        );
        include!(
            "types/accesscontextmanager/access_levels_access_level_basic_condition_vpc_network_source_vpc_subnetwork.rs"
        );
        include!("types/accesscontextmanager/access_levels_access_level_custom.rs");
        include!("types/accesscontextmanager/access_levels_access_level_custom_expr.rs");
        include!("types/accesscontextmanager/access_policy_iam_binding_condition.rs");
        include!("types/accesscontextmanager/access_policy_iam_member_condition.rs");
        include!(
            "types/accesscontextmanager/service_perimeter_dry_run_egress_policy_egress_from.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_dry_run_egress_policy_egress_from_source.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_dry_run_egress_policy_egress_to.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_dry_run_egress_policy_egress_to_operation.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_dry_run_egress_policy_egress_to_operation_method_selector.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_dry_run_ingress_policy_ingress_from.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_dry_run_ingress_policy_ingress_from_source.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_dry_run_ingress_policy_ingress_to.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_dry_run_ingress_policy_ingress_to_operation.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_dry_run_ingress_policy_ingress_to_operation_method_selector.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_egress_policy_egress_from.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_egress_policy_egress_from_source.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_egress_policy_egress_to.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_egress_policy_egress_to_operation.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_egress_policy_egress_to_operation_method_selector.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_ingress_policy_ingress_from.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_ingress_policy_ingress_from_source.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_ingress_policy_ingress_to.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_ingress_policy_ingress_to_operation.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_ingress_policy_ingress_to_operation_method_selector.rs"
        );
        include!("types/accesscontextmanager/service_perimeter_spec.rs");
        include!("types/accesscontextmanager/service_perimeter_spec_egress_policy.rs");
        include!(
            "types/accesscontextmanager/service_perimeter_spec_egress_policy_egress_from.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_spec_egress_policy_egress_from_source.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_spec_egress_policy_egress_to.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_spec_egress_policy_egress_to_operation.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_spec_egress_policy_egress_to_operation_method_selector.rs"
        );
        include!("types/accesscontextmanager/service_perimeter_spec_ingress_policy.rs");
        include!(
            "types/accesscontextmanager/service_perimeter_spec_ingress_policy_ingress_from.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_spec_ingress_policy_ingress_from_source.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_spec_ingress_policy_ingress_to.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_spec_ingress_policy_ingress_to_operation.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_spec_ingress_policy_ingress_to_operation_method_selector.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_spec_vpc_accessible_services.rs"
        );
        include!("types/accesscontextmanager/service_perimeter_status.rs");
        include!("types/accesscontextmanager/service_perimeter_status_egress_policy.rs");
        include!(
            "types/accesscontextmanager/service_perimeter_status_egress_policy_egress_from.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_status_egress_policy_egress_from_source.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_status_egress_policy_egress_to.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_status_egress_policy_egress_to_operation.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_status_egress_policy_egress_to_operation_method_selector.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_status_ingress_policy.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_status_ingress_policy_ingress_from.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_status_ingress_policy_ingress_from_source.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_status_ingress_policy_ingress_to.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_status_ingress_policy_ingress_to_operation.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_status_ingress_policy_ingress_to_operation_method_selector.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeter_status_vpc_accessible_services.rs"
        );
        include!("types/accesscontextmanager/service_perimeters_service_perimeter.rs");
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_spec.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_spec_egress_policy.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_spec_egress_policy_egress_from.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_spec_egress_policy_egress_from_source.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_spec_egress_policy_egress_to.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_spec_egress_policy_egress_to_operation.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_spec_egress_policy_egress_to_operation_method_selector.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_spec_ingress_policy.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_spec_ingress_policy_ingress_from.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_spec_ingress_policy_ingress_from_source.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_spec_ingress_policy_ingress_to.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_spec_ingress_policy_ingress_to_operation.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_spec_ingress_policy_ingress_to_operation_method_selector.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_spec_vpc_accessible_services.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_status.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_status_egress_policy.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_status_egress_policy_egress_from.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_status_egress_policy_egress_from_source.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_status_egress_policy_egress_to.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_status_egress_policy_egress_to_operation.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_status_egress_policy_egress_to_operation_method_selector.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_status_ingress_policy.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_status_ingress_policy_ingress_from.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_status_ingress_policy_ingress_from_source.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_status_ingress_policy_ingress_to.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_status_ingress_policy_ingress_to_operation.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_status_ingress_policy_ingress_to_operation_method_selector.rs"
        );
        include!(
            "types/accesscontextmanager/service_perimeters_service_perimeter_status_vpc_accessible_services.rs"
        );
    }
    pub mod alloydb {
        include!("types/alloydb/backup_encryption_config.rs");
        include!("types/alloydb/backup_encryption_info.rs");
        include!("types/alloydb/backup_expiry_quantity.rs");
        include!("types/alloydb/cluster_automated_backup_policy.rs");
        include!("types/alloydb/cluster_automated_backup_policy_encryption_config.rs");
        include!(
            "types/alloydb/cluster_automated_backup_policy_quantity_based_retention.rs"
        );
        include!(
            "types/alloydb/cluster_automated_backup_policy_time_based_retention.rs"
        );
        include!("types/alloydb/cluster_automated_backup_policy_weekly_schedule.rs");
        include!(
            "types/alloydb/cluster_automated_backup_policy_weekly_schedule_start_time.rs"
        );
        include!("types/alloydb/cluster_backup_source.rs");
        include!("types/alloydb/cluster_continuous_backup_config.rs");
        include!("types/alloydb/cluster_continuous_backup_config_encryption_config.rs");
        include!("types/alloydb/cluster_continuous_backup_info.rs");
        include!("types/alloydb/cluster_continuous_backup_info_encryption_info.rs");
        include!("types/alloydb/cluster_encryption_config.rs");
        include!("types/alloydb/cluster_encryption_info.rs");
        include!("types/alloydb/cluster_initial_user.rs");
        include!("types/alloydb/cluster_maintenance_update_policy.rs");
        include!(
            "types/alloydb/cluster_maintenance_update_policy_maintenance_window.rs"
        );
        include!(
            "types/alloydb/cluster_maintenance_update_policy_maintenance_window_start_time.rs"
        );
        include!("types/alloydb/cluster_migration_source.rs");
        include!("types/alloydb/cluster_network_config.rs");
        include!("types/alloydb/cluster_psc_config.rs");
        include!("types/alloydb/cluster_restore_backup_source.rs");
        include!("types/alloydb/cluster_restore_continuous_backup_source.rs");
        include!("types/alloydb/cluster_secondary_config.rs");
        include!("types/alloydb/cluster_trial_metadata.rs");
        include!("types/alloydb/instance_client_connection_config.rs");
        include!("types/alloydb/instance_client_connection_config_ssl_config.rs");
        include!("types/alloydb/instance_machine_config.rs");
        include!("types/alloydb/instance_network_config.rs");
        include!("types/alloydb/instance_network_config_authorized_external_network.rs");
        include!("types/alloydb/instance_observability_config.rs");
        include!("types/alloydb/instance_psc_instance_config.rs");
        include!("types/alloydb/instance_query_insights_config.rs");
        include!("types/alloydb/instance_read_pool_config.rs");
        include!("types/alloydb/get_locations_location.rs");
        include!(
            "types/alloydb/get_supported_database_flags_supported_database_flag.rs"
        );
        include!(
            "types/alloydb/get_supported_database_flags_supported_database_flag_integer_restrictions.rs"
        );
        include!(
            "types/alloydb/get_supported_database_flags_supported_database_flag_string_restrictions.rs"
        );
    }
    pub mod apigateway {
        include!("types/apigateway/api_config_gateway_config.rs");
        include!("types/apigateway/api_config_gateway_config_backend_config.rs");
        include!("types/apigateway/api_config_grpc_service.rs");
        include!("types/apigateway/api_config_grpc_service_file_descriptor_set.rs");
        include!("types/apigateway/api_config_grpc_service_source.rs");
        include!("types/apigateway/api_config_iam_binding_condition.rs");
        include!("types/apigateway/api_config_iam_member_condition.rs");
        include!("types/apigateway/api_config_managed_service_config.rs");
        include!("types/apigateway/api_config_openapi_document.rs");
        include!("types/apigateway/api_config_openapi_document_document.rs");
        include!("types/apigateway/api_iam_binding_condition.rs");
        include!("types/apigateway/api_iam_member_condition.rs");
        include!("types/apigateway/gateway_iam_binding_condition.rs");
        include!("types/apigateway/gateway_iam_member_condition.rs");
    }
    pub mod apigee {
        include!("types/apigee/addons_config_addons_config.rs");
        include!("types/apigee/addons_config_addons_config_advanced_api_ops_config.rs");
        include!("types/apigee/addons_config_addons_config_api_security_config.rs");
        include!(
            "types/apigee/addons_config_addons_config_connectors_platform_config.rs"
        );
        include!("types/apigee/addons_config_addons_config_integration_config.rs");
        include!("types/apigee/addons_config_addons_config_monetization_config.rs");
        include!("types/apigee/api_meta_data.rs");
        include!("types/apigee/app_group_attribute.rs");
        include!("types/apigee/developer_attribute.rs");
        include!("types/apigee/environment_iam_binding_condition.rs");
        include!("types/apigee/environment_iam_member_condition.rs");
        include!("types/apigee/environment_node_config.rs");
        include!("types/apigee/keystores_aliases_key_cert_file_certs_info.rs");
        include!("types/apigee/keystores_aliases_key_cert_file_certs_info_cert_info.rs");
        include!("types/apigee/keystores_aliases_pkcs_12_certs_info.rs");
        include!("types/apigee/keystores_aliases_pkcs_12_certs_info_cert_info.rs");
        include!("types/apigee/keystores_aliases_self_signed_cert_certs_info.rs");
        include!(
            "types/apigee/keystores_aliases_self_signed_cert_certs_info_cert_info.rs"
        );
        include!("types/apigee/keystores_aliases_self_signed_cert_subject.rs");
        include!(
            "types/apigee/keystores_aliases_self_signed_cert_subject_alternative_dns_names.rs"
        );
        include!("types/apigee/organization_properties.rs");
        include!("types/apigee/organization_properties_property.rs");
        include!("types/apigee/sharedflow_meta_data.rs");
        include!("types/apigee/target_server_s_sl_info.rs");
        include!("types/apigee/target_server_s_sl_info_common_name.rs");
    }
    pub mod appengine {
        include!("types/appengine/application_feature_settings.rs");
        include!("types/appengine/application_iap.rs");
        include!("types/appengine/application_url_dispatch_rule.rs");
        include!("types/appengine/application_url_dispatch_rules_dispatch_rule.rs");
        include!("types/appengine/domain_mapping_resource_record.rs");
        include!("types/appengine/domain_mapping_ssl_settings.rs");
        include!("types/appengine/engine_split_traffic_split.rs");
        include!("types/appengine/flexible_app_version_api_config.rs");
        include!("types/appengine/flexible_app_version_automatic_scaling.rs");
        include!(
            "types/appengine/flexible_app_version_automatic_scaling_cpu_utilization.rs"
        );
        include!(
            "types/appengine/flexible_app_version_automatic_scaling_disk_utilization.rs"
        );
        include!(
            "types/appengine/flexible_app_version_automatic_scaling_network_utilization.rs"
        );
        include!(
            "types/appengine/flexible_app_version_automatic_scaling_request_utilization.rs"
        );
        include!("types/appengine/flexible_app_version_deployment.rs");
        include!(
            "types/appengine/flexible_app_version_deployment_cloud_build_options.rs"
        );
        include!("types/appengine/flexible_app_version_deployment_container.rs");
        include!("types/appengine/flexible_app_version_deployment_file.rs");
        include!("types/appengine/flexible_app_version_deployment_zip.rs");
        include!("types/appengine/flexible_app_version_endpoints_api_service.rs");
        include!("types/appengine/flexible_app_version_entrypoint.rs");
        include!("types/appengine/flexible_app_version_flexible_runtime_settings.rs");
        include!("types/appengine/flexible_app_version_handler.rs");
        include!("types/appengine/flexible_app_version_handler_script.rs");
        include!("types/appengine/flexible_app_version_handler_static_files.rs");
        include!("types/appengine/flexible_app_version_liveness_check.rs");
        include!("types/appengine/flexible_app_version_manual_scaling.rs");
        include!("types/appengine/flexible_app_version_network.rs");
        include!("types/appengine/flexible_app_version_readiness_check.rs");
        include!("types/appengine/flexible_app_version_resources.rs");
        include!("types/appengine/flexible_app_version_resources_volume.rs");
        include!("types/appengine/flexible_app_version_vpc_access_connector.rs");
        include!("types/appengine/service_network_settings_network_settings.rs");
        include!("types/appengine/standard_app_version_automatic_scaling.rs");
        include!(
            "types/appengine/standard_app_version_automatic_scaling_standard_scheduler_settings.rs"
        );
        include!("types/appengine/standard_app_version_basic_scaling.rs");
        include!("types/appengine/standard_app_version_deployment.rs");
        include!("types/appengine/standard_app_version_deployment_file.rs");
        include!("types/appengine/standard_app_version_deployment_zip.rs");
        include!("types/appengine/standard_app_version_entrypoint.rs");
        include!("types/appengine/standard_app_version_handler.rs");
        include!("types/appengine/standard_app_version_handler_script.rs");
        include!("types/appengine/standard_app_version_handler_static_files.rs");
        include!("types/appengine/standard_app_version_library.rs");
        include!("types/appengine/standard_app_version_manual_scaling.rs");
        include!("types/appengine/standard_app_version_vpc_access_connector.rs");
    }
    pub mod apphub {
        include!("types/apphub/application_attributes.rs");
        include!("types/apphub/application_attributes_business_owner.rs");
        include!("types/apphub/application_attributes_criticality.rs");
        include!("types/apphub/application_attributes_developer_owner.rs");
        include!("types/apphub/application_attributes_environment.rs");
        include!("types/apphub/application_attributes_operator_owner.rs");
        include!("types/apphub/application_scope.rs");
        include!("types/apphub/service_attributes.rs");
        include!("types/apphub/service_attributes_business_owner.rs");
        include!("types/apphub/service_attributes_criticality.rs");
        include!("types/apphub/service_attributes_developer_owner.rs");
        include!("types/apphub/service_attributes_environment.rs");
        include!("types/apphub/service_attributes_operator_owner.rs");
        include!("types/apphub/service_service_property.rs");
        include!("types/apphub/service_service_reference.rs");
        include!("types/apphub/workload_attributes.rs");
        include!("types/apphub/workload_attributes_business_owner.rs");
        include!("types/apphub/workload_attributes_criticality.rs");
        include!("types/apphub/workload_attributes_developer_owner.rs");
        include!("types/apphub/workload_attributes_environment.rs");
        include!("types/apphub/workload_attributes_operator_owner.rs");
        include!("types/apphub/workload_workload_property.rs");
        include!("types/apphub/workload_workload_reference.rs");
        include!("types/apphub/get_application_attribute.rs");
        include!("types/apphub/get_application_attribute_business_owner.rs");
        include!("types/apphub/get_application_attribute_criticality.rs");
        include!("types/apphub/get_application_attribute_developer_owner.rs");
        include!("types/apphub/get_application_attribute_environment.rs");
        include!("types/apphub/get_application_attribute_operator_owner.rs");
        include!("types/apphub/get_application_scope.rs");
        include!("types/apphub/get_discovered_service_service_property.rs");
        include!("types/apphub/get_discovered_service_service_reference.rs");
        include!("types/apphub/get_discovered_workload_workload_property.rs");
        include!("types/apphub/get_discovered_workload_workload_reference.rs");
    }
    pub mod applicationintegration {
        include!("types/applicationintegration/auth_config_client_certificate.rs");
        include!("types/applicationintegration/auth_config_decrypted_credential.rs");
        include!(
            "types/applicationintegration/auth_config_decrypted_credential_auth_token.rs"
        );
        include!("types/applicationintegration/auth_config_decrypted_credential_jwt.rs");
        include!(
            "types/applicationintegration/auth_config_decrypted_credential_oauth_2_authorization_code.rs"
        );
        include!(
            "types/applicationintegration/auth_config_decrypted_credential_oauth_2_client_credentials.rs"
        );
        include!(
            "types/applicationintegration/auth_config_decrypted_credential_oauth_2_client_credentials_token_params.rs"
        );
        include!(
            "types/applicationintegration/auth_config_decrypted_credential_oauth_2_client_credentials_token_params_entry.rs"
        );
        include!(
            "types/applicationintegration/auth_config_decrypted_credential_oauth_2_client_credentials_token_params_entry_key.rs"
        );
        include!(
            "types/applicationintegration/auth_config_decrypted_credential_oauth_2_client_credentials_token_params_entry_key_literal_value.rs"
        );
        include!(
            "types/applicationintegration/auth_config_decrypted_credential_oauth_2_client_credentials_token_params_entry_value.rs"
        );
        include!(
            "types/applicationintegration/auth_config_decrypted_credential_oauth_2_client_credentials_token_params_entry_value_literal_value.rs"
        );
        include!(
            "types/applicationintegration/auth_config_decrypted_credential_oidc_token.rs"
        );
        include!(
            "types/applicationintegration/auth_config_decrypted_credential_service_account_credentials.rs"
        );
        include!(
            "types/applicationintegration/auth_config_decrypted_credential_username_and_password.rs"
        );
        include!("types/applicationintegration/client_cloud_kms_config.rs");
    }
    pub mod artifactregistry {
        include!("types/artifactregistry/repository_cleanup_policy.rs");
        include!("types/artifactregistry/repository_cleanup_policy_condition.rs");
        include!(
            "types/artifactregistry/repository_cleanup_policy_most_recent_versions.rs"
        );
        include!("types/artifactregistry/repository_docker_config.rs");
        include!("types/artifactregistry/repository_iam_binding_condition.rs");
        include!("types/artifactregistry/repository_iam_member_condition.rs");
        include!("types/artifactregistry/repository_maven_config.rs");
        include!("types/artifactregistry/repository_remote_repository_config.rs");
        include!(
            "types/artifactregistry/repository_remote_repository_config_apt_repository.rs"
        );
        include!(
            "types/artifactregistry/repository_remote_repository_config_apt_repository_public_repository.rs"
        );
        include!(
            "types/artifactregistry/repository_remote_repository_config_common_repository.rs"
        );
        include!(
            "types/artifactregistry/repository_remote_repository_config_docker_repository.rs"
        );
        include!(
            "types/artifactregistry/repository_remote_repository_config_docker_repository_custom_repository.rs"
        );
        include!(
            "types/artifactregistry/repository_remote_repository_config_maven_repository.rs"
        );
        include!(
            "types/artifactregistry/repository_remote_repository_config_maven_repository_custom_repository.rs"
        );
        include!(
            "types/artifactregistry/repository_remote_repository_config_npm_repository.rs"
        );
        include!(
            "types/artifactregistry/repository_remote_repository_config_npm_repository_custom_repository.rs"
        );
        include!(
            "types/artifactregistry/repository_remote_repository_config_python_repository.rs"
        );
        include!(
            "types/artifactregistry/repository_remote_repository_config_python_repository_custom_repository.rs"
        );
        include!(
            "types/artifactregistry/repository_remote_repository_config_upstream_credentials.rs"
        );
        include!(
            "types/artifactregistry/repository_remote_repository_config_upstream_credentials_username_password_credentials.rs"
        );
        include!(
            "types/artifactregistry/repository_remote_repository_config_yum_repository.rs"
        );
        include!(
            "types/artifactregistry/repository_remote_repository_config_yum_repository_public_repository.rs"
        );
        include!("types/artifactregistry/repository_virtual_repository_config.rs");
        include!(
            "types/artifactregistry/repository_virtual_repository_config_upstream_policy.rs"
        );
        include!("types/artifactregistry/get_repository_cleanup_policy.rs");
        include!("types/artifactregistry/get_repository_cleanup_policy_condition.rs");
        include!(
            "types/artifactregistry/get_repository_cleanup_policy_most_recent_version.rs"
        );
        include!("types/artifactregistry/get_repository_docker_config.rs");
        include!("types/artifactregistry/get_repository_maven_config.rs");
        include!("types/artifactregistry/get_repository_remote_repository_config.rs");
        include!(
            "types/artifactregistry/get_repository_remote_repository_config_apt_repository.rs"
        );
        include!(
            "types/artifactregistry/get_repository_remote_repository_config_apt_repository_public_repository.rs"
        );
        include!(
            "types/artifactregistry/get_repository_remote_repository_config_common_repository.rs"
        );
        include!(
            "types/artifactregistry/get_repository_remote_repository_config_docker_repository.rs"
        );
        include!(
            "types/artifactregistry/get_repository_remote_repository_config_docker_repository_custom_repository.rs"
        );
        include!(
            "types/artifactregistry/get_repository_remote_repository_config_maven_repository.rs"
        );
        include!(
            "types/artifactregistry/get_repository_remote_repository_config_maven_repository_custom_repository.rs"
        );
        include!(
            "types/artifactregistry/get_repository_remote_repository_config_npm_repository.rs"
        );
        include!(
            "types/artifactregistry/get_repository_remote_repository_config_npm_repository_custom_repository.rs"
        );
        include!(
            "types/artifactregistry/get_repository_remote_repository_config_python_repository.rs"
        );
        include!(
            "types/artifactregistry/get_repository_remote_repository_config_python_repository_custom_repository.rs"
        );
        include!(
            "types/artifactregistry/get_repository_remote_repository_config_upstream_credential.rs"
        );
        include!(
            "types/artifactregistry/get_repository_remote_repository_config_upstream_credential_username_password_credential.rs"
        );
        include!(
            "types/artifactregistry/get_repository_remote_repository_config_yum_repository.rs"
        );
        include!(
            "types/artifactregistry/get_repository_remote_repository_config_yum_repository_public_repository.rs"
        );
        include!("types/artifactregistry/get_repository_virtual_repository_config.rs");
        include!(
            "types/artifactregistry/get_repository_virtual_repository_config_upstream_policy.rs"
        );
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
#[link_section = "pulumi_wasm_provider::gcp"]
#[no_mangle]
pub static PULUMI_WASM_PROVIDER_gcp: [u8; 6] = *b"8.12.1";
