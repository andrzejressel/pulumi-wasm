pub mod rds {
    include!("resources/rds/certificate.rs");
    include!("resources/rds/cluster.rs");
    include!("resources/rds/cluster_activity_stream.rs");
    include!("resources/rds/cluster_endpoint.rs");
    include!("resources/rds/cluster_instance.rs");
    include!("resources/rds/cluster_parameter_group.rs");
    include!("resources/rds/cluster_role_association.rs");
    include!("resources/rds/cluster_snapshot.rs");
    include!("resources/rds/cluster_snapshot_copy.rs");
    include!("resources/rds/custom_db_engine_version.rs");
    include!("resources/rds/event_subscription.rs");
    include!("resources/rds/export_task.rs");
    include!("resources/rds/global_cluster.rs");
    include!("resources/rds/instance.rs");
    include!("resources/rds/instance_automated_backups_replication.rs");
    include!("resources/rds/instance_desired_state.rs");
    include!("resources/rds/integration.rs");
    include!("resources/rds/option_group.rs");
    include!("resources/rds/parameter_group.rs");
    include!("resources/rds/proxy.rs");
    include!("resources/rds/proxy_default_target_group.rs");
    include!("resources/rds/proxy_endpoint.rs");
    include!("resources/rds/proxy_target.rs");
    include!("resources/rds/reserved_instance.rs");
    include!("resources/rds/role_association.rs");
    include!("resources/rds/snapshot.rs");
    include!("resources/rds/snapshot_copy.rs");
    include!("resources/rds/subnet_group.rs");
}
pub mod redshift {
    include!("resources/redshift/authentication_profile.rs");
    include!("resources/redshift/cluster.rs");
    include!("resources/redshift/cluster_iam_roles.rs");
    include!("resources/redshift/cluster_snapshot.rs");
    include!("resources/redshift/data_share_authorization.rs");
    include!("resources/redshift/data_share_consumer_association.rs");
    include!("resources/redshift/endpoint_access.rs");
    include!("resources/redshift/endpoint_authorization.rs");
    include!("resources/redshift/event_subscription.rs");
    include!("resources/redshift/hsm_client_certificate.rs");
    include!("resources/redshift/hsm_configuration.rs");
    include!("resources/redshift/logging.rs");
    include!("resources/redshift/parameter_group.rs");
    include!("resources/redshift/partner.rs");
    include!("resources/redshift/resource_policy.rs");
    include!("resources/redshift/scheduled_action.rs");
    include!("resources/redshift/snapshot_copy.rs");
    include!("resources/redshift/snapshot_copy_grant.rs");
    include!("resources/redshift/snapshot_schedule.rs");
    include!("resources/redshift/snapshot_schedule_association.rs");
    include!("resources/redshift/subnet_group.rs");
    include!("resources/redshift/usage_limit.rs");
}
pub mod redshiftdata {
    include!("resources/redshiftdata/statement.rs");
}
pub mod redshiftserverless {
    include!("resources/redshiftserverless/custom_domain_association.rs");
    include!("resources/redshiftserverless/endpoint_access.rs");
    include!("resources/redshiftserverless/namespace.rs");
    include!("resources/redshiftserverless/resource_policy.rs");
    include!("resources/redshiftserverless/snapshot.rs");
    include!("resources/redshiftserverless/usage_limit.rs");
    include!("resources/redshiftserverless/workgroup.rs");
}
pub mod rekognition {
    include!("resources/rekognition/collection.rs");
    include!("resources/rekognition/project.rs");
    include!("resources/rekognition/stream_processor.rs");
}
pub mod resiliencehub {
    include!("resources/resiliencehub/resiliency_policy.rs");
}
pub mod resourceexplorer {
    include!("resources/resourceexplorer/index.rs");
    include!("resources/resourceexplorer/view.rs");
}
pub mod resourcegroups {
    include!("resources/resourcegroups/group.rs");
    include!("resources/resourcegroups/resource.rs");
}
pub mod rolesanywhere {
    include!("resources/rolesanywhere/profile.rs");
    include!("resources/rolesanywhere/trust_anchor.rs");
}
pub mod functions {
    pub mod rds {
        include!("functions/rds/get_certificate.rs");
        include!("functions/rds/get_cluster.rs");
        include!("functions/rds/get_cluster_parameter_group.rs");
        include!("functions/rds/get_cluster_snapshot.rs");
        include!("functions/rds/get_clusters.rs");
        include!("functions/rds/get_engine_version.rs");
        include!("functions/rds/get_event_categories.rs");
        include!("functions/rds/get_instance.rs");
        include!("functions/rds/get_instances.rs");
        include!("functions/rds/get_orderable_db_instance.rs");
        include!("functions/rds/get_parameter_group.rs");
        include!("functions/rds/get_proxy.rs");
        include!("functions/rds/get_reserved_instance_offering.rs");
        include!("functions/rds/get_snapshot.rs");
        include!("functions/rds/get_subnet_group.rs");
    }
    pub mod redshift {
        include!("functions/redshift/get_cluster.rs");
        include!("functions/redshift/get_cluster_credentials.rs");
        include!("functions/redshift/get_data_shares.rs");
        include!("functions/redshift/get_orderable_cluster.rs");
        include!("functions/redshift/get_producer_data_shares.rs");
        include!("functions/redshift/get_service_account.rs");
        include!("functions/redshift/get_subnet_group.rs");
    }
    pub mod redshiftserverless {
        include!("functions/redshiftserverless/get_credentials.rs");
        include!("functions/redshiftserverless/get_namespace.rs");
        include!("functions/redshiftserverless/get_workgroup.rs");
    }
    pub mod resourceexplorer {
        include!("functions/resourceexplorer/search.rs");
    }
    pub mod resourcegroupstaggingapi {
        include!("functions/resourcegroupstaggingapi/get_resources.rs");
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
    pub mod rds {
        include!("types/rds/cluster_master_user_secret.rs");
        include!("types/rds/cluster_parameter_group_parameter.rs");
        include!("types/rds/cluster_restore_to_point_in_time.rs");
        include!("types/rds/cluster_s_3_import.rs");
        include!("types/rds/cluster_scaling_configuration.rs");
        include!("types/rds/cluster_serverlessv_2_scaling_configuration.rs");
        include!("types/rds/cluster_snapshot_copy_timeouts.rs");
        include!("types/rds/export_task_timeouts.rs");
        include!("types/rds/global_cluster_global_cluster_member.rs");
        include!("types/rds/instance_blue_green_update.rs");
        include!("types/rds/instance_desired_state_timeouts.rs");
        include!("types/rds/instance_listener_endpoint.rs");
        include!("types/rds/instance_master_user_secret.rs");
        include!("types/rds/instance_restore_to_point_in_time.rs");
        include!("types/rds/instance_s_3_import.rs");
        include!("types/rds/integration_timeouts.rs");
        include!("types/rds/option_group_option.rs");
        include!("types/rds/option_group_option_option_setting.rs");
        include!("types/rds/parameter_group_parameter.rs");
        include!("types/rds/proxy_auth.rs");
        include!("types/rds/proxy_default_target_group_connection_pool_config.rs");
        include!("types/rds/reserved_instance_recurring_charge.rs");
        include!("types/rds/get_cluster_master_user_secret.rs");
        include!("types/rds/get_clusters_filter.rs");
        include!("types/rds/get_engine_version_filter.rs");
        include!("types/rds/get_instance_master_user_secret.rs");
        include!("types/rds/get_instances_filter.rs");
        include!("types/rds/get_proxy_auth.rs");
    }
    pub mod redshift {
        include!("types/redshift/cluster_cluster_node.rs");
        include!("types/redshift/cluster_logging.rs");
        include!("types/redshift/cluster_snapshot_copy.rs");
        include!("types/redshift/endpoint_access_vpc_endpoint.rs");
        include!("types/redshift/endpoint_access_vpc_endpoint_network_interface.rs");
        include!("types/redshift/parameter_group_parameter.rs");
        include!("types/redshift/scheduled_action_target_action.rs");
        include!("types/redshift/scheduled_action_target_action_pause_cluster.rs");
        include!("types/redshift/scheduled_action_target_action_resize_cluster.rs");
        include!("types/redshift/scheduled_action_target_action_resume_cluster.rs");
        include!("types/redshift/get_cluster_cluster_node.rs");
        include!("types/redshift/get_data_shares_data_share.rs");
        include!("types/redshift/get_producer_data_shares_data_share.rs");
    }
    pub mod redshiftdata {
        include!("types/redshiftdata/statement_parameter.rs");
    }
    pub mod redshiftserverless {
        include!("types/redshiftserverless/endpoint_access_vpc_endpoint.rs");
        include!(
            "types/redshiftserverless/endpoint_access_vpc_endpoint_network_interface.rs"
        );
        include!("types/redshiftserverless/workgroup_config_parameter.rs");
        include!("types/redshiftserverless/workgroup_endpoint.rs");
        include!("types/redshiftserverless/workgroup_endpoint_vpc_endpoint.rs");
        include!(
            "types/redshiftserverless/workgroup_endpoint_vpc_endpoint_network_interface.rs"
        );
        include!("types/redshiftserverless/get_workgroup_endpoint.rs");
        include!("types/redshiftserverless/get_workgroup_endpoint_vpc_endpoint.rs");
        include!(
            "types/redshiftserverless/get_workgroup_endpoint_vpc_endpoint_network_interface.rs"
        );
    }
    pub mod rekognition {
        include!("types/rekognition/collection_timeouts.rs");
        include!("types/rekognition/project_timeouts.rs");
        include!("types/rekognition/stream_processor_data_sharing_preference.rs");
        include!("types/rekognition/stream_processor_input.rs");
        include!("types/rekognition/stream_processor_input_kinesis_video_stream.rs");
        include!("types/rekognition/stream_processor_notification_channel.rs");
        include!("types/rekognition/stream_processor_output.rs");
        include!("types/rekognition/stream_processor_output_kinesis_data_stream.rs");
        include!("types/rekognition/stream_processor_output_s_3_destination.rs");
        include!("types/rekognition/stream_processor_regions_of_interest.rs");
        include!(
            "types/rekognition/stream_processor_regions_of_interest_bounding_box.rs"
        );
        include!("types/rekognition/stream_processor_regions_of_interest_polygon.rs");
        include!("types/rekognition/stream_processor_settings.rs");
        include!("types/rekognition/stream_processor_settings_connected_home.rs");
        include!("types/rekognition/stream_processor_settings_face_search.rs");
        include!("types/rekognition/stream_processor_timeouts.rs");
    }
    pub mod resiliencehub {
        include!("types/resiliencehub/resiliency_policy_policy.rs");
        include!("types/resiliencehub/resiliency_policy_policy_az.rs");
        include!("types/resiliencehub/resiliency_policy_policy_hardware.rs");
        include!("types/resiliencehub/resiliency_policy_policy_region.rs");
        include!("types/resiliencehub/resiliency_policy_policy_software.rs");
        include!("types/resiliencehub/resiliency_policy_timeouts.rs");
    }
    pub mod resourceexplorer {
        include!("types/resourceexplorer/index_timeouts.rs");
        include!("types/resourceexplorer/search_resource.rs");
        include!("types/resourceexplorer/search_resource_count.rs");
        include!("types/resourceexplorer/search_resource_property.rs");
        include!("types/resourceexplorer/view_filters.rs");
        include!("types/resourceexplorer/view_included_property.rs");
    }
    pub mod resourcegroups {
        include!("types/resourcegroups/group_configuration.rs");
        include!("types/resourcegroups/group_configuration_parameter.rs");
        include!("types/resourcegroups/group_resource_query.rs");
    }
    pub mod resourcegroupstaggingapi {
        include!(
            "types/resourcegroupstaggingapi/get_resources_resource_tag_mapping_list.rs"
        );
        include!(
            "types/resourcegroupstaggingapi/get_resources_resource_tag_mapping_list_compliance_detail.rs"
        );
        include!("types/resourcegroupstaggingapi/get_resources_tag_filter.rs");
    }
    pub mod rolesanywhere {
        include!("types/rolesanywhere/trust_anchor_notification_setting.rs");
        include!("types/rolesanywhere/trust_anchor_source.rs");
        include!("types/rolesanywhere/trust_anchor_source_source_data.rs");
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
#[link_section = "pulumi_wasm_provider::aws"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_AWS: [u8; 45] = *b"{\"version\":\"6.66.2\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "6.66.2".to_string()
}
