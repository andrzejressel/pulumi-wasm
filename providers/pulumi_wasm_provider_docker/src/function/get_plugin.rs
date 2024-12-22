use std::collections::HashMap;
use crate::bindings::exports::pulumi::docker::get_plugin;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_plugin::Guest for Component {
    fn invoke(
        args: get_plugin::Args
    ) -> get_plugin::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "docker:index/getPlugin:getPlugin".into(),
            object: vec![
                ObjectField { name: "alias".into(), value: args.alias },
                ObjectField { name: "id".into(), value: args.id },
            ],
            results: vec![
                ResultField { name: "alias".into() },
                ResultField { name: "enabled".into() },
                ResultField { name: "envs".into() },
                ResultField { name: "grantAllPermissions".into() },
                ResultField { name: "id".into() },
                ResultField { name: "name".into() },
                ResultField { name: "pluginReference".into() },
            ],
        };

        let o = invoke(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        get_plugin::Res {
            alias: hashmap.remove("alias").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            envs: hashmap.remove("envs").unwrap(),
            grant_all_permissions: hashmap.remove("grantAllPermissions").unwrap(),
            id: hashmap.remove("id").unwrap(),
            name: hashmap.remove("name").unwrap(),
            plugin_reference: hashmap.remove("pluginReference").unwrap(),
        }
    }
}
