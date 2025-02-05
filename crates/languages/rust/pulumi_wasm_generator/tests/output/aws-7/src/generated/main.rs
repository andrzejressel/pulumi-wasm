pub mod detective {
    include!("resources/detective/graph.rs");
    include!("resources/detective/invitation_accepter.rs");
    include!("resources/detective/member.rs");
    include!("resources/detective/organization_admin_account.rs");
    include!("resources/detective/organization_configuration.rs");
}
pub mod devicefarm {
    include!("resources/devicefarm/device_pool.rs");
    include!("resources/devicefarm/instance_profile.rs");
    include!("resources/devicefarm/network_profile.rs");
    include!("resources/devicefarm/project.rs");
    include!("resources/devicefarm/test_grid_project.rs");
    include!("resources/devicefarm/upload.rs");
}
pub mod devopsguru {
    include!("resources/devopsguru/event_sources_config.rs");
    include!("resources/devopsguru/notification_channel.rs");
    include!("resources/devopsguru/resource_collection.rs");
    include!("resources/devopsguru/service_integration.rs");
}
pub mod directconnect {
    include!("resources/directconnect/bgp_peer.rs");
    include!("resources/directconnect/connection.rs");
    include!("resources/directconnect/connection_association.rs");
    include!("resources/directconnect/connection_confirmation.rs");
    include!("resources/directconnect/gateway.rs");
    include!("resources/directconnect/gateway_association.rs");
    include!("resources/directconnect/gateway_association_proposal.rs");
    include!("resources/directconnect/hosted_connection.rs");
    include!("resources/directconnect/hosted_private_virtual_interface.rs");
    include!("resources/directconnect/hosted_private_virtual_interface_accepter.rs");
    include!("resources/directconnect/hosted_public_virtual_interface.rs");
    include!("resources/directconnect/hosted_public_virtual_interface_accepter.rs");
    include!("resources/directconnect/hosted_transit_virtual_interface.rs");
    include!("resources/directconnect/hosted_transit_virtual_interface_acceptor.rs");
    include!("resources/directconnect/link_aggregation_group.rs");
    include!("resources/directconnect/macsec_key_association.rs");
    include!("resources/directconnect/private_virtual_interface.rs");
    include!("resources/directconnect/public_virtual_interface.rs");
    include!("resources/directconnect/transit_virtual_interface.rs");
}
pub mod directoryservice {
    include!("resources/directoryservice/conditional_forwader.rs");
    include!("resources/directoryservice/directory.rs");
    include!("resources/directoryservice/log_service.rs");
    include!("resources/directoryservice/radius_settings.rs");
    include!("resources/directoryservice/service_region.rs");
    include!("resources/directoryservice/shared_directory.rs");
    include!("resources/directoryservice/shared_directory_accepter.rs");
    include!("resources/directoryservice/trust.rs");
}
pub mod dlm {
    include!("resources/dlm/lifecycle_policy.rs");
}
pub mod dms {
    include!("resources/dms/certificate.rs");
    include!("resources/dms/endpoint.rs");
    include!("resources/dms/event_subscription.rs");
    include!("resources/dms/replication_config.rs");
    include!("resources/dms/replication_instance.rs");
    include!("resources/dms/replication_subnet_group.rs");
    include!("resources/dms/replication_task.rs");
    include!("resources/dms/s_3_endpoint.rs");
}
pub mod docdb {
    include!("resources/docdb/cluster.rs");
    include!("resources/docdb/cluster_instance.rs");
    include!("resources/docdb/cluster_parameter_group.rs");
    include!("resources/docdb/cluster_snapshot.rs");
    include!("resources/docdb/elastic_cluster.rs");
    include!("resources/docdb/event_subscription.rs");
    include!("resources/docdb/global_cluster.rs");
    include!("resources/docdb/subnet_group.rs");
}
pub mod drs {
    include!("resources/drs/replication_configuration_template.rs");
}
pub mod dynamodb {
    include!("resources/dynamodb/contributor_insights.rs");
    include!("resources/dynamodb/global_table.rs");
    include!("resources/dynamodb/kinesis_streaming_destination.rs");
    include!("resources/dynamodb/resource_policy.rs");
    include!("resources/dynamodb/table.rs");
    include!("resources/dynamodb/table_export.rs");
    include!("resources/dynamodb/table_item.rs");
    include!("resources/dynamodb/table_replica.rs");
    include!("resources/dynamodb/tag.rs");
}
pub mod functions {
    pub mod devopsguru {
        include!("functions/devopsguru/get_notification_channel.rs");
        include!("functions/devopsguru/get_resource_collection.rs");
    }
    pub mod directconnect {
        include!("functions/directconnect/get_connection.rs");
        include!("functions/directconnect/get_gateway.rs");
        include!("functions/directconnect/get_location.rs");
        include!("functions/directconnect/get_locations.rs");
        include!("functions/directconnect/get_router_configuration.rs");
    }
    pub mod directoryservice {
        include!("functions/directoryservice/get_directory.rs");
    }
    pub mod dms {
        include!("functions/dms/get_certificate.rs");
        include!("functions/dms/get_endpoint.rs");
        include!("functions/dms/get_replication_instance.rs");
        include!("functions/dms/get_replication_subnet_group.rs");
        include!("functions/dms/get_replication_task.rs");
    }
    pub mod docdb {
        include!("functions/docdb/get_engine_version.rs");
        include!("functions/docdb/get_orderable_db_instance.rs");
    }
    pub mod dynamodb {
        include!("functions/dynamodb/get_table.rs");
        include!("functions/dynamodb/get_table_item.rs");
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
    pub mod devicefarm {
        include!("types/devicefarm/device_pool_rule.rs");
        include!("types/devicefarm/test_grid_project_vpc_config.rs");
    }
    pub mod devopsguru {
        include!("types/devopsguru/event_sources_config_event_source.rs");
        include!(
            "types/devopsguru/event_sources_config_event_source_amazon_code_guru_profiler.rs"
        );
        include!("types/devopsguru/notification_channel_filters.rs");
        include!("types/devopsguru/notification_channel_sns.rs");
        include!("types/devopsguru/resource_collection_cloudformation.rs");
        include!("types/devopsguru/resource_collection_tags.rs");
        include!("types/devopsguru/service_integration_kms_server_side_encryption.rs");
        include!("types/devopsguru/service_integration_logs_anomaly_detection.rs");
        include!("types/devopsguru/service_integration_ops_center.rs");
        include!("types/devopsguru/get_notification_channel_filter.rs");
        include!("types/devopsguru/get_notification_channel_sn.rs");
        include!("types/devopsguru/get_resource_collection_cloudformation.rs");
        include!("types/devopsguru/get_resource_collection_tag.rs");
    }
    pub mod directconnect {
        include!("types/directconnect/get_router_configuration_router.rs");
    }
    pub mod directoryservice {
        include!("types/directoryservice/directory_connect_settings.rs");
        include!("types/directoryservice/directory_vpc_settings.rs");
        include!("types/directoryservice/service_region_vpc_settings.rs");
        include!("types/directoryservice/shared_directory_target.rs");
        include!("types/directoryservice/get_directory_connect_setting.rs");
        include!("types/directoryservice/get_directory_radius_setting.rs");
        include!("types/directoryservice/get_directory_vpc_setting.rs");
    }
    pub mod dlm {
        include!("types/dlm/lifecycle_policy_policy_details.rs");
        include!("types/dlm/lifecycle_policy_policy_details_action.rs");
        include!(
            "types/dlm/lifecycle_policy_policy_details_action_cross_region_copy.rs"
        );
        include!(
            "types/dlm/lifecycle_policy_policy_details_action_cross_region_copy_encryption_configuration.rs"
        );
        include!(
            "types/dlm/lifecycle_policy_policy_details_action_cross_region_copy_retain_rule.rs"
        );
        include!("types/dlm/lifecycle_policy_policy_details_event_source.rs");
        include!("types/dlm/lifecycle_policy_policy_details_event_source_parameters.rs");
        include!("types/dlm/lifecycle_policy_policy_details_parameters.rs");
        include!("types/dlm/lifecycle_policy_policy_details_schedule.rs");
        include!("types/dlm/lifecycle_policy_policy_details_schedule_create_rule.rs");
        include!(
            "types/dlm/lifecycle_policy_policy_details_schedule_cross_region_copy_rule.rs"
        );
        include!(
            "types/dlm/lifecycle_policy_policy_details_schedule_cross_region_copy_rule_deprecate_rule.rs"
        );
        include!(
            "types/dlm/lifecycle_policy_policy_details_schedule_cross_region_copy_rule_retain_rule.rs"
        );
        include!("types/dlm/lifecycle_policy_policy_details_schedule_deprecate_rule.rs");
        include!(
            "types/dlm/lifecycle_policy_policy_details_schedule_fast_restore_rule.rs"
        );
        include!("types/dlm/lifecycle_policy_policy_details_schedule_retain_rule.rs");
        include!("types/dlm/lifecycle_policy_policy_details_schedule_share_rule.rs");
    }
    pub mod dms {
        include!("types/dms/endpoint_elasticsearch_settings.rs");
        include!("types/dms/endpoint_kafka_settings.rs");
        include!("types/dms/endpoint_kinesis_settings.rs");
        include!("types/dms/endpoint_mongodb_settings.rs");
        include!("types/dms/endpoint_postgres_settings.rs");
        include!("types/dms/endpoint_redis_settings.rs");
        include!("types/dms/endpoint_redshift_settings.rs");
        include!("types/dms/endpoint_s_3_settings.rs");
        include!("types/dms/replication_config_compute_config.rs");
        include!("types/dms/get_endpoint_elasticsearch_setting.rs");
        include!("types/dms/get_endpoint_kafka_setting.rs");
        include!("types/dms/get_endpoint_kinesis_setting.rs");
        include!("types/dms/get_endpoint_mongodb_setting.rs");
        include!("types/dms/get_endpoint_postgres_setting.rs");
        include!("types/dms/get_endpoint_redis_setting.rs");
        include!("types/dms/get_endpoint_redshift_setting.rs");
        include!("types/dms/get_endpoint_s_3_setting.rs");
    }
    pub mod docdb {
        include!("types/docdb/cluster_parameter_group_parameter.rs");
        include!("types/docdb/cluster_restore_to_point_in_time.rs");
        include!("types/docdb/elastic_cluster_timeouts.rs");
        include!("types/docdb/global_cluster_global_cluster_member.rs");
    }
    pub mod drs {
        include!("types/drs/replication_configuration_template_pit_policy.rs");
        include!("types/drs/replication_configuration_template_timeouts.rs");
    }
    pub mod dynamodb {
        include!("types/dynamodb/global_table_replica.rs");
        include!("types/dynamodb/table_attribute.rs");
        include!("types/dynamodb/table_global_secondary_index.rs");
        include!("types/dynamodb/table_global_secondary_index_on_demand_throughput.rs");
        include!("types/dynamodb/table_import_table.rs");
        include!("types/dynamodb/table_import_table_input_format_options.rs");
        include!("types/dynamodb/table_import_table_input_format_options_csv.rs");
        include!("types/dynamodb/table_import_table_s_3_bucket_source.rs");
        include!("types/dynamodb/table_local_secondary_index.rs");
        include!("types/dynamodb/table_on_demand_throughput.rs");
        include!("types/dynamodb/table_point_in_time_recovery.rs");
        include!("types/dynamodb/table_replica.rs");
        include!("types/dynamodb/table_server_side_encryption.rs");
        include!("types/dynamodb/table_ttl.rs");
        include!("types/dynamodb/get_table_attribute.rs");
        include!("types/dynamodb/get_table_global_secondary_index.rs");
        include!("types/dynamodb/get_table_local_secondary_index.rs");
        include!("types/dynamodb/get_table_point_in_time_recovery.rs");
        include!("types/dynamodb/get_table_replica.rs");
        include!("types/dynamodb/get_table_server_side_encryption.rs");
        include!("types/dynamodb/get_table_ttl.rs");
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
        pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::output_interface
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
