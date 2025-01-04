pub mod ec2clientvpn {
    include!("resources/ec2clientvpn/authorization_rule.rs");
    include!("resources/ec2clientvpn/endpoint.rs");
    include!("resources/ec2clientvpn/network_association.rs");
    include!("resources/ec2clientvpn/route.rs");
}
pub mod ec2transitgateway {
    include!("resources/ec2transitgateway/connect.rs");
    include!("resources/ec2transitgateway/connect_peer.rs");
    include!("resources/ec2transitgateway/default_route_table_association.rs");
    include!("resources/ec2transitgateway/default_route_table_propagation.rs");
    include!("resources/ec2transitgateway/instance_connect_endpoint.rs");
    include!("resources/ec2transitgateway/instance_state.rs");
    include!("resources/ec2transitgateway/multicast_domain.rs");
    include!("resources/ec2transitgateway/multicast_domain_association.rs");
    include!("resources/ec2transitgateway/multicast_group_member.rs");
    include!("resources/ec2transitgateway/multicast_group_source.rs");
    include!("resources/ec2transitgateway/peering_attachment.rs");
    include!("resources/ec2transitgateway/peering_attachment_accepter.rs");
    include!("resources/ec2transitgateway/policy_table.rs");
    include!("resources/ec2transitgateway/policy_table_association.rs");
    include!("resources/ec2transitgateway/prefix_list_reference.rs");
    include!("resources/ec2transitgateway/route.rs");
    include!("resources/ec2transitgateway/route_table.rs");
    include!("resources/ec2transitgateway/route_table_association.rs");
    include!("resources/ec2transitgateway/route_table_propagation.rs");
    include!("resources/ec2transitgateway/transit_gateway.rs");
    include!("resources/ec2transitgateway/vpc_attachment.rs");
    include!("resources/ec2transitgateway/vpc_attachment_accepter.rs");
}
pub mod ecr {
    include!("resources/ecr/account_setting.rs");
    include!("resources/ecr/lifecycle_policy.rs");
    include!("resources/ecr/pull_through_cache_rule.rs");
    include!("resources/ecr/registry_policy.rs");
    include!("resources/ecr/registry_scanning_configuration.rs");
    include!("resources/ecr/replication_configuration.rs");
    include!("resources/ecr/repository.rs");
    include!("resources/ecr/repository_creation_template.rs");
    include!("resources/ecr/repository_policy.rs");
}
pub mod ecrpublic {
    include!("resources/ecrpublic/repository.rs");
    include!("resources/ecrpublic/repository_policy.rs");
}
pub mod ecs {
    include!("resources/ecs/account_setting_default.rs");
    include!("resources/ecs/capacity_provider.rs");
    include!("resources/ecs/cluster.rs");
    include!("resources/ecs/cluster_capacity_providers.rs");
    include!("resources/ecs/service.rs");
    include!("resources/ecs/tag.rs");
    include!("resources/ecs/task_definition.rs");
    include!("resources/ecs/task_set.rs");
}
pub mod efs {
    include!("resources/efs/access_point.rs");
    include!("resources/efs/backup_policy.rs");
    include!("resources/efs/file_system.rs");
    include!("resources/efs/file_system_policy.rs");
    include!("resources/efs/mount_target.rs");
    include!("resources/efs/replication_configuration.rs");
}
pub mod eks {
    include!("resources/eks/access_entry.rs");
    include!("resources/eks/access_policy_association.rs");
    include!("resources/eks/addon.rs");
    include!("resources/eks/cluster.rs");
    include!("resources/eks/fargate_profile.rs");
    include!("resources/eks/identity_provider_config.rs");
    include!("resources/eks/node_group.rs");
    include!("resources/eks/pod_identity_association.rs");
}
pub mod elasticache {
    include!("resources/elasticache/cluster.rs");
    include!("resources/elasticache/global_replication_group.rs");
    include!("resources/elasticache/parameter_group.rs");
    include!("resources/elasticache/replication_group.rs");
    include!("resources/elasticache/reserved_cache_node.rs");
    include!("resources/elasticache/serverless_cache.rs");
    include!("resources/elasticache/subnet_group.rs");
    include!("resources/elasticache/user.rs");
    include!("resources/elasticache/user_group.rs");
    include!("resources/elasticache/user_group_association.rs");
}
pub mod elasticbeanstalk {
    include!("resources/elasticbeanstalk/application.rs");
    include!("resources/elasticbeanstalk/application_version.rs");
    include!("resources/elasticbeanstalk/configuration_template.rs");
    include!("resources/elasticbeanstalk/environment.rs");
}
pub mod elasticsearch {
    include!("resources/elasticsearch/domain.rs");
    include!("resources/elasticsearch/domain_policy.rs");
    include!("resources/elasticsearch/domain_saml_options.rs");
    include!("resources/elasticsearch/vpc_endpoint.rs");
}
pub mod functions {
    pub mod ec2clientvpn {
        include!("functions/ec2clientvpn/get_endpoint.rs");
    }
    pub mod ec2transitgateway {
        include!("functions/ec2transitgateway/get_attachment.rs");
        include!("functions/ec2transitgateway/get_attachments.rs");
        include!("functions/ec2transitgateway/get_connect.rs");
        include!("functions/ec2transitgateway/get_connect_peer.rs");
        include!("functions/ec2transitgateway/get_direct_connect_gateway_attachment.rs");
        include!("functions/ec2transitgateway/get_multicast_domain.rs");
        include!("functions/ec2transitgateway/get_peering_attachment.rs");
        include!("functions/ec2transitgateway/get_peering_attachments.rs");
        include!("functions/ec2transitgateway/get_route_table.rs");
        include!("functions/ec2transitgateway/get_route_table_associations.rs");
        include!("functions/ec2transitgateway/get_route_table_propagations.rs");
        include!("functions/ec2transitgateway/get_route_table_routes.rs");
        include!("functions/ec2transitgateway/get_transit_gateway.rs");
        include!("functions/ec2transitgateway/get_vpc_attachment.rs");
        include!("functions/ec2transitgateway/get_vpc_attachments.rs");
        include!("functions/ec2transitgateway/get_vpn_attachment.rs");
    }
    pub mod ecr {
        include!("functions/ecr/get_authorization_token.rs");
        include!("functions/ecr/get_credentials.rs");
        include!("functions/ecr/get_image.rs");
        include!("functions/ecr/get_lifecycle_policy_document.rs");
        include!("functions/ecr/get_pull_through_cache_rule.rs");
        include!("functions/ecr/get_repositories.rs");
        include!("functions/ecr/get_repository.rs");
        include!("functions/ecr/get_repository_creation_template.rs");
    }
    pub mod ecrpublic {
        include!("functions/ecrpublic/get_authorization_token.rs");
    }
    pub mod ecs {
        include!("functions/ecs/get_cluster.rs");
        include!("functions/ecs/get_container_definition.rs");
        include!("functions/ecs/get_service.rs");
        include!("functions/ecs/get_task_definition.rs");
        include!("functions/ecs/get_task_execution.rs");
    }
    pub mod efs {
        include!("functions/efs/get_access_point.rs");
        include!("functions/efs/get_access_points.rs");
        include!("functions/efs/get_file_system.rs");
        include!("functions/efs/get_mount_target.rs");
    }
    pub mod eks {
        include!("functions/eks/get_access_entry.rs");
        include!("functions/eks/get_addon.rs");
        include!("functions/eks/get_addon_version.rs");
        include!("functions/eks/get_cluster.rs");
        include!("functions/eks/get_cluster_auth.rs");
        include!("functions/eks/get_clusters.rs");
        include!("functions/eks/get_node_group.rs");
        include!("functions/eks/get_node_groups.rs");
    }
    pub mod elasticache {
        include!("functions/elasticache/get_cluster.rs");
        include!("functions/elasticache/get_replication_group.rs");
        include!("functions/elasticache/get_reserved_cache_node_offering.rs");
        include!("functions/elasticache/get_serverless_cache.rs");
        include!("functions/elasticache/get_subnet_group.rs");
        include!("functions/elasticache/get_user.rs");
    }
    pub mod elasticbeanstalk {
        include!("functions/elasticbeanstalk/get_application.rs");
        include!("functions/elasticbeanstalk/get_hosted_zone.rs");
        include!("functions/elasticbeanstalk/get_solution_stack.rs");
    }
    pub mod elasticsearch {
        include!("functions/elasticsearch/get_domain.rs");
    }
    include!("functions/get_arn.rs");
    include!("functions/get_availability_zone.rs");
    include!("functions/get_availability_zones.rs");
    include!("functions/get_billing_service_account.rs");
    include!("functions/get_caller_identity.rs");
    include!("functions/get_default_tags.rs");
    include!("functions/get_ip_ranges.rs");
    include!("functions/get_partition.rs");
    include!("functions/get_region.rs");
    include!("functions/get_regions.rs");
    include!("functions/get_service.rs");
    include!("functions/get_service_principal.rs");
}
pub mod types {
    pub mod ec2clientvpn {
        include!("types/ec2clientvpn/endpoint_authentication_option.rs");
        include!("types/ec2clientvpn/endpoint_client_connect_options.rs");
        include!("types/ec2clientvpn/endpoint_client_login_banner_options.rs");
        include!("types/ec2clientvpn/endpoint_connection_log_options.rs");
        include!("types/ec2clientvpn/get_endpoint_authentication_option.rs");
        include!("types/ec2clientvpn/get_endpoint_client_connect_option.rs");
        include!("types/ec2clientvpn/get_endpoint_client_login_banner_option.rs");
        include!("types/ec2clientvpn/get_endpoint_connection_log_option.rs");
        include!("types/ec2clientvpn/get_endpoint_filter.rs");
    }
    pub mod ec2transitgateway {
        include!("types/ec2transitgateway/default_route_table_association_timeouts.rs");
        include!("types/ec2transitgateway/default_route_table_propagation_timeouts.rs");
        include!("types/ec2transitgateway/instance_connect_endpoint_timeouts.rs");
        include!("types/ec2transitgateway/peering_attachment_options.rs");
        include!("types/ec2transitgateway/get_attachment_filter.rs");
        include!("types/ec2transitgateway/get_attachments_filter.rs");
        include!("types/ec2transitgateway/get_connect_filter.rs");
        include!("types/ec2transitgateway/get_connect_peer_filter.rs");
        include!(
            "types/ec2transitgateway/get_direct_connect_gateway_attachment_filter.rs"
        );
        include!("types/ec2transitgateway/get_multicast_domain_association.rs");
        include!("types/ec2transitgateway/get_multicast_domain_filter.rs");
        include!("types/ec2transitgateway/get_multicast_domain_member.rs");
        include!("types/ec2transitgateway/get_multicast_domain_source.rs");
        include!("types/ec2transitgateway/get_peering_attachment_filter.rs");
        include!("types/ec2transitgateway/get_peering_attachments_filter.rs");
        include!("types/ec2transitgateway/get_route_table_associations_filter.rs");
        include!("types/ec2transitgateway/get_route_table_filter.rs");
        include!("types/ec2transitgateway/get_route_table_propagations_filter.rs");
        include!("types/ec2transitgateway/get_route_table_routes_filter.rs");
        include!("types/ec2transitgateway/get_route_table_routes_route.rs");
        include!("types/ec2transitgateway/get_transit_gateway_filter.rs");
        include!("types/ec2transitgateway/get_vpc_attachment_filter.rs");
        include!("types/ec2transitgateway/get_vpc_attachments_filter.rs");
        include!("types/ec2transitgateway/get_vpn_attachment_filter.rs");
    }
    pub mod ecr {
        include!("types/ecr/registry_scanning_configuration_rule.rs");
        include!("types/ecr/registry_scanning_configuration_rule_repository_filter.rs");
        include!("types/ecr/replication_configuration_replication_configuration.rs");
        include!(
            "types/ecr/replication_configuration_replication_configuration_rule.rs"
        );
        include!(
            "types/ecr/replication_configuration_replication_configuration_rule_destination.rs"
        );
        include!(
            "types/ecr/replication_configuration_replication_configuration_rule_repository_filter.rs"
        );
        include!("types/ecr/repository_creation_template_encryption_configuration.rs");
        include!("types/ecr/repository_encryption_configuration.rs");
        include!("types/ecr/repository_image_scanning_configuration.rs");
        include!("types/ecr/get_lifecycle_policy_document_rule.rs");
        include!("types/ecr/get_lifecycle_policy_document_rule_action.rs");
        include!("types/ecr/get_lifecycle_policy_document_rule_selection.rs");
        include!(
            "types/ecr/get_repository_creation_template_encryption_configuration.rs"
        );
        include!("types/ecr/get_repository_encryption_configuration.rs");
        include!("types/ecr/get_repository_image_scanning_configuration.rs");
    }
    pub mod ecrpublic {
        include!("types/ecrpublic/repository_catalog_data.rs");
    }
    pub mod ecs {
        include!("types/ecs/capacity_provider_auto_scaling_group_provider.rs");
        include!(
            "types/ecs/capacity_provider_auto_scaling_group_provider_managed_scaling.rs"
        );
        include!(
            "types/ecs/cluster_capacity_providers_default_capacity_provider_strategy.rs"
        );
        include!("types/ecs/cluster_configuration.rs");
        include!("types/ecs/cluster_configuration_execute_command_configuration.rs");
        include!(
            "types/ecs/cluster_configuration_execute_command_configuration_log_configuration.rs"
        );
        include!("types/ecs/cluster_configuration_managed_storage_configuration.rs");
        include!("types/ecs/cluster_service_connect_defaults.rs");
        include!("types/ecs/cluster_setting.rs");
        include!("types/ecs/service_alarms.rs");
        include!("types/ecs/service_capacity_provider_strategy.rs");
        include!("types/ecs/service_deployment_circuit_breaker.rs");
        include!("types/ecs/service_deployment_controller.rs");
        include!("types/ecs/service_load_balancer.rs");
        include!("types/ecs/service_network_configuration.rs");
        include!("types/ecs/service_ordered_placement_strategy.rs");
        include!("types/ecs/service_placement_constraint.rs");
        include!("types/ecs/service_service_connect_configuration.rs");
        include!("types/ecs/service_service_connect_configuration_log_configuration.rs");
        include!(
            "types/ecs/service_service_connect_configuration_log_configuration_secret_option.rs"
        );
        include!("types/ecs/service_service_connect_configuration_service.rs");
        include!(
            "types/ecs/service_service_connect_configuration_service_client_alias.rs"
        );
        include!("types/ecs/service_service_connect_configuration_service_timeout.rs");
        include!("types/ecs/service_service_connect_configuration_service_tls.rs");
        include!(
            "types/ecs/service_service_connect_configuration_service_tls_issuer_cert_authority.rs"
        );
        include!("types/ecs/service_service_registries.rs");
        include!("types/ecs/service_volume_configuration.rs");
        include!("types/ecs/service_volume_configuration_managed_ebs_volume.rs");
        include!(
            "types/ecs/service_volume_configuration_managed_ebs_volume_tag_specification.rs"
        );
        include!("types/ecs/service_vpc_lattice_configuration.rs");
        include!("types/ecs/task_definition_ephemeral_storage.rs");
        include!("types/ecs/task_definition_inference_accelerator.rs");
        include!("types/ecs/task_definition_placement_constraint.rs");
        include!("types/ecs/task_definition_proxy_configuration.rs");
        include!("types/ecs/task_definition_runtime_platform.rs");
        include!("types/ecs/task_definition_volume.rs");
        include!("types/ecs/task_definition_volume_docker_volume_configuration.rs");
        include!("types/ecs/task_definition_volume_efs_volume_configuration.rs");
        include!(
            "types/ecs/task_definition_volume_efs_volume_configuration_authorization_config.rs"
        );
        include!(
            "types/ecs/task_definition_volume_fsx_windows_file_server_volume_configuration.rs"
        );
        include!(
            "types/ecs/task_definition_volume_fsx_windows_file_server_volume_configuration_authorization_config.rs"
        );
        include!("types/ecs/task_set_capacity_provider_strategy.rs");
        include!("types/ecs/task_set_load_balancer.rs");
        include!("types/ecs/task_set_network_configuration.rs");
        include!("types/ecs/task_set_scale.rs");
        include!("types/ecs/task_set_service_registries.rs");
        include!("types/ecs/get_cluster_service_connect_default.rs");
        include!("types/ecs/get_cluster_setting.rs");
        include!("types/ecs/get_task_execution_capacity_provider_strategy.rs");
        include!("types/ecs/get_task_execution_network_configuration.rs");
        include!("types/ecs/get_task_execution_overrides.rs");
        include!("types/ecs/get_task_execution_overrides_container_override.rs");
        include!(
            "types/ecs/get_task_execution_overrides_container_override_environment.rs"
        );
        include!(
            "types/ecs/get_task_execution_overrides_container_override_resource_requirement.rs"
        );
        include!(
            "types/ecs/get_task_execution_overrides_inference_accelerator_override.rs"
        );
        include!("types/ecs/get_task_execution_placement_constraint.rs");
        include!("types/ecs/get_task_execution_placement_strategy.rs");
    }
    pub mod efs {
        include!("types/efs/access_point_posix_user.rs");
        include!("types/efs/access_point_root_directory.rs");
        include!("types/efs/access_point_root_directory_creation_info.rs");
        include!("types/efs/backup_policy_backup_policy.rs");
        include!("types/efs/file_system_lifecycle_policy.rs");
        include!("types/efs/file_system_protection.rs");
        include!("types/efs/file_system_size_in_byte.rs");
        include!("types/efs/replication_configuration_destination.rs");
        include!("types/efs/get_access_point_posix_user.rs");
        include!("types/efs/get_access_point_root_directory.rs");
        include!("types/efs/get_access_point_root_directory_creation_info.rs");
        include!("types/efs/get_file_system_lifecycle_policy.rs");
        include!("types/efs/get_file_system_protection.rs");
    }
    pub mod eks {
        include!("types/eks/access_policy_association_access_scope.rs");
        include!("types/eks/addon_pod_identity_association.rs");
        include!("types/eks/cluster_access_config.rs");
        include!("types/eks/cluster_certificate_authority.rs");
        include!("types/eks/cluster_compute_config.rs");
        include!("types/eks/cluster_encryption_config.rs");
        include!("types/eks/cluster_encryption_config_provider.rs");
        include!("types/eks/cluster_identity.rs");
        include!("types/eks/cluster_identity_oidc.rs");
        include!("types/eks/cluster_kubernetes_network_config.rs");
        include!(
            "types/eks/cluster_kubernetes_network_config_elastic_load_balancing.rs"
        );
        include!("types/eks/cluster_outpost_config.rs");
        include!("types/eks/cluster_outpost_config_control_plane_placement.rs");
        include!("types/eks/cluster_remote_network_config.rs");
        include!("types/eks/cluster_remote_network_config_remote_node_networks.rs");
        include!("types/eks/cluster_remote_network_config_remote_pod_networks.rs");
        include!("types/eks/cluster_storage_config.rs");
        include!("types/eks/cluster_storage_config_block_storage.rs");
        include!("types/eks/cluster_upgrade_policy.rs");
        include!("types/eks/cluster_vpc_config.rs");
        include!("types/eks/cluster_zonal_shift_config.rs");
        include!("types/eks/fargate_profile_selector.rs");
        include!("types/eks/identity_provider_config_oidc.rs");
        include!("types/eks/node_group_launch_template.rs");
        include!("types/eks/node_group_remote_access.rs");
        include!("types/eks/node_group_resource.rs");
        include!("types/eks/node_group_resource_autoscaling_group.rs");
        include!("types/eks/node_group_scaling_config.rs");
        include!("types/eks/node_group_taint.rs");
        include!("types/eks/node_group_update_config.rs");
        include!("types/eks/get_addon_pod_identity_association.rs");
        include!("types/eks/get_cluster_access_config.rs");
        include!("types/eks/get_cluster_certificate_authority.rs");
        include!("types/eks/get_cluster_compute_config.rs");
        include!("types/eks/get_cluster_identity.rs");
        include!("types/eks/get_cluster_identity_oidc.rs");
        include!("types/eks/get_cluster_kubernetes_network_config.rs");
        include!(
            "types/eks/get_cluster_kubernetes_network_config_elastic_load_balancing.rs"
        );
        include!("types/eks/get_cluster_outpost_config.rs");
        include!("types/eks/get_cluster_outpost_config_control_plane_placement.rs");
        include!("types/eks/get_cluster_remote_network_config.rs");
        include!("types/eks/get_cluster_remote_network_config_remote_node_network.rs");
        include!("types/eks/get_cluster_remote_network_config_remote_pod_network.rs");
        include!("types/eks/get_cluster_storage_config.rs");
        include!("types/eks/get_cluster_storage_config_block_storage.rs");
        include!("types/eks/get_cluster_upgrade_policy.rs");
        include!("types/eks/get_cluster_vpc_config.rs");
        include!("types/eks/get_cluster_zonal_shift_config.rs");
        include!("types/eks/get_node_group_launch_template.rs");
        include!("types/eks/get_node_group_remote_access.rs");
        include!("types/eks/get_node_group_resource.rs");
        include!("types/eks/get_node_group_resource_autoscaling_group.rs");
        include!("types/eks/get_node_group_scaling_config.rs");
        include!("types/eks/get_node_group_taint.rs");
    }
    pub mod elasticache {
        include!("types/elasticache/cluster_cache_node.rs");
        include!("types/elasticache/cluster_log_delivery_configuration.rs");
        include!("types/elasticache/global_replication_group_global_node_group.rs");
        include!("types/elasticache/parameter_group_parameter.rs");
        include!("types/elasticache/replication_group_log_delivery_configuration.rs");
        include!("types/elasticache/reserved_cache_node_recurring_charge.rs");
        include!("types/elasticache/reserved_cache_node_timeouts.rs");
        include!("types/elasticache/serverless_cache_cache_usage_limits.rs");
        include!(
            "types/elasticache/serverless_cache_cache_usage_limits_data_storage.rs"
        );
        include!(
            "types/elasticache/serverless_cache_cache_usage_limits_ecpu_per_second.rs"
        );
        include!("types/elasticache/serverless_cache_endpoint.rs");
        include!("types/elasticache/serverless_cache_reader_endpoint.rs");
        include!("types/elasticache/serverless_cache_timeouts.rs");
        include!("types/elasticache/user_authentication_mode.rs");
        include!("types/elasticache/get_cluster_cache_node.rs");
        include!("types/elasticache/get_cluster_log_delivery_configuration.rs");
        include!(
            "types/elasticache/get_replication_group_log_delivery_configuration.rs"
        );
        include!("types/elasticache/get_serverless_cache_cache_usage_limits.rs");
        include!(
            "types/elasticache/get_serverless_cache_cache_usage_limits_data_storage.rs"
        );
        include!(
            "types/elasticache/get_serverless_cache_cache_usage_limits_ecpu_per_second.rs"
        );
        include!("types/elasticache/get_serverless_cache_endpoint.rs");
        include!("types/elasticache/get_serverless_cache_reader_endpoint.rs");
        include!("types/elasticache/get_user_authentication_mode.rs");
    }
    pub mod elasticbeanstalk {
        include!("types/elasticbeanstalk/application_appversion_lifecycle.rs");
        include!("types/elasticbeanstalk/configuration_template_setting.rs");
        include!("types/elasticbeanstalk/environment_all_setting.rs");
        include!("types/elasticbeanstalk/environment_setting.rs");
        include!("types/elasticbeanstalk/get_application_appversion_lifecycle.rs");
    }
    pub mod elasticsearch {
        include!("types/elasticsearch/domain_advanced_security_options.rs");
        include!(
            "types/elasticsearch/domain_advanced_security_options_master_user_options.rs"
        );
        include!("types/elasticsearch/domain_auto_tune_options.rs");
        include!("types/elasticsearch/domain_auto_tune_options_maintenance_schedule.rs");
        include!(
            "types/elasticsearch/domain_auto_tune_options_maintenance_schedule_duration.rs"
        );
        include!("types/elasticsearch/domain_cluster_config.rs");
        include!("types/elasticsearch/domain_cluster_config_cold_storage_options.rs");
        include!("types/elasticsearch/domain_cluster_config_zone_awareness_config.rs");
        include!("types/elasticsearch/domain_cognito_options.rs");
        include!("types/elasticsearch/domain_domain_endpoint_options.rs");
        include!("types/elasticsearch/domain_ebs_options.rs");
        include!("types/elasticsearch/domain_encrypt_at_rest.rs");
        include!("types/elasticsearch/domain_log_publishing_option.rs");
        include!("types/elasticsearch/domain_node_to_node_encryption.rs");
        include!("types/elasticsearch/domain_saml_options_saml_options.rs");
        include!("types/elasticsearch/domain_saml_options_saml_options_idp.rs");
        include!("types/elasticsearch/domain_snapshot_options.rs");
        include!("types/elasticsearch/domain_vpc_options.rs");
        include!("types/elasticsearch/vpc_endpoint_vpc_options.rs");
        include!("types/elasticsearch/get_domain_advanced_security_option.rs");
        include!("types/elasticsearch/get_domain_auto_tune_option.rs");
        include!(
            "types/elasticsearch/get_domain_auto_tune_option_maintenance_schedule.rs"
        );
        include!(
            "types/elasticsearch/get_domain_auto_tune_option_maintenance_schedule_duration.rs"
        );
        include!("types/elasticsearch/get_domain_cluster_config.rs");
        include!("types/elasticsearch/get_domain_cluster_config_cold_storage_option.rs");
        include!(
            "types/elasticsearch/get_domain_cluster_config_zone_awareness_config.rs"
        );
        include!("types/elasticsearch/get_domain_cognito_option.rs");
        include!("types/elasticsearch/get_domain_ebs_option.rs");
        include!("types/elasticsearch/get_domain_encryption_at_rest.rs");
        include!("types/elasticsearch/get_domain_log_publishing_option.rs");
        include!("types/elasticsearch/get_domain_node_to_node_encryption.rs");
        include!("types/elasticsearch/get_domain_snapshot_option.rs");
        include!("types/elasticsearch/get_domain_vpc_option.rs");
    }
    include!("types/get_availability_zone_filter.rs");
    include!("types/get_availability_zones_filter.rs");
    include!("types/get_regions_filter.rs");
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-aws {
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
