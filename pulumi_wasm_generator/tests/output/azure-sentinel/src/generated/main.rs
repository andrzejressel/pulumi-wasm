pub mod sentinel {
    include!("resources/sentinel/alert_rule_anomaly_built_in.rs");
    include!("resources/sentinel/alert_rule_anomaly_duplicate.rs");
    include!("resources/sentinel/alert_rule_fusion.rs");
    include!("resources/sentinel/alert_rule_machine_learning_behavior_analytics.rs");
    include!("resources/sentinel/alert_rule_ms_security_incident.rs");
    include!("resources/sentinel/alert_rule_nrt.rs");
    include!("resources/sentinel/alert_rule_scheduled.rs");
    include!("resources/sentinel/alert_rule_threat_intelligence.rs");
    include!("resources/sentinel/authomation_rule.rs");
    include!("resources/sentinel/automation_rule.rs");
    include!("resources/sentinel/data_connector_aws_cloud_trail.rs");
    include!("resources/sentinel/data_connector_aws_s_3.rs");
    include!("resources/sentinel/data_connector_azure_active_directory.rs");
    include!("resources/sentinel/data_connector_azure_advanced_thread_protection.rs");
    include!("resources/sentinel/data_connector_azure_security_center.rs");
    include!("resources/sentinel/data_connector_dynamics_365.rs");
    include!("resources/sentinel/data_connector_iot.rs");
    include!("resources/sentinel/data_connector_microsoft_cloud_app_security.rs");
    include!(
        "resources/sentinel/data_connector_microsoft_defender_advanced_threat_protection.rs"
    );
    include!("resources/sentinel/data_connector_microsoft_threat_intelligence.rs");
    include!("resources/sentinel/data_connector_microsoft_threat_protection.rs");
    include!("resources/sentinel/data_connector_office_365.rs");
    include!("resources/sentinel/data_connector_office_365_project.rs");
    include!("resources/sentinel/data_connector_office_atp.rs");
    include!("resources/sentinel/data_connector_office_irm.rs");
    include!("resources/sentinel/data_connector_office_power_bi.rs");
    include!("resources/sentinel/data_connector_threat_intelligence.rs");
    include!("resources/sentinel/data_connector_threat_intelligence_taxii.rs");
    include!("resources/sentinel/log_analytics_workspace_onboarding.rs");
    include!("resources/sentinel/metadata.rs");
    include!("resources/sentinel/threat_intelligence_indicator.rs");
    include!("resources/sentinel/watchlist.rs");
    include!("resources/sentinel/watchlist_item.rs");
}
pub mod functions {
    pub mod sentinel {
        include!("functions/sentinel/get_alert_rule.rs");
        include!("functions/sentinel/get_alert_rule_anomaly.rs");
        include!("functions/sentinel/get_alert_rule_template.rs");
    }
}
pub mod types {
    pub mod sentinel {
        include!(
            "types/sentinel/alert_rule_anomaly_built_in_multi_select_observation.rs"
        );
        include!(
            "types/sentinel/alert_rule_anomaly_built_in_prioritized_exclude_observation.rs"
        );
        include!(
            "types/sentinel/alert_rule_anomaly_built_in_required_data_connector.rs"
        );
        include!(
            "types/sentinel/alert_rule_anomaly_built_in_single_select_observation.rs"
        );
        include!("types/sentinel/alert_rule_anomaly_built_in_threshold_observation.rs");
        include!(
            "types/sentinel/alert_rule_anomaly_duplicate_multi_select_observation.rs"
        );
        include!(
            "types/sentinel/alert_rule_anomaly_duplicate_prioritized_exclude_observation.rs"
        );
        include!(
            "types/sentinel/alert_rule_anomaly_duplicate_required_data_connector.rs"
        );
        include!(
            "types/sentinel/alert_rule_anomaly_duplicate_single_select_observation.rs"
        );
        include!("types/sentinel/alert_rule_anomaly_duplicate_threshold_observation.rs");
        include!("types/sentinel/alert_rule_fusion_source.rs");
        include!("types/sentinel/alert_rule_fusion_source_sub_type.rs");
        include!("types/sentinel/alert_rule_nrt_alert_details_override.rs");
        include!(
            "types/sentinel/alert_rule_nrt_alert_details_override_dynamic_property.rs"
        );
        include!("types/sentinel/alert_rule_nrt_entity_mapping.rs");
        include!("types/sentinel/alert_rule_nrt_entity_mapping_field_mapping.rs");
        include!("types/sentinel/alert_rule_nrt_event_grouping.rs");
        include!("types/sentinel/alert_rule_nrt_incident.rs");
        include!("types/sentinel/alert_rule_nrt_incident_grouping.rs");
        include!("types/sentinel/alert_rule_nrt_sentinel_entity_mapping.rs");
        include!("types/sentinel/alert_rule_scheduled_alert_details_override.rs");
        include!(
            "types/sentinel/alert_rule_scheduled_alert_details_override_dynamic_property.rs"
        );
        include!("types/sentinel/alert_rule_scheduled_entity_mapping.rs");
        include!("types/sentinel/alert_rule_scheduled_entity_mapping_field_mapping.rs");
        include!("types/sentinel/alert_rule_scheduled_event_grouping.rs");
        include!("types/sentinel/alert_rule_scheduled_incident.rs");
        include!("types/sentinel/alert_rule_scheduled_incident_grouping.rs");
        include!("types/sentinel/alert_rule_scheduled_sentinel_entity_mapping.rs");
        include!("types/sentinel/authomation_rule_action_incident.rs");
        include!("types/sentinel/authomation_rule_action_playbook.rs");
        include!("types/sentinel/automation_rule_action_incident.rs");
        include!("types/sentinel/automation_rule_action_playbook.rs");
        include!("types/sentinel/metadata_author.rs");
        include!("types/sentinel/metadata_category.rs");
        include!("types/sentinel/metadata_source.rs");
        include!("types/sentinel/metadata_support.rs");
        include!("types/sentinel/threat_intelligence_indicator_external_reference.rs");
        include!("types/sentinel/threat_intelligence_indicator_granular_marking.rs");
        include!("types/sentinel/threat_intelligence_indicator_kill_chain_phase.rs");
        include!("types/sentinel/threat_intelligence_indicator_parsed_pattern.rs");
        include!(
            "types/sentinel/threat_intelligence_indicator_parsed_pattern_pattern_type_value.rs"
        );
        include!("types/sentinel/get_alert_rule_anomaly_multi_select_observation.rs");
        include!(
            "types/sentinel/get_alert_rule_anomaly_prioritized_exclude_observation.rs"
        );
        include!("types/sentinel/get_alert_rule_anomaly_required_data_connector.rs");
        include!("types/sentinel/get_alert_rule_anomaly_single_select_observation.rs");
        include!("types/sentinel/get_alert_rule_anomaly_threshold_observation.rs");
        include!("types/sentinel/get_alert_rule_template_nrt_template.rs");
        include!("types/sentinel/get_alert_rule_template_scheduled_template.rs");
        include!("types/sentinel/get_alert_rule_template_security_incident_template.rs");
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
