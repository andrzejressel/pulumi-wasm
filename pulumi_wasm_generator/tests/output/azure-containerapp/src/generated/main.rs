pub mod containerapp {
    include!("resources/containerapp/app.rs");
    include!("resources/containerapp/custom_domain.rs");
    include!("resources/containerapp/environment.rs");
    include!("resources/containerapp/environment_certificate.rs");
    include!("resources/containerapp/environment_custom_domain.rs");
    include!("resources/containerapp/environment_dapr_component.rs");
    include!("resources/containerapp/environment_storage.rs");
    include!("resources/containerapp/job.rs");
}
pub mod functions {
    pub mod containerapp {
        include!("functions/containerapp/get_app.rs");
        include!("functions/containerapp/get_environment.rs");
        include!("functions/containerapp/get_environment_certificate.rs");
    }
}
pub mod types {
    pub mod containerapp {
        include!("types/containerapp/app_dapr.rs");
        include!("types/containerapp/app_identity.rs");
        include!("types/containerapp/app_ingress.rs");
        include!("types/containerapp/app_ingress_custom_domain.rs");
        include!("types/containerapp/app_ingress_ip_security_restriction.rs");
        include!("types/containerapp/app_ingress_traffic_weight.rs");
        include!("types/containerapp/app_registry.rs");
        include!("types/containerapp/app_secret.rs");
        include!("types/containerapp/app_template.rs");
        include!("types/containerapp/app_template_azure_queue_scale_rule.rs");
        include!(
            "types/containerapp/app_template_azure_queue_scale_rule_authentication.rs"
        );
        include!("types/containerapp/app_template_container.rs");
        include!("types/containerapp/app_template_container_env.rs");
        include!("types/containerapp/app_template_container_liveness_probe.rs");
        include!("types/containerapp/app_template_container_liveness_probe_header.rs");
        include!("types/containerapp/app_template_container_readiness_probe.rs");
        include!("types/containerapp/app_template_container_readiness_probe_header.rs");
        include!("types/containerapp/app_template_container_startup_probe.rs");
        include!("types/containerapp/app_template_container_startup_probe_header.rs");
        include!("types/containerapp/app_template_container_volume_mount.rs");
        include!("types/containerapp/app_template_custom_scale_rule.rs");
        include!("types/containerapp/app_template_custom_scale_rule_authentication.rs");
        include!("types/containerapp/app_template_http_scale_rule.rs");
        include!("types/containerapp/app_template_http_scale_rule_authentication.rs");
        include!("types/containerapp/app_template_init_container.rs");
        include!("types/containerapp/app_template_init_container_env.rs");
        include!("types/containerapp/app_template_init_container_volume_mount.rs");
        include!("types/containerapp/app_template_tcp_scale_rule.rs");
        include!("types/containerapp/app_template_tcp_scale_rule_authentication.rs");
        include!("types/containerapp/app_template_volume.rs");
        include!("types/containerapp/environment_dapr_component_metadata.rs");
        include!("types/containerapp/environment_dapr_component_secret.rs");
        include!("types/containerapp/environment_workload_profile.rs");
        include!("types/containerapp/job_event_trigger_config.rs");
        include!("types/containerapp/job_event_trigger_config_scale.rs");
        include!("types/containerapp/job_event_trigger_config_scale_rule.rs");
        include!(
            "types/containerapp/job_event_trigger_config_scale_rule_authentication.rs"
        );
        include!("types/containerapp/job_identity.rs");
        include!("types/containerapp/job_manual_trigger_config.rs");
        include!("types/containerapp/job_registry.rs");
        include!("types/containerapp/job_schedule_trigger_config.rs");
        include!("types/containerapp/job_secret.rs");
        include!("types/containerapp/job_template.rs");
        include!("types/containerapp/job_template_container.rs");
        include!("types/containerapp/job_template_container_env.rs");
        include!("types/containerapp/job_template_container_liveness_probe.rs");
        include!("types/containerapp/job_template_container_liveness_probe_header.rs");
        include!("types/containerapp/job_template_container_readiness_probe.rs");
        include!("types/containerapp/job_template_container_readiness_probe_header.rs");
        include!("types/containerapp/job_template_container_startup_probe.rs");
        include!("types/containerapp/job_template_container_startup_probe_header.rs");
        include!("types/containerapp/job_template_container_volume_mount.rs");
        include!("types/containerapp/job_template_init_container.rs");
        include!("types/containerapp/job_template_init_container_env.rs");
        include!("types/containerapp/job_template_init_container_volume_mount.rs");
        include!("types/containerapp/job_template_volume.rs");
        include!("types/containerapp/get_app_dapr.rs");
        include!("types/containerapp/get_app_identity.rs");
        include!("types/containerapp/get_app_ingress.rs");
        include!("types/containerapp/get_app_ingress_custom_domain.rs");
        include!("types/containerapp/get_app_ingress_ip_security_restriction.rs");
        include!("types/containerapp/get_app_ingress_traffic_weight.rs");
        include!("types/containerapp/get_app_registry.rs");
        include!("types/containerapp/get_app_secret.rs");
        include!("types/containerapp/get_app_template.rs");
        include!("types/containerapp/get_app_template_azure_queue_scale_rule.rs");
        include!(
            "types/containerapp/get_app_template_azure_queue_scale_rule_authentication.rs"
        );
        include!("types/containerapp/get_app_template_container.rs");
        include!("types/containerapp/get_app_template_container_env.rs");
        include!("types/containerapp/get_app_template_container_liveness_probe.rs");
        include!(
            "types/containerapp/get_app_template_container_liveness_probe_header.rs"
        );
        include!("types/containerapp/get_app_template_container_readiness_probe.rs");
        include!(
            "types/containerapp/get_app_template_container_readiness_probe_header.rs"
        );
        include!("types/containerapp/get_app_template_container_startup_probe.rs");
        include!(
            "types/containerapp/get_app_template_container_startup_probe_header.rs"
        );
        include!("types/containerapp/get_app_template_container_volume_mount.rs");
        include!("types/containerapp/get_app_template_custom_scale_rule.rs");
        include!(
            "types/containerapp/get_app_template_custom_scale_rule_authentication.rs"
        );
        include!("types/containerapp/get_app_template_http_scale_rule.rs");
        include!(
            "types/containerapp/get_app_template_http_scale_rule_authentication.rs"
        );
        include!("types/containerapp/get_app_template_init_container.rs");
        include!("types/containerapp/get_app_template_init_container_env.rs");
        include!("types/containerapp/get_app_template_init_container_volume_mount.rs");
        include!("types/containerapp/get_app_template_tcp_scale_rule.rs");
        include!("types/containerapp/get_app_template_tcp_scale_rule_authentication.rs");
        include!("types/containerapp/get_app_template_volume.rs");
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
