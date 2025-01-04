pub mod synapse {
    include!("resources/synapse/firewall_rule.rs");
    include!("resources/synapse/integration_runtime_azure.rs");
    include!("resources/synapse/integration_runtime_self_hosted.rs");
    include!("resources/synapse/linked_service.rs");
    include!("resources/synapse/managed_private_endpoint.rs");
    include!("resources/synapse/private_link_hub.rs");
    include!("resources/synapse/role_assignment.rs");
    include!("resources/synapse/spark_pool.rs");
    include!("resources/synapse/sql_pool.rs");
    include!("resources/synapse/sql_pool_extended_auditing_policy.rs");
    include!("resources/synapse/sql_pool_security_alert_policy.rs");
    include!("resources/synapse/sql_pool_vulnerability_assessment.rs");
    include!("resources/synapse/sql_pool_vulnerability_assessment_baseline.rs");
    include!("resources/synapse/sql_pool_workload_classifier.rs");
    include!("resources/synapse/sql_pool_workload_group.rs");
    include!("resources/synapse/workspace.rs");
    include!("resources/synapse/workspace_aad_admin.rs");
    include!("resources/synapse/workspace_extended_auditing_policy.rs");
    include!("resources/synapse/workspace_key.rs");
    include!("resources/synapse/workspace_security_alert_policy.rs");
    include!("resources/synapse/workspace_sql_aad_admin.rs");
    include!("resources/synapse/workspace_vulnerability_assessment.rs");
}
pub mod functions {
    pub mod synapse {
        include!("functions/synapse/get_workspace.rs");
    }
}
pub mod types {
    pub mod synapse {
        include!("types/synapse/linked_service_integration_runtime.rs");
        include!("types/synapse/spark_pool_auto_pause.rs");
        include!("types/synapse/spark_pool_auto_scale.rs");
        include!("types/synapse/spark_pool_library_requirement.rs");
        include!("types/synapse/spark_pool_spark_config.rs");
        include!("types/synapse/sql_pool_restore.rs");
        include!("types/synapse/sql_pool_vulnerability_assessment_baseline_baseline.rs");
        include!("types/synapse/sql_pool_vulnerability_assessment_recurring_scans.rs");
        include!("types/synapse/workspace_azure_devops_repo.rs");
        include!("types/synapse/workspace_customer_managed_key.rs");
        include!("types/synapse/workspace_github_repo.rs");
        include!("types/synapse/workspace_identity.rs");
        include!("types/synapse/workspace_vulnerability_assessment_recurring_scans.rs");
        include!("types/synapse/get_workspace_identity.rs");
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
