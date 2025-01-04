pub mod core {
    include!("resources/core/custom_provider.rs");
    include!("resources/core/portal_tenant_configuration.rs");
    include!("resources/core/resource_deployment_script_azure_cli.rs");
    include!("resources/core/resource_deployment_script_power_shell.rs");
    include!("resources/core/resource_group.rs");
    include!("resources/core/resource_group_cost_management_export.rs");
    include!("resources/core/resource_group_cost_management_view.rs");
    include!("resources/core/resource_group_policy_assignment.rs");
    include!("resources/core/resource_group_policy_exemption.rs");
    include!("resources/core/resource_group_policy_remediation.rs");
    include!("resources/core/resource_group_template_deployment.rs");
    include!("resources/core/resource_policy_assignment.rs");
    include!("resources/core/resource_policy_exemption.rs");
    include!("resources/core/resource_policy_remediation.rs");
    include!("resources/core/resource_provider_registration.rs");
    include!("resources/core/subscription.rs");
    include!("resources/core/subscription_cost_management_export.rs");
    include!("resources/core/subscription_cost_management_view.rs");
    include!("resources/core/subscription_policy_assignment.rs");
    include!("resources/core/subscription_policy_exemption.rs");
    include!("resources/core/subscription_policy_remediation.rs");
    include!("resources/core/subscription_template_deployment.rs");
    include!("resources/core/tenant_template_deployment.rs");
}
pub mod functions {
    pub mod core {
        include!("functions/core/get_client_config.rs");
        include!("functions/core/get_extended_locations.rs");
        include!("functions/core/get_location.rs");
        include!("functions/core/get_resource_group.rs");
        include!("functions/core/get_resource_group_template_deployment.rs");
        include!("functions/core/get_resources.rs");
        include!("functions/core/get_subscription.rs");
        include!("functions/core/get_subscription_template_deployment.rs");
        include!("functions/core/get_subscriptions.rs");
        include!("functions/core/get_template_spec_version.rs");
        include!("functions/core/get_tenant_template_deployment.rs");
        include!("functions/core/get_user_assigned_identity.rs");
    }
}
pub mod types {
    pub mod core {
        include!("types/core/custom_provider_action.rs");
        include!("types/core/custom_provider_resource_type.rs");
        include!("types/core/custom_provider_validation.rs");
        include!("types/core/resource_deployment_script_azure_cli_container.rs");
        include!(
            "types/core/resource_deployment_script_azure_cli_environment_variable.rs"
        );
        include!("types/core/resource_deployment_script_azure_cli_identity.rs");
        include!("types/core/resource_deployment_script_azure_cli_storage_account.rs");
        include!("types/core/resource_deployment_script_power_shell_container.rs");
        include!(
            "types/core/resource_deployment_script_power_shell_environment_variable.rs"
        );
        include!("types/core/resource_deployment_script_power_shell_identity.rs");
        include!("types/core/resource_deployment_script_power_shell_storage_account.rs");
        include!(
            "types/core/resource_group_cost_management_export_export_data_options.rs"
        );
        include!(
            "types/core/resource_group_cost_management_export_export_data_storage_location.rs"
        );
        include!("types/core/resource_group_cost_management_view_dataset.rs");
        include!(
            "types/core/resource_group_cost_management_view_dataset_aggregation.rs"
        );
        include!("types/core/resource_group_cost_management_view_dataset_grouping.rs");
        include!("types/core/resource_group_cost_management_view_dataset_sorting.rs");
        include!("types/core/resource_group_cost_management_view_kpi.rs");
        include!("types/core/resource_group_cost_management_view_pivot.rs");
        include!("types/core/resource_group_policy_assignment_identity.rs");
        include!(
            "types/core/resource_group_policy_assignment_non_compliance_message.rs"
        );
        include!("types/core/resource_group_policy_assignment_override.rs");
        include!("types/core/resource_group_policy_assignment_override_selector.rs");
        include!("types/core/resource_group_policy_assignment_resource_selector.rs");
        include!(
            "types/core/resource_group_policy_assignment_resource_selector_selector.rs"
        );
        include!("types/core/resource_policy_assignment_identity.rs");
        include!("types/core/resource_policy_assignment_non_compliance_message.rs");
        include!("types/core/resource_policy_assignment_override.rs");
        include!("types/core/resource_policy_assignment_override_selector.rs");
        include!("types/core/resource_policy_assignment_resource_selector.rs");
        include!("types/core/resource_policy_assignment_resource_selector_selector.rs");
        include!("types/core/resource_provider_registration_feature.rs");
        include!(
            "types/core/subscription_cost_management_export_export_data_options.rs"
        );
        include!(
            "types/core/subscription_cost_management_export_export_data_storage_location.rs"
        );
        include!("types/core/subscription_cost_management_view_dataset.rs");
        include!("types/core/subscription_cost_management_view_dataset_aggregation.rs");
        include!("types/core/subscription_cost_management_view_dataset_grouping.rs");
        include!("types/core/subscription_cost_management_view_dataset_sorting.rs");
        include!("types/core/subscription_cost_management_view_kpi.rs");
        include!("types/core/subscription_cost_management_view_pivot.rs");
        include!("types/core/subscription_policy_assignment_identity.rs");
        include!("types/core/subscription_policy_assignment_non_compliance_message.rs");
        include!("types/core/subscription_policy_assignment_override.rs");
        include!("types/core/subscription_policy_assignment_override_selector.rs");
        include!("types/core/subscription_policy_assignment_resource_selector.rs");
        include!(
            "types/core/subscription_policy_assignment_resource_selector_selector.rs"
        );
        include!("types/core/get_location_zone_mapping.rs");
        include!("types/core/get_resources_resource.rs");
        include!("types/core/get_subscriptions_subscription.rs");
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
