use std::collections::HashMap;
use crate::bindings::exports::pulumi::plant::tree_rubber_tree;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl tree_rubber_tree::Guest for Component {
    fn invoke(name: String, args: tree_rubber_tree::Args) -> tree_rubber_tree::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "plant:tree/v1:RubberTree".into(),
            name,
            object: vec![
                ObjectField { name: "container".into(), value: args.container },
                ObjectField { name: "diameter".into(), value: args.diameter },
                ObjectField { name: "farm".into(), value: args.farm },
                ObjectField { name: "size".into(), value: args.size },
                ObjectField { name: "type".into(), value: args.type_ },
            ],
            results: vec![
                ResultField { name: "container".into() },
                ResultField { name: "diameter".into() },
                ResultField { name: "farm".into() },
                ResultField { name: "size".into() },
                ResultField { name: "type".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        tree_rubber_tree::Res {
            container: hashmap.remove("container").unwrap(),
            diameter: hashmap.remove("diameter").unwrap(),
            farm: hashmap.remove("farm").unwrap(),
            size: hashmap.remove("size").unwrap(),
            type_: hashmap.remove("type").unwrap(),
        }

    }
}
