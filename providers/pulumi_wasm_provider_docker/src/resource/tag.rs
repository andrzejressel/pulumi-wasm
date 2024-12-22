use std::collections::HashMap;
use crate::bindings::exports::pulumi::docker::tag;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl tag::Guest for Component {
    fn invoke(
        name: String,
        args: tag::Args
    ) -> tag::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/tag:Tag".into(),
            name,
            object: vec![
                ObjectField { name: "sourceImage".into(), value: args.source_image },
                ObjectField { name: "targetImage".into(), value: args.target_image },
            ],
            results: vec![
                ResultField { name: "sourceImage".into() },
                ResultField { name: "sourceImageId".into() },
                ResultField { name: "targetImage".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        tag::Res {
            source_image: hashmap.remove("sourceImage").unwrap(),
            source_image_id: hashmap.remove("sourceImageId").unwrap(),
            target_image: hashmap.remove("targetImage").unwrap(),
        }

    }
}
