use std::collections::HashMap;
use crate::bindings::exports::pulumi::example::foo;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl foo::Guest for Component {
    fn invoke(name: String, args: foo::Args) -> foo::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "example:index:Foo".into(),
            name,
            object: vec![
                ObjectField { name: "argument".into(), value: args.argument },
                ObjectField { name: "backupKubeClientSettings".into(), value: args.backup_kube_client_settings },
                ObjectField { name: "kubeClientSettings".into(), value: args.kube_client_settings },
                ObjectField { name: "settings".into(), value: args.settings },
            ],
            results: vec![
                ResultField { name: "defaultKubeClientSettings".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        foo::Res {
            default_kube_client_settings: hashmap.remove("defaultKubeClientSettings").unwrap(),
        }

    }
}
