pub mod bot {
    include!("resources/bot/channel_alexa.rs");
    include!("resources/bot/channel_direct_line.rs");
    include!("resources/bot/channel_direct_line_speech.rs");
    include!("resources/bot/channel_email.rs");
    include!("resources/bot/channel_facebook.rs");
    include!("resources/bot/channel_line.rs");
    include!("resources/bot/channel_slack.rs");
    include!("resources/bot/channel_sms.rs");
    include!("resources/bot/channel_teams.rs");
    include!("resources/bot/channel_web_chat.rs");
    include!("resources/bot/channels_registration.rs");
    include!("resources/bot/connection.rs");
    include!("resources/bot/healthbot.rs");
    include!("resources/bot/service_azure_bot.rs");
    include!("resources/bot/web_app.rs");
}
pub mod functions {}
pub mod types {
    pub mod bot {
        include!("types/bot/channel_direct_line_site.rs");
        include!("types/bot/channel_facebook_page.rs");
        include!("types/bot/channel_line_line_channel.rs");
        include!("types/bot/channel_web_chat_site.rs");
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
