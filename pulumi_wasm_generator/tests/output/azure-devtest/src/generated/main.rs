pub mod devtest {
    include!("resources/devtest/global_vm_shutdown_schedule.rs");
    include!("resources/devtest/lab.rs");
    include!("resources/devtest/linux_virtual_machine.rs");
    include!("resources/devtest/policy.rs");
    include!("resources/devtest/schedule.rs");
    include!("resources/devtest/virtual_network.rs");
    include!("resources/devtest/windows_virtual_machine.rs");
}
pub mod functions {
    pub mod devtest {
        include!("functions/devtest/get_lab.rs");
        include!("functions/devtest/get_virtual_network.rs");
    }
}
pub mod types {
    pub mod devtest {
        include!("types/devtest/global_vm_shutdown_schedule_notification_settings.rs");
        include!("types/devtest/linux_virtual_machine_gallery_image_reference.rs");
        include!("types/devtest/linux_virtual_machine_inbound_nat_rule.rs");
        include!("types/devtest/schedule_daily_recurrence.rs");
        include!("types/devtest/schedule_hourly_recurrence.rs");
        include!("types/devtest/schedule_notification_settings.rs");
        include!("types/devtest/schedule_weekly_recurrence.rs");
        include!("types/devtest/virtual_network_subnet.rs");
        include!("types/devtest/virtual_network_subnet_shared_public_ip_address.rs");
        include!(
            "types/devtest/virtual_network_subnet_shared_public_ip_address_allowed_port.rs"
        );
        include!("types/devtest/windows_virtual_machine_gallery_image_reference.rs");
        include!("types/devtest/windows_virtual_machine_inbound_nat_rule.rs");
        include!("types/devtest/get_virtual_network_allowed_subnet.rs");
        include!("types/devtest/get_virtual_network_subnet_override.rs");
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
