pub mod elastictranscoder {
    include!("resources/elastictranscoder/pipeline.rs");
    include!("resources/elastictranscoder/preset.rs");
}
pub mod elb {
    include!("resources/elb/app_cookie_stickiness_policy.rs");
    include!("resources/elb/attachment.rs");
    include!("resources/elb/listener_policy.rs");
    include!("resources/elb/load_balancer.rs");
    include!("resources/elb/load_balancer_backend_server_policy.rs");
    include!("resources/elb/load_balancer_cookie_stickiness_policy.rs");
    include!("resources/elb/load_balancer_policy.rs");
    include!("resources/elb/ssl_negotiation_policy.rs");
}
pub mod emr {
    include!("resources/emr/block_public_access_configuration.rs");
    include!("resources/emr/cluster.rs");
    include!("resources/emr/instance_fleet.rs");
    include!("resources/emr/instance_group.rs");
    include!("resources/emr/managed_scaling_policy.rs");
    include!("resources/emr/security_configuration.rs");
    include!("resources/emr/studio.rs");
    include!("resources/emr/studio_session_mapping.rs");
}
pub mod emrcontainers {
    include!("resources/emrcontainers/job_template.rs");
    include!("resources/emrcontainers/virtual_cluster.rs");
}
pub mod emrserverless {
    include!("resources/emrserverless/application.rs");
}
pub mod evidently {
    include!("resources/evidently/feature.rs");
    include!("resources/evidently/launch.rs");
    include!("resources/evidently/project.rs");
    include!("resources/evidently/segment.rs");
}
pub mod finspace {
    include!("resources/finspace/kx_cluster.rs");
    include!("resources/finspace/kx_database.rs");
    include!("resources/finspace/kx_dataview.rs");
    include!("resources/finspace/kx_environment.rs");
    include!("resources/finspace/kx_scaling_group.rs");
    include!("resources/finspace/kx_user.rs");
    include!("resources/finspace/kx_volume.rs");
}
pub mod fis {
    include!("resources/fis/experiment_template.rs");
}
pub mod fms {
    include!("resources/fms/admin_account.rs");
    include!("resources/fms/policy.rs");
    include!("resources/fms/resource_set.rs");
}
pub mod fsx {
    include!("resources/fsx/backup.rs");
    include!("resources/fsx/data_repository_association.rs");
    include!("resources/fsx/file_cache.rs");
    include!("resources/fsx/lustre_file_system.rs");
    include!("resources/fsx/ontap_file_system.rs");
    include!("resources/fsx/ontap_storage_virtual_machine.rs");
    include!("resources/fsx/ontap_volume.rs");
    include!("resources/fsx/open_zfs_file_system.rs");
    include!("resources/fsx/open_zfs_snapshot.rs");
    include!("resources/fsx/open_zfs_volume.rs");
    include!("resources/fsx/windows_file_system.rs");
}
pub mod functions {
    pub mod elb {
        include!("functions/elb/get_hosted_zone_id.rs");
        include!("functions/elb/get_load_balancer.rs");
        include!("functions/elb/get_service_account.rs");
    }
    pub mod emr {
        include!("functions/emr/get_release_labels.rs");
        include!("functions/emr/get_supported_instance_types.rs");
    }
    pub mod emrcontainers {
        include!("functions/emrcontainers/get_virtual_cluster.rs");
    }
    pub mod fsx {
        include!("functions/fsx/get_ontap_file_system.rs");
        include!("functions/fsx/get_ontap_storage_virtual_machine.rs");
        include!("functions/fsx/get_ontap_storage_virtual_machines.rs");
        include!("functions/fsx/get_open_zfs_snapshot.rs");
        include!("functions/fsx/get_windows_file_system.rs");
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
    pub mod elastictranscoder {
        include!("types/elastictranscoder/pipeline_content_config.rs");
        include!("types/elastictranscoder/pipeline_content_config_permission.rs");
        include!("types/elastictranscoder/pipeline_notifications.rs");
        include!("types/elastictranscoder/pipeline_thumbnail_config.rs");
        include!("types/elastictranscoder/pipeline_thumbnail_config_permission.rs");
        include!("types/elastictranscoder/preset_audio.rs");
        include!("types/elastictranscoder/preset_audio_codec_options.rs");
        include!("types/elastictranscoder/preset_thumbnails.rs");
        include!("types/elastictranscoder/preset_video.rs");
        include!("types/elastictranscoder/preset_video_watermark.rs");
    }
    pub mod elb {
        include!("types/elb/load_balancer_access_logs.rs");
        include!("types/elb/load_balancer_health_check.rs");
        include!("types/elb/load_balancer_listener.rs");
        include!("types/elb/load_balancer_policy_policy_attribute.rs");
        include!("types/elb/ssl_negotiation_policy_attribute.rs");
        include!("types/elb/get_load_balancer_access_logs.rs");
        include!("types/elb/get_load_balancer_health_check.rs");
        include!("types/elb/get_load_balancer_listener.rs");
    }
    pub mod emr {
        include!(
            "types/emr/block_public_access_configuration_permitted_public_security_group_rule_range.rs"
        );
        include!("types/emr/cluster_auto_termination_policy.rs");
        include!("types/emr/cluster_bootstrap_action.rs");
        include!("types/emr/cluster_core_instance_fleet.rs");
        include!("types/emr/cluster_core_instance_fleet_instance_type_config.rs");
        include!(
            "types/emr/cluster_core_instance_fleet_instance_type_config_configuration.rs"
        );
        include!(
            "types/emr/cluster_core_instance_fleet_instance_type_config_ebs_config.rs"
        );
        include!("types/emr/cluster_core_instance_fleet_launch_specifications.rs");
        include!(
            "types/emr/cluster_core_instance_fleet_launch_specifications_on_demand_specification.rs"
        );
        include!(
            "types/emr/cluster_core_instance_fleet_launch_specifications_spot_specification.rs"
        );
        include!("types/emr/cluster_core_instance_group.rs");
        include!("types/emr/cluster_core_instance_group_ebs_config.rs");
        include!("types/emr/cluster_ec_2_attributes.rs");
        include!("types/emr/cluster_kerberos_attributes.rs");
        include!("types/emr/cluster_master_instance_fleet.rs");
        include!("types/emr/cluster_master_instance_fleet_instance_type_config.rs");
        include!(
            "types/emr/cluster_master_instance_fleet_instance_type_config_configuration.rs"
        );
        include!(
            "types/emr/cluster_master_instance_fleet_instance_type_config_ebs_config.rs"
        );
        include!("types/emr/cluster_master_instance_fleet_launch_specifications.rs");
        include!(
            "types/emr/cluster_master_instance_fleet_launch_specifications_on_demand_specification.rs"
        );
        include!(
            "types/emr/cluster_master_instance_fleet_launch_specifications_spot_specification.rs"
        );
        include!("types/emr/cluster_master_instance_group.rs");
        include!("types/emr/cluster_master_instance_group_ebs_config.rs");
        include!("types/emr/cluster_placement_group_config.rs");
        include!("types/emr/cluster_step.rs");
        include!("types/emr/cluster_step_hadoop_jar_step.rs");
        include!("types/emr/instance_fleet_instance_type_config.rs");
        include!("types/emr/instance_fleet_instance_type_config_configuration.rs");
        include!("types/emr/instance_fleet_instance_type_config_ebs_config.rs");
        include!("types/emr/instance_fleet_launch_specifications.rs");
        include!(
            "types/emr/instance_fleet_launch_specifications_on_demand_specification.rs"
        );
        include!("types/emr/instance_fleet_launch_specifications_spot_specification.rs");
        include!("types/emr/instance_group_ebs_config.rs");
        include!("types/emr/managed_scaling_policy_compute_limit.rs");
        include!("types/emr/get_release_labels_filters.rs");
        include!("types/emr/get_supported_instance_types_supported_instance_type.rs");
    }
    pub mod emrcontainers {
        include!("types/emrcontainers/job_template_job_template_data.rs");
        include!(
            "types/emrcontainers/job_template_job_template_data_configuration_overrides.rs"
        );
        include!(
            "types/emrcontainers/job_template_job_template_data_configuration_overrides_application_configuration.rs"
        );
        include!(
            "types/emrcontainers/job_template_job_template_data_configuration_overrides_application_configuration_configuration.rs"
        );
        include!(
            "types/emrcontainers/job_template_job_template_data_configuration_overrides_monitoring_configuration.rs"
        );
        include!(
            "types/emrcontainers/job_template_job_template_data_configuration_overrides_monitoring_configuration_cloud_watch_monitoring_configuration.rs"
        );
        include!(
            "types/emrcontainers/job_template_job_template_data_configuration_overrides_monitoring_configuration_s_3_monitoring_configuration.rs"
        );
        include!("types/emrcontainers/job_template_job_template_data_job_driver.rs");
        include!(
            "types/emrcontainers/job_template_job_template_data_job_driver_spark_sql_job_driver.rs"
        );
        include!(
            "types/emrcontainers/job_template_job_template_data_job_driver_spark_submit_job_driver.rs"
        );
        include!("types/emrcontainers/virtual_cluster_container_provider.rs");
        include!("types/emrcontainers/virtual_cluster_container_provider_info.rs");
        include!(
            "types/emrcontainers/virtual_cluster_container_provider_info_eks_info.rs"
        );
        include!("types/emrcontainers/get_virtual_cluster_container_provider.rs");
        include!("types/emrcontainers/get_virtual_cluster_container_provider_info.rs");
        include!(
            "types/emrcontainers/get_virtual_cluster_container_provider_info_eks_info.rs"
        );
    }
    pub mod emrserverless {
        include!("types/emrserverless/application_auto_start_configuration.rs");
        include!("types/emrserverless/application_auto_stop_configuration.rs");
        include!("types/emrserverless/application_image_configuration.rs");
        include!("types/emrserverless/application_initial_capacity.rs");
        include!(
            "types/emrserverless/application_initial_capacity_initial_capacity_config.rs"
        );
        include!(
            "types/emrserverless/application_initial_capacity_initial_capacity_config_worker_configuration.rs"
        );
        include!("types/emrserverless/application_interactive_configuration.rs");
        include!("types/emrserverless/application_maximum_capacity.rs");
        include!("types/emrserverless/application_network_configuration.rs");
    }
    pub mod evidently {
        include!("types/evidently/feature_evaluation_rule.rs");
        include!("types/evidently/feature_variation.rs");
        include!("types/evidently/feature_variation_value.rs");
        include!("types/evidently/launch_execution.rs");
        include!("types/evidently/launch_group.rs");
        include!("types/evidently/launch_metric_monitor.rs");
        include!("types/evidently/launch_metric_monitor_metric_definition.rs");
        include!("types/evidently/launch_scheduled_splits_config.rs");
        include!("types/evidently/launch_scheduled_splits_config_step.rs");
        include!(
            "types/evidently/launch_scheduled_splits_config_step_segment_override.rs"
        );
        include!("types/evidently/project_data_delivery.rs");
        include!("types/evidently/project_data_delivery_cloudwatch_logs.rs");
        include!("types/evidently/project_data_delivery_s_3_destination.rs");
    }
    pub mod finspace {
        include!("types/finspace/kx_cluster_auto_scaling_configuration.rs");
        include!("types/finspace/kx_cluster_cache_storage_configuration.rs");
        include!("types/finspace/kx_cluster_capacity_configuration.rs");
        include!("types/finspace/kx_cluster_code.rs");
        include!("types/finspace/kx_cluster_database.rs");
        include!("types/finspace/kx_cluster_database_cache_configuration.rs");
        include!("types/finspace/kx_cluster_savedown_storage_configuration.rs");
        include!("types/finspace/kx_cluster_scaling_group_configuration.rs");
        include!("types/finspace/kx_cluster_tickerplant_log_configuration.rs");
        include!("types/finspace/kx_cluster_vpc_configuration.rs");
        include!("types/finspace/kx_dataview_segment_configuration.rs");
        include!("types/finspace/kx_environment_custom_dns_configuration.rs");
        include!("types/finspace/kx_environment_transit_gateway_configuration.rs");
        include!(
            "types/finspace/kx_environment_transit_gateway_configuration_attachment_network_acl_configuration.rs"
        );
        include!(
            "types/finspace/kx_environment_transit_gateway_configuration_attachment_network_acl_configuration_icmp_type_code.rs"
        );
        include!(
            "types/finspace/kx_environment_transit_gateway_configuration_attachment_network_acl_configuration_port_range.rs"
        );
        include!("types/finspace/kx_volume_attached_cluster.rs");
        include!("types/finspace/kx_volume_nas_1_configuration.rs");
    }
    pub mod fis {
        include!("types/fis/experiment_template_action.rs");
        include!("types/fis/experiment_template_action_parameter.rs");
        include!("types/fis/experiment_template_action_target.rs");
        include!("types/fis/experiment_template_experiment_options.rs");
        include!("types/fis/experiment_template_log_configuration.rs");
        include!(
            "types/fis/experiment_template_log_configuration_cloudwatch_logs_configuration.rs"
        );
        include!("types/fis/experiment_template_log_configuration_s_3_configuration.rs");
        include!("types/fis/experiment_template_stop_condition.rs");
        include!("types/fis/experiment_template_target.rs");
        include!("types/fis/experiment_template_target_filter.rs");
        include!("types/fis/experiment_template_target_resource_tag.rs");
    }
    pub mod fms {
        include!("types/fms/policy_exclude_map.rs");
        include!("types/fms/policy_include_map.rs");
        include!("types/fms/policy_security_service_policy_data.rs");
        include!("types/fms/policy_security_service_policy_data_policy_option.rs");
        include!(
            "types/fms/policy_security_service_policy_data_policy_option_network_firewall_policy.rs"
        );
        include!(
            "types/fms/policy_security_service_policy_data_policy_option_third_party_firewall_policy.rs"
        );
        include!("types/fms/resource_set_resource_set.rs");
        include!("types/fms/resource_set_timeouts.rs");
    }
    pub mod fsx {
        include!("types/fsx/data_repository_association_s_3.rs");
        include!("types/fsx/data_repository_association_s_3_auto_export_policy.rs");
        include!("types/fsx/data_repository_association_s_3_auto_import_policy.rs");
        include!("types/fsx/file_cache_data_repository_association.rs");
        include!("types/fsx/file_cache_data_repository_association_nf.rs");
        include!("types/fsx/file_cache_lustre_configuration.rs");
        include!("types/fsx/file_cache_lustre_configuration_log_configuration.rs");
        include!("types/fsx/file_cache_lustre_configuration_metadata_configuration.rs");
        include!("types/fsx/lustre_file_system_log_configuration.rs");
        include!("types/fsx/lustre_file_system_metadata_configuration.rs");
        include!("types/fsx/lustre_file_system_root_squash_configuration.rs");
        include!("types/fsx/ontap_file_system_disk_iops_configuration.rs");
        include!("types/fsx/ontap_file_system_endpoint.rs");
        include!("types/fsx/ontap_file_system_endpoint_intercluster.rs");
        include!("types/fsx/ontap_file_system_endpoint_management.rs");
        include!(
            "types/fsx/ontap_storage_virtual_machine_active_directory_configuration.rs"
        );
        include!(
            "types/fsx/ontap_storage_virtual_machine_active_directory_configuration_self_managed_active_directory_configuration.rs"
        );
        include!("types/fsx/ontap_storage_virtual_machine_endpoint.rs");
        include!("types/fsx/ontap_storage_virtual_machine_endpoint_iscsi.rs");
        include!("types/fsx/ontap_storage_virtual_machine_endpoint_management.rs");
        include!("types/fsx/ontap_storage_virtual_machine_endpoint_nf.rs");
        include!("types/fsx/ontap_storage_virtual_machine_endpoint_smb.rs");
        include!("types/fsx/ontap_volume_aggregate_configuration.rs");
        include!("types/fsx/ontap_volume_snaplock_configuration.rs");
        include!("types/fsx/ontap_volume_snaplock_configuration_autocommit_period.rs");
        include!("types/fsx/ontap_volume_snaplock_configuration_retention_period.rs");
        include!(
            "types/fsx/ontap_volume_snaplock_configuration_retention_period_default_retention.rs"
        );
        include!(
            "types/fsx/ontap_volume_snaplock_configuration_retention_period_maximum_retention.rs"
        );
        include!(
            "types/fsx/ontap_volume_snaplock_configuration_retention_period_minimum_retention.rs"
        );
        include!("types/fsx/ontap_volume_tiering_policy.rs");
        include!("types/fsx/open_zfs_file_system_disk_iops_configuration.rs");
        include!("types/fsx/open_zfs_file_system_root_volume_configuration.rs");
        include!(
            "types/fsx/open_zfs_file_system_root_volume_configuration_nfs_exports.rs"
        );
        include!(
            "types/fsx/open_zfs_file_system_root_volume_configuration_nfs_exports_client_configuration.rs"
        );
        include!(
            "types/fsx/open_zfs_file_system_root_volume_configuration_user_and_group_quota.rs"
        );
        include!("types/fsx/open_zfs_volume_nfs_exports.rs");
        include!("types/fsx/open_zfs_volume_nfs_exports_client_configuration.rs");
        include!("types/fsx/open_zfs_volume_origin_snapshot.rs");
        include!("types/fsx/open_zfs_volume_user_and_group_quota.rs");
        include!("types/fsx/windows_file_system_audit_log_configuration.rs");
        include!("types/fsx/windows_file_system_disk_iops_configuration.rs");
        include!("types/fsx/windows_file_system_self_managed_active_directory.rs");
        include!("types/fsx/get_ontap_file_system_disk_iops_configuration.rs");
        include!("types/fsx/get_ontap_file_system_endpoint.rs");
        include!("types/fsx/get_ontap_file_system_endpoint_intercluster.rs");
        include!("types/fsx/get_ontap_file_system_endpoint_management.rs");
        include!(
            "types/fsx/get_ontap_storage_virtual_machine_active_directory_configuration.rs"
        );
        include!(
            "types/fsx/get_ontap_storage_virtual_machine_active_directory_configuration_self_managed_active_directory_configuration.rs"
        );
        include!("types/fsx/get_ontap_storage_virtual_machine_endpoint.rs");
        include!("types/fsx/get_ontap_storage_virtual_machine_endpoint_iscsi.rs");
        include!("types/fsx/get_ontap_storage_virtual_machine_endpoint_management.rs");
        include!("types/fsx/get_ontap_storage_virtual_machine_endpoint_nf.rs");
        include!("types/fsx/get_ontap_storage_virtual_machine_endpoint_smb.rs");
        include!("types/fsx/get_ontap_storage_virtual_machine_filter.rs");
        include!(
            "types/fsx/get_ontap_storage_virtual_machine_lifecycle_transition_reason.rs"
        );
        include!("types/fsx/get_ontap_storage_virtual_machines_filter.rs");
        include!("types/fsx/get_open_zfs_snapshot_filter.rs");
        include!("types/fsx/get_windows_file_system_audit_log_configuration.rs");
        include!("types/fsx/get_windows_file_system_disk_iops_configuration.rs");
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
