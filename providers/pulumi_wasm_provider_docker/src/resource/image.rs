use std::collections::HashMap;
use crate::bindings::exports::pulumi::docker::image;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl image::Guest for Component {
    fn invoke(
        name: String,
        args: image::Args
    ) -> image::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/image:Image".into(),
            name,
            object: vec![
                ObjectField { name: "build".into(), value: args.build },
                ObjectField { name: "buildOnPreview".into(), value: args.build_on_preview },
                ObjectField { name: "imageName".into(), value: args.image_name },
                ObjectField { name: "registry".into(), value: args.registry },
                ObjectField { name: "skipPush".into(), value: args.skip_push },
            ],
            results: vec![
                ResultField { name: "baseImageName".into() },
                ResultField { name: "context".into() },
                ResultField { name: "dockerfile".into() },
                ResultField { name: "imageName".into() },
                ResultField { name: "platform".into() },
                ResultField { name: "registryServer".into() },
                ResultField { name: "repoDigest".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        image::Res {
            base_image_name: hashmap.remove("baseImageName").unwrap(),
            context: hashmap.remove("context").unwrap(),
            dockerfile: hashmap.remove("dockerfile").unwrap(),
            image_name: hashmap.remove("imageName").unwrap(),
            platform: hashmap.remove("platform").unwrap(),
            registry_server: hashmap.remove("registryServer").unwrap(),
            repo_digest: hashmap.remove("repoDigest").unwrap(),
        }
    }
}
