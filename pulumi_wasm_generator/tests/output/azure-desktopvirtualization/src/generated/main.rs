pub mod desktopvirtualization {
    include!("resources/desktopvirtualization/application.rs");
    include!("resources/desktopvirtualization/application_group.rs");
    include!("resources/desktopvirtualization/host_pool.rs");
    include!("resources/desktopvirtualization/scaling_plan.rs");
    include!("resources/desktopvirtualization/scaling_plan_host_pool_association.rs");
    include!("resources/desktopvirtualization/workspace.rs");
    include!(
        "resources/desktopvirtualization/workspace_application_group_association.rs"
    );
    include!("resources/desktopvirtualization/get_host_pool_registration_info.rs");
}
pub mod functions {
    pub mod desktopvirtualization {
        include!("functions/desktopvirtualization/get_application_group.rs");
        include!("functions/desktopvirtualization/get_host_pool.rs");
        include!("functions/desktopvirtualization/get_workspace.rs");
    }
}
pub mod types {
    pub mod desktopvirtualization {
        include!("types/desktopvirtualization/host_pool_scheduled_agent_updates.rs");
        include!(
            "types/desktopvirtualization/host_pool_scheduled_agent_updates_schedule.rs"
        );
        include!("types/desktopvirtualization/scaling_plan_host_pool.rs");
        include!("types/desktopvirtualization/scaling_plan_schedule.rs");
        include!("types/desktopvirtualization/get_host_pool_scheduled_agent_update.rs");
        include!(
            "types/desktopvirtualization/get_host_pool_scheduled_agent_update_schedule.rs"
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
