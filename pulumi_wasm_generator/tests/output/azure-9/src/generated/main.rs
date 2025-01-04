pub mod networkfunction {
    include!("resources/networkfunction/azure_traffic_collector.rs");
    include!("resources/networkfunction/collector_policy.rs");
}
pub mod newrelic {
    include!("resources/newrelic/monitor.rs");
    include!("resources/newrelic/tag_rule.rs");
}
pub mod nginx {
    include!("resources/nginx/certificate.rs");
    include!("resources/nginx/configuration.rs");
    include!("resources/nginx/deployment.rs");
}
pub mod notificationhub {
    include!("resources/notificationhub/authorization_rule.rs");
    include!("resources/notificationhub/hub.rs");
    include!("resources/notificationhub/namespace.rs");
}
pub mod operationalinsights {
    include!("resources/operationalinsights/analytics_solution.rs");
    include!("resources/operationalinsights/analytics_workspace.rs");
    include!("resources/operationalinsights/query_pack_query.rs");
}
pub mod oracle {
    include!("resources/oracle/autonomous_database.rs");
    include!("resources/oracle/cloud_vm_cluster.rs");
    include!("resources/oracle/exadata_infrastructure.rs");
}
pub mod orbital {
    include!("resources/orbital/contact.rs");
    include!("resources/orbital/contact_profile.rs");
    include!("resources/orbital/spacecraft.rs");
}
pub mod paloalto {
    include!("resources/paloalto/local_rulestack.rs");
    include!("resources/paloalto/local_rulestack_certificate.rs");
    include!("resources/paloalto/local_rulestack_fqdn_list.rs");
    include!(
        "resources/paloalto/local_rulestack_outbound_trust_certificate_association.rs"
    );
    include!(
        "resources/paloalto/local_rulestack_outbound_untrust_certificate_association.rs"
    );
    include!("resources/paloalto/local_rulestack_prefix_list.rs");
    include!("resources/paloalto/local_rulestack_rule.rs");
    include!(
        "resources/paloalto/next_generation_firewall_virtual_hub_local_rulestack.rs"
    );
    include!("resources/paloalto/next_generation_firewall_virtual_hub_panorama.rs");
    include!(
        "resources/paloalto/next_generation_firewall_virtual_network_local_rulestack.rs"
    );
    include!("resources/paloalto/next_generation_firewall_virtual_network_panorama.rs");
    include!("resources/paloalto/virtual_network_appliance.rs");
}
pub mod pim {
    include!("resources/pim/active_role_assignment.rs");
    include!("resources/pim/eligible_role_assignment.rs");
    include!("resources/pim/role_management_policy.rs");
}
pub mod policy {
    include!("resources/policy/definition.rs");
    include!("resources/policy/policy_set_definition.rs");
    include!("resources/policy/virtual_machine_configuration_assignment.rs");
}
pub mod functions {
    pub mod nginx {
        include!("functions/nginx/get_certificate.rs");
        include!("functions/nginx/get_configuration.rs");
        include!("functions/nginx/get_deployment.rs");
    }
    pub mod notificationhub {
        include!("functions/notificationhub/get_hub.rs");
        include!("functions/notificationhub/get_namespace.rs");
    }
    pub mod operationalinsights {
        include!("functions/operationalinsights/get_analytics_workspace.rs");
    }
    pub mod oracle {
        include!("functions/oracle/get_adbs_character_sets.rs");
        include!("functions/oracle/get_adbs_national_character_sets.rs");
        include!("functions/oracle/get_autonomous_database.rs");
        include!("functions/oracle/get_cloud_vm_cluster.rs");
        include!("functions/oracle/get_db_nodes.rs");
        include!("functions/oracle/get_db_servers.rs");
        include!("functions/oracle/get_db_system_shapes.rs");
        include!("functions/oracle/get_exadata_infrastructure.rs");
        include!("functions/oracle/get_gi_versions.rs");
    }
    pub mod paloalto {
        include!("functions/paloalto/get_local_rulestack.rs");
    }
    pub mod pim {
        include!("functions/pim/get_role_management_policy.rs");
    }
    pub mod policy {
        include!("functions/policy/get_policy_assignment.rs");
        include!("functions/policy/get_policy_defintion.rs");
        include!("functions/policy/get_policy_defintion_built_in.rs");
        include!("functions/policy/get_policy_set_definition.rs");
        include!("functions/policy/get_virtual_machine_configuration_assignment.rs");
    }
}
pub mod types {
    pub mod networkfunction {
        include!("types/networkfunction/collector_policy_ipfx_emission.rs");
        include!("types/networkfunction/collector_policy_ipfx_ingestion.rs");
    }
    pub mod newrelic {
        include!("types/newrelic/monitor_identity.rs");
        include!("types/newrelic/monitor_plan.rs");
        include!("types/newrelic/monitor_user.rs");
        include!("types/newrelic/tag_rule_log_tag_filter.rs");
        include!("types/newrelic/tag_rule_metric_tag_filter.rs");
    }
    pub mod nginx {
        include!("types/nginx/configuration_config_file.rs");
        include!("types/nginx/configuration_protected_file.rs");
        include!("types/nginx/deployment_auto_scale_profile.rs");
        include!("types/nginx/deployment_frontend_private.rs");
        include!("types/nginx/deployment_frontend_public.rs");
        include!("types/nginx/deployment_identity.rs");
        include!("types/nginx/deployment_logging_storage_account.rs");
        include!("types/nginx/deployment_network_interface.rs");
        include!("types/nginx/get_configuration_config_file.rs");
        include!("types/nginx/get_configuration_protected_file.rs");
        include!("types/nginx/get_deployment_auto_scale_profile.rs");
        include!("types/nginx/get_deployment_frontend_private.rs");
        include!("types/nginx/get_deployment_frontend_public.rs");
        include!("types/nginx/get_deployment_identity.rs");
        include!("types/nginx/get_deployment_logging_storage_account.rs");
        include!("types/nginx/get_deployment_network_interface.rs");
    }
    pub mod notificationhub {
        include!("types/notificationhub/hub_apns_credential.rs");
        include!("types/notificationhub/hub_browser_credential.rs");
        include!("types/notificationhub/hub_gcm_credential.rs");
        include!("types/notificationhub/get_hub_apns_credential.rs");
        include!("types/notificationhub/get_hub_gcm_credential.rs");
        include!("types/notificationhub/get_namespace_sku.rs");
    }
    pub mod operationalinsights {
        include!("types/operationalinsights/analytics_solution_plan.rs");
        include!("types/operationalinsights/analytics_workspace_identity.rs");
    }
    pub mod oracle {
        include!("types/oracle/cloud_vm_cluster_data_collection_options.rs");
        include!("types/oracle/exadata_infrastructure_maintenance_window.rs");
        include!("types/oracle/get_adbs_character_sets_character_set.rs");
        include!("types/oracle/get_adbs_national_character_sets_character_set.rs");
        include!("types/oracle/get_cloud_vm_cluster_data_collection_option.rs");
        include!("types/oracle/get_cloud_vm_cluster_iorm_config_cach.rs");
        include!("types/oracle/get_cloud_vm_cluster_iorm_config_cach_db_plan.rs");
        include!("types/oracle/get_db_nodes_db_node.rs");
        include!("types/oracle/get_db_servers_db_server.rs");
        include!("types/oracle/get_db_system_shapes_db_system_shape.rs");
        include!("types/oracle/get_exadata_infrastructure_estimated_patching_time.rs");
        include!("types/oracle/get_exadata_infrastructure_maintenance_window.rs");
    }
    pub mod orbital {
        include!("types/orbital/contact_profile_link.rs");
        include!("types/orbital/contact_profile_link_channel.rs");
        include!("types/orbital/contact_profile_link_channel_end_point.rs");
        include!("types/orbital/spacecraft_link.rs");
    }
    pub mod paloalto {
        include!("types/paloalto/local_rulestack_rule_category.rs");
        include!("types/paloalto/local_rulestack_rule_destination.rs");
        include!("types/paloalto/local_rulestack_rule_source.rs");
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_local_rulestack_destination_nat.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_local_rulestack_destination_nat_backend_config.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_local_rulestack_destination_nat_frontend_config.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_local_rulestack_dns_settings.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_local_rulestack_network_profile.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_panorama_destination_nat.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_panorama_destination_nat_backend_config.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_panorama_destination_nat_frontend_config.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_panorama_dns_settings.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_panorama_network_profile.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_panorama_panorama.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_local_rulestack_destination_nat.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_local_rulestack_destination_nat_backend_config.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_local_rulestack_destination_nat_frontend_config.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_local_rulestack_dns_settings.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_local_rulestack_network_profile.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_local_rulestack_network_profile_vnet_configuration.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_panorama_destination_nat.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_panorama_destination_nat_backend_config.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_panorama_destination_nat_frontend_config.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_panorama_dns_settings.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_panorama_network_profile.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_panorama_network_profile_vnet_configuration.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_panorama_panorama.rs"
        );
    }
    pub mod pim {
        include!("types/pim/active_role_assignment_schedule.rs");
        include!("types/pim/active_role_assignment_schedule_expiration.rs");
        include!("types/pim/active_role_assignment_ticket.rs");
        include!("types/pim/eligible_role_assignment_schedule.rs");
        include!("types/pim/eligible_role_assignment_schedule_expiration.rs");
        include!("types/pim/eligible_role_assignment_ticket.rs");
        include!("types/pim/role_management_policy_activation_rules.rs");
        include!("types/pim/role_management_policy_activation_rules_approval_stage.rs");
        include!(
            "types/pim/role_management_policy_activation_rules_approval_stage_primary_approver.rs"
        );
        include!("types/pim/role_management_policy_active_assignment_rules.rs");
        include!("types/pim/role_management_policy_eligible_assignment_rules.rs");
        include!("types/pim/role_management_policy_notification_rules.rs");
        include!(
            "types/pim/role_management_policy_notification_rules_active_assignments.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_active_assignments_admin_notifications.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_active_assignments_approver_notifications.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_active_assignments_assignee_notifications.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_eligible_activations.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_eligible_activations_admin_notifications.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_eligible_activations_approver_notifications.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_eligible_activations_assignee_notifications.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_eligible_assignments.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_eligible_assignments_admin_notifications.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_eligible_assignments_approver_notifications.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_eligible_assignments_assignee_notifications.rs"
        );
        include!("types/pim/get_role_management_policy_activation_rule.rs");
        include!(
            "types/pim/get_role_management_policy_activation_rule_approval_stage.rs"
        );
        include!(
            "types/pim/get_role_management_policy_activation_rule_approval_stage_primary_approver.rs"
        );
        include!("types/pim/get_role_management_policy_active_assignment_rule.rs");
        include!("types/pim/get_role_management_policy_eligible_assignment_rule.rs");
        include!("types/pim/get_role_management_policy_notification_rule.rs");
        include!(
            "types/pim/get_role_management_policy_notification_rule_active_assignment.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_active_assignment_admin_notification.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_active_assignment_approver_notification.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_active_assignment_assignee_notification.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_eligible_activation.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_eligible_activation_admin_notification.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_eligible_activation_approver_notification.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_eligible_activation_assignee_notification.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_eligible_assignment.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_eligible_assignment_admin_notification.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_eligible_assignment_approver_notification.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_eligible_assignment_assignee_notification.rs"
        );
    }
    pub mod policy {
        include!("types/policy/policy_set_definition_policy_definition_group.rs");
        include!("types/policy/policy_set_definition_policy_definition_reference.rs");
        include!(
            "types/policy/virtual_machine_configuration_assignment_configuration.rs"
        );
        include!(
            "types/policy/virtual_machine_configuration_assignment_configuration_parameter.rs"
        );
        include!("types/policy/get_policy_assignment_identity.rs");
        include!("types/policy/get_policy_assignment_non_compliance_message.rs");
        include!("types/policy/get_policy_set_definition_policy_definition_group.rs");
        include!(
            "types/policy/get_policy_set_definition_policy_definition_reference.rs"
        );
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
