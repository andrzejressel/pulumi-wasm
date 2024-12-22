use std::collections::HashMap;
use crate::bindings::exports::pulumi::docker::plugin;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl plugin::Guest for Component {
    fn invoke(
        name: String,
        args: plugin::Args
    ) -> plugin::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/plugin:Plugin".into(),
            name,
            object: vec![
                ObjectField { name: "alias".into(), value: args.alias },
                ObjectField { name: "enableTimeout".into(), value: args.enable_timeout },
                ObjectField { name: "enabled".into(), value: args.enabled },
                ObjectField { name: "envs".into(), value: args.envs },
                ObjectField { name: "forceDestroy".into(), value: args.force_destroy },
                ObjectField { name: "forceDisable".into(), value: args.force_disable },
                ObjectField { name: "grantAllPermissions".into(), value: args.grant_all_permissions },
                ObjectField { name: "grantPermissions".into(), value: args.grant_permissions },
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "alias".into() },
                ResultField { name: "enableTimeout".into() },
                ResultField { name: "enabled".into() },
                ResultField { name: "envs".into() },
                ResultField { name: "forceDestroy".into() },
                ResultField { name: "forceDisable".into() },
                ResultField { name: "grantAllPermissions".into() },
                ResultField { name: "grantPermissions".into() },
                ResultField { name: "name".into() },
                ResultField { name: "pluginReference".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        plugin::Res {
            alias: hashmap.remove("alias").unwrap(),
            enable_timeout: hashmap.remove("enableTimeout").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            envs: hashmap.remove("envs").unwrap(),
            force_destroy: hashmap.remove("forceDestroy").unwrap(),
            force_disable: hashmap.remove("forceDisable").unwrap(),
            grant_all_permissions: hashmap.remove("grantAllPermissions").unwrap(),
            grant_permissions: hashmap.remove("grantPermissions").unwrap(),
            name: hashmap.remove("name").unwrap(),
            plugin_reference: hashmap.remove("pluginReference").unwrap(),
        }
    }
}
