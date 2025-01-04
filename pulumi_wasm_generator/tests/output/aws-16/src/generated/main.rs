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
pub mod route53 {
    include!("resources/route53/cidr_collection.rs");
    include!("resources/route53/cidr_location.rs");
    include!("resources/route53/delegation_set.rs");
    include!("resources/route53/health_check.rs");
    include!("resources/route53/hosted_zone_dns_sec.rs");
    include!("resources/route53/key_signing_key.rs");
    include!("resources/route53/profiles_association.rs");
    include!("resources/route53/profiles_profile.rs");
    include!("resources/route53/profiles_resource_association.rs");
    include!("resources/route53/query_log.rs");
    include!("resources/route53/record.rs");
    include!("resources/route53/resolver_config.rs");
    include!("resources/route53/resolver_dns_sec_config.rs");
    include!("resources/route53/resolver_endpoint.rs");
    include!("resources/route53/resolver_firewall_config.rs");
    include!("resources/route53/resolver_firewall_domain_list.rs");
    include!("resources/route53/resolver_firewall_rule.rs");
    include!("resources/route53/resolver_firewall_rule_group.rs");
    include!("resources/route53/resolver_firewall_rule_group_association.rs");
    include!("resources/route53/resolver_query_log_config.rs");
    include!("resources/route53/resolver_query_log_config_association.rs");
    include!("resources/route53/resolver_rule.rs");
    include!("resources/route53/resolver_rule_association.rs");
    include!("resources/route53/traffic_policy.rs");
    include!("resources/route53/traffic_policy_instance.rs");
    include!("resources/route53/vpc_association_authorization.rs");
    include!("resources/route53/zone.rs");
    include!("resources/route53/zone_association.rs");
}
pub mod route53domains {
    include!("resources/route53domains/delegation_signer_record.rs");
    include!("resources/route53domains/registered_domain.rs");
}
pub mod route53recoverycontrol {
    include!("resources/route53recoverycontrol/cluster.rs");
    include!("resources/route53recoverycontrol/control_panel.rs");
    include!("resources/route53recoverycontrol/routing_control.rs");
    include!("resources/route53recoverycontrol/safety_rule.rs");
}
pub mod functions {
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
    pub mod route53 {
        include!("functions/route53/get_delegation_set.rs");
        include!("functions/route53/get_profiles_profiles.rs");
        include!("functions/route53/get_query_log_config.rs");
        include!("functions/route53/get_resolver_endpoint.rs");
        include!("functions/route53/get_resolver_firewall_config.rs");
        include!("functions/route53/get_resolver_firewall_domain_list.rs");
        include!("functions/route53/get_resolver_firewall_rule_group.rs");
        include!("functions/route53/get_resolver_firewall_rule_group_association.rs");
        include!("functions/route53/get_resolver_firewall_rules.rs");
        include!("functions/route53/get_resolver_rule.rs");
        include!("functions/route53/get_resolver_rules.rs");
        include!("functions/route53/get_traffic_policy_document.rs");
        include!("functions/route53/get_zone.rs");
        include!("functions/route53/get_zones.rs");
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
    pub mod route53 {
        include!("types/route53/profiles_association_timeouts.rs");
        include!("types/route53/profiles_profile_timeouts.rs");
        include!("types/route53/profiles_resource_association_timeouts.rs");
        include!("types/route53/record_alias.rs");
        include!("types/route53/record_cidr_routing_policy.rs");
        include!("types/route53/record_failover_routing_policy.rs");
        include!("types/route53/record_geolocation_routing_policy.rs");
        include!("types/route53/record_geoproximity_routing_policy.rs");
        include!("types/route53/record_geoproximity_routing_policy_coordinate.rs");
        include!("types/route53/record_latency_routing_policy.rs");
        include!("types/route53/record_weighted_routing_policy.rs");
        include!("types/route53/resolver_endpoint_ip_address.rs");
        include!("types/route53/resolver_rule_target_ip.rs");
        include!("types/route53/zone_vpc.rs");
        include!("types/route53/get_profiles_profiles_profile.rs");
        include!("types/route53/get_query_log_config_filter.rs");
        include!("types/route53/get_resolver_endpoint_filter.rs");
        include!("types/route53/get_resolver_firewall_rules_firewall_rule.rs");
        include!("types/route53/get_traffic_policy_document_endpoint.rs");
        include!("types/route53/get_traffic_policy_document_rule.rs");
        include!(
            "types/route53/get_traffic_policy_document_rule_geo_proximity_location.rs"
        );
        include!("types/route53/get_traffic_policy_document_rule_item.rs");
        include!("types/route53/get_traffic_policy_document_rule_location.rs");
        include!("types/route53/get_traffic_policy_document_rule_primary.rs");
        include!("types/route53/get_traffic_policy_document_rule_region.rs");
        include!("types/route53/get_traffic_policy_document_rule_secondary.rs");
    }
    pub mod route53domains {
        include!("types/route53domains/delegation_signer_record_signing_attributes.rs");
        include!("types/route53domains/delegation_signer_record_timeouts.rs");
        include!("types/route53domains/registered_domain_admin_contact.rs");
        include!("types/route53domains/registered_domain_billing_contact.rs");
        include!("types/route53domains/registered_domain_name_server.rs");
        include!("types/route53domains/registered_domain_registrant_contact.rs");
        include!("types/route53domains/registered_domain_tech_contact.rs");
    }
    pub mod route53recoverycontrol {
        include!("types/route53recoverycontrol/cluster_cluster_endpoint.rs");
        include!("types/route53recoverycontrol/safety_rule_rule_config.rs");
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
