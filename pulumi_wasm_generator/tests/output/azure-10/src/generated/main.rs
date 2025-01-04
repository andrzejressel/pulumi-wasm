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
pub mod recoveryservices {
    include!("resources/recoveryservices/vault.rs");
    include!("resources/recoveryservices/vault_resource_guard_association.rs");
}
pub mod redhatopenshift {
    include!("resources/redhatopenshift/cluster.rs");
}
pub mod redis {
    include!("resources/redis/cache.rs");
    include!("resources/redis/cache_access_policy.rs");
    include!("resources/redis/cache_access_policy_assignment.rs");
    include!("resources/redis/enterprise_cluster.rs");
    include!("resources/redis/enterprise_database.rs");
    include!("resources/redis/firewall_rule.rs");
    include!("resources/redis/linked_server.rs");
}
pub mod functions {
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
    pub mod recoveryservices {
        include!("functions/recoveryservices/get_vault.rs");
    }
    pub mod redis {
        include!("functions/redis/get_cache.rs");
        include!("functions/redis/get_enterprise_database.rs");
    }
}
pub mod types {
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
    pub mod recoveryservices {
        include!("types/recoveryservices/vault_encryption.rs");
        include!("types/recoveryservices/vault_identity.rs");
        include!("types/recoveryservices/vault_monitoring.rs");
        include!("types/recoveryservices/get_vault_identity.rs");
    }
    pub mod redhatopenshift {
        include!("types/redhatopenshift/cluster_api_server_profile.rs");
        include!("types/redhatopenshift/cluster_cluster_profile.rs");
        include!("types/redhatopenshift/cluster_ingress_profile.rs");
        include!("types/redhatopenshift/cluster_main_profile.rs");
        include!("types/redhatopenshift/cluster_network_profile.rs");
        include!("types/redhatopenshift/cluster_service_principal.rs");
        include!("types/redhatopenshift/cluster_worker_profile.rs");
    }
    pub mod redis {
        include!("types/redis/cache_identity.rs");
        include!("types/redis/cache_patch_schedule.rs");
        include!("types/redis/cache_redis_configuration.rs");
        include!("types/redis/enterprise_database_module.rs");
        include!("types/redis/get_cache_patch_schedule.rs");
        include!("types/redis/get_cache_redis_configuration.rs");
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
