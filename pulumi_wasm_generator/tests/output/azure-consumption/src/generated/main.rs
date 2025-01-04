pub mod consumption {
    include!("resources/consumption/budget_management_group.rs");
    include!("resources/consumption/budget_resource_group.rs");
    include!("resources/consumption/budget_subscription.rs");
}
pub mod functions {
    pub mod consumption {
        include!("functions/consumption/get_budget_resource_group.rs");
        include!("functions/consumption/get_budget_subscription.rs");
    }
}
pub mod types {
    pub mod consumption {
        include!("types/consumption/budget_management_group_filter.rs");
        include!("types/consumption/budget_management_group_filter_dimension.rs");
        include!("types/consumption/budget_management_group_filter_tag.rs");
        include!("types/consumption/budget_management_group_notification.rs");
        include!("types/consumption/budget_management_group_time_period.rs");
        include!("types/consumption/budget_resource_group_filter.rs");
        include!("types/consumption/budget_resource_group_filter_dimension.rs");
        include!("types/consumption/budget_resource_group_filter_tag.rs");
        include!("types/consumption/budget_resource_group_notification.rs");
        include!("types/consumption/budget_resource_group_time_period.rs");
        include!("types/consumption/budget_subscription_filter.rs");
        include!("types/consumption/budget_subscription_filter_dimension.rs");
        include!("types/consumption/budget_subscription_filter_tag.rs");
        include!("types/consumption/budget_subscription_notification.rs");
        include!("types/consumption/budget_subscription_time_period.rs");
        include!("types/consumption/get_budget_resource_group_filter.rs");
        include!("types/consumption/get_budget_resource_group_filter_dimension.rs");
        include!("types/consumption/get_budget_resource_group_filter_not.rs");
        include!("types/consumption/get_budget_resource_group_filter_not_dimension.rs");
        include!("types/consumption/get_budget_resource_group_filter_not_tag.rs");
        include!("types/consumption/get_budget_resource_group_filter_tag.rs");
        include!("types/consumption/get_budget_resource_group_notification.rs");
        include!("types/consumption/get_budget_resource_group_time_period.rs");
        include!("types/consumption/get_budget_subscription_filter.rs");
        include!("types/consumption/get_budget_subscription_filter_dimension.rs");
        include!("types/consumption/get_budget_subscription_filter_not.rs");
        include!("types/consumption/get_budget_subscription_filter_not_dimension.rs");
        include!("types/consumption/get_budget_subscription_filter_not_tag.rs");
        include!("types/consumption/get_budget_subscription_filter_tag.rs");
        include!("types/consumption/get_budget_subscription_notification.rs");
        include!("types/consumption/get_budget_subscription_time_period.rs");
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
