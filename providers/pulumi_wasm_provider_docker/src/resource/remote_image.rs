use std::collections::HashMap;
use crate::bindings::exports::pulumi::docker::remote_image;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl remote_image::Guest for Component {
    fn invoke(
        name: String,
        args: remote_image::Args
    ) -> remote_image::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/remoteImage:RemoteImage".into(),
            name,
            object: vec![
                ObjectField { name: "build".into(), value: args.build },
                ObjectField { name: "forceRemove".into(), value: args.force_remove },
                ObjectField { name: "keepLocally".into(), value: args.keep_locally },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "platform".into(), value: args.platform },
                ObjectField { name: "pullTriggers".into(), value: args.pull_triggers },
                ObjectField { name: "triggers".into(), value: args.triggers },
            ],
            results: vec![
                ResultField { name: "build".into() },
                ResultField { name: "forceRemove".into() },
                ResultField { name: "imageId".into() },
                ResultField { name: "keepLocally".into() },
                ResultField { name: "name".into() },
                ResultField { name: "platform".into() },
                ResultField { name: "pullTriggers".into() },
                ResultField { name: "repoDigest".into() },
                ResultField { name: "triggers".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        remote_image::Res {
            build: hashmap.remove("build").unwrap(),
            force_remove: hashmap.remove("forceRemove").unwrap(),
            image_id: hashmap.remove("imageId").unwrap(),
            keep_locally: hashmap.remove("keepLocally").unwrap(),
            name: hashmap.remove("name").unwrap(),
            platform: hashmap.remove("platform").unwrap(),
            pull_triggers: hashmap.remove("pullTriggers").unwrap(),
            repo_digest: hashmap.remove("repoDigest").unwrap(),
            triggers: hashmap.remove("triggers").unwrap(),
        }
    }
}
