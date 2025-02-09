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
pub mod portal {
    include!("resources/portal/portal_dashboard.rs");
}
pub mod postgresql {
    include!("resources/postgresql/active_directory_administrator.rs");
    include!("resources/postgresql/configuration.rs");
    include!("resources/postgresql/database.rs");
    include!("resources/postgresql/firewall_rule.rs");
    include!("resources/postgresql/flexible_server.rs");
    include!("resources/postgresql/flexible_server_active_directory_administrator.rs");
    include!("resources/postgresql/flexible_server_configuration.rs");
    include!("resources/postgresql/flexible_server_database.rs");
    include!("resources/postgresql/flexible_server_firewall_rule.rs");
    include!("resources/postgresql/flexible_server_virtual_endpoint.rs");
    include!("resources/postgresql/server.rs");
    include!("resources/postgresql/server_key.rs");
    include!("resources/postgresql/virtual_network_rule.rs");
}
pub mod powerbi {
    include!("resources/powerbi/embedded.rs");
}
pub mod privatedns {
    include!("resources/privatedns/aaaa_record.rs");
    include!("resources/privatedns/a_record.rs");
    include!("resources/privatedns/cname_record.rs");
    include!("resources/privatedns/link_service.rs");
    include!("resources/privatedns/mx_record.rs");
    include!("resources/privatedns/ptr_record.rs");
    include!("resources/privatedns/resolver.rs");
    include!("resources/privatedns/resolver_dns_forwarding_ruleset.rs");
    include!("resources/privatedns/resolver_forwarding_rule.rs");
    include!("resources/privatedns/resolver_inbound_endpoint.rs");
    include!("resources/privatedns/resolver_outbound_endpoint.rs");
    include!("resources/privatedns/resolver_virtual_network_link.rs");
    include!("resources/privatedns/srv_record.rs");
    include!("resources/privatedns/txt_record.rs");
    include!("resources/privatedns/zone.rs");
    include!("resources/privatedns/zone_virtual_network_link.rs");
}
pub mod privatelink {
    include!("resources/privatelink/application_security_group_association.rs");
    include!("resources/privatelink/endpoint.rs");
}
pub mod proximity {
    include!("resources/proximity/placement_group.rs");
}
pub mod purview {
    include!("resources/purview/account.rs");
}
pub mod functions {
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
    pub mod portal {
        include!("functions/portal/azurerm_portal_dashboard.rs");
    }
    pub mod postgresql {
        include!("functions/postgresql/get_flexible_server.rs");
        include!("functions/postgresql/get_server.rs");
    }
    pub mod privatedns {
        include!("functions/privatedns/get_aaaa_record.rs");
        include!("functions/privatedns/get_a_record.rs");
        include!("functions/privatedns/get_cname_record.rs");
        include!("functions/privatedns/get_dns_zone.rs");
        include!("functions/privatedns/get_mx_record.rs");
        include!("functions/privatedns/get_ptr_record.rs");
        include!("functions/privatedns/get_resolver.rs");
        include!("functions/privatedns/get_resolver_dns_forwarding_ruleset.rs");
        include!("functions/privatedns/get_resolver_forwarding_rule.rs");
        include!("functions/privatedns/get_resolver_inbound_endpoint.rs");
        include!("functions/privatedns/get_resolver_outbound_endpoint.rs");
        include!("functions/privatedns/get_resolver_virtual_network_link.rs");
        include!("functions/privatedns/get_soa_record.rs");
        include!("functions/privatedns/get_srv_record.rs");
        include!("functions/privatedns/get_txt_record.rs");
        include!("functions/privatedns/get_zone_virtual_network_link.rs");
    }
    pub mod privatelink {
        include!("functions/privatelink/get_endpoint_connection.rs");
        include!("functions/privatelink/get_service.rs");
        include!("functions/privatelink/get_service_endpoint_connections.rs");
    }
    pub mod proximity {
        include!("functions/proximity/get_placement_group.rs");
    }
}
pub mod types {
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
    pub mod postgresql {
        include!("types/postgresql/flexible_server_authentication.rs");
        include!("types/postgresql/flexible_server_customer_managed_key.rs");
        include!("types/postgresql/flexible_server_high_availability.rs");
        include!("types/postgresql/flexible_server_identity.rs");
        include!("types/postgresql/flexible_server_maintenance_window.rs");
        include!("types/postgresql/server_identity.rs");
        include!("types/postgresql/server_threat_detection_policy.rs");
        include!("types/postgresql/get_server_identity.rs");
    }
    pub mod privatedns {
        include!("types/privatedns/link_service_nat_ip_configuration.rs");
        include!("types/privatedns/mx_record_record.rs");
        include!("types/privatedns/resolver_forwarding_rule_target_dns_server.rs");
        include!("types/privatedns/resolver_inbound_endpoint_ip_configurations.rs");
        include!("types/privatedns/srv_record_record.rs");
        include!("types/privatedns/txt_record_record.rs");
        include!("types/privatedns/zone_soa_record.rs");
        include!("types/privatedns/get_mx_record_record.rs");
        include!("types/privatedns/get_resolver_forwarding_rule_target_dns_server.rs");
        include!("types/privatedns/get_resolver_inbound_endpoint_ip_configuration.rs");
        include!("types/privatedns/get_srv_record_record.rs");
        include!("types/privatedns/get_txt_record_record.rs");
    }
    pub mod privatelink {
        include!("types/privatelink/endpoint_custom_dns_config.rs");
        include!("types/privatelink/endpoint_ip_configuration.rs");
        include!("types/privatelink/endpoint_network_interface.rs");
        include!("types/privatelink/endpoint_private_dns_zone_config.rs");
        include!("types/privatelink/endpoint_private_dns_zone_config_record_set.rs");
        include!("types/privatelink/endpoint_private_dns_zone_group.rs");
        include!("types/privatelink/endpoint_private_service_connection.rs");
        include!("types/privatelink/get_endpoint_connection_network_interface.rs");
        include!(
            "types/privatelink/get_endpoint_connection_private_service_connection.rs"
        );
        include!(
            "types/privatelink/get_service_endpoint_connections_private_endpoint_connection.rs"
        );
        include!("types/privatelink/get_service_nat_ip_configuration.rs");
    }
    pub mod purview {
        include!("types/purview/account_identity.rs");
        include!("types/purview/account_managed_resource.rs");
    }
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    pulumi_gestalt_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-gestalt@0.0.0-DEV;

world world-azure {
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
        clone: func() -> output;
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
        with : { "component:pulumi-gestalt/output-interface@0.0.0-DEV" :
        pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::output_interface
        } }
    );
}
#[link_section = "pulumi_gestalt_provider::azure"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_AZURE: [u8; 45] = *b"{\"version\":\"6.14.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "6.14.0".to_string()
}
