use std::collections::HashMap;
use crate::bindings::exports::pulumi::docker::get_remote_image;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_remote_image::Guest for Component {
    fn invoke(
        args: get_remote_image::Args
    ) -> get_remote_image::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "docker:index/getRemoteImage:getRemoteImage".into(),
            object: vec![
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "id".into() },
                ResultField { name: "name".into() },
                ResultField { name: "repoDigest".into() },
            ],
        };

        let o = invoke(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        get_remote_image::Res {
            id: hashmap.remove("id").unwrap(),
            name: hashmap.remove("name").unwrap(),
            repo_digest: hashmap.remove("repoDigest").unwrap(),
        }
    }
}
