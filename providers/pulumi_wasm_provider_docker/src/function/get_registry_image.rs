use std::collections::HashMap;
use crate::bindings::exports::pulumi::docker::get_registry_image;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_registry_image::Guest for Component {
    fn invoke(
        args: get_registry_image::Args
    ) -> get_registry_image::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "docker:index/getRegistryImage:getRegistryImage".into(),
            object: vec![
                ObjectField { name: "insecureSkipVerify".into(), value: args.insecure_skip_verify },
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "id".into() },
                ResultField { name: "insecureSkipVerify".into() },
                ResultField { name: "name".into() },
                ResultField { name: "sha256Digest".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_registry_image::Res {
            id: hashmap.remove("id").unwrap(),
            insecure_skip_verify: hashmap.remove("insecureSkipVerify").unwrap(),
            name: hashmap.remove("name").unwrap(),
            sha256_digest: hashmap.remove("sha256Digest").unwrap(),
        }
    }
}
