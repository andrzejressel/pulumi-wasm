use std::collections::HashMap;
use crate::bindings::exports::pulumi::mypkg::get_integration_runtime_object_metadatum;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_integration_runtime_object_metadatum::Guest for Component {
    fn invoke(
        args: get_integration_runtime_object_metadatum::Args
    ) -> get_integration_runtime_object_metadatum::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "mypkg::getIntegrationRuntimeObjectMetadatum".into(),
            object: vec![
                ObjectField { name: "factoryName".into(), value: args.factory_name },
                ObjectField { name: "integrationRuntimeName".into(), value: args.integration_runtime_name },
                ObjectField { name: "metadataPath".into(), value: args.metadata_path },
                ObjectField { name: "resourceGroupName".into(), value: args.resource_group_name },
            ],
            results: vec![
                ResultField { name: "nextLink".into() },
                ResultField { name: "value".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_integration_runtime_object_metadatum::Res {
            next_link: hashmap.remove("nextLink").unwrap(),
            value: hashmap.remove("value").unwrap(),
        }
    }
}
