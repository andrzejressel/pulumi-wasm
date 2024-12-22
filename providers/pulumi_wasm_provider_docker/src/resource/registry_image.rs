use std::collections::HashMap;
use crate::bindings::exports::pulumi::docker::registry_image;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl registry_image::Guest for Component {
    fn invoke(
        name: String,
        args: registry_image::Args
    ) -> registry_image::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/registryImage:RegistryImage".into(),
            name,
            object: vec![
                ObjectField { name: "insecureSkipVerify".into(), value: args.insecure_skip_verify },
                ObjectField { name: "keepRemotely".into(), value: args.keep_remotely },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "triggers".into(), value: args.triggers },
            ],
            results: vec![
                ResultField { name: "insecureSkipVerify".into() },
                ResultField { name: "keepRemotely".into() },
                ResultField { name: "name".into() },
                ResultField { name: "sha256Digest".into() },
                ResultField { name: "triggers".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        registry_image::Res {
            insecure_skip_verify: hashmap.remove("insecureSkipVerify").unwrap(),
            keep_remotely: hashmap.remove("keepRemotely").unwrap(),
            name: hashmap.remove("name").unwrap(),
            sha256_digest: hashmap.remove("sha256Digest").unwrap(),
            triggers: hashmap.remove("triggers").unwrap(),
        }
    }
}
