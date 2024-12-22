use std::collections::HashMap;
use crate::bindings::exports::pulumi::mypkg::get_bastion_shareable_link;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_bastion_shareable_link::Guest for Component {
    fn invoke(
        args: get_bastion_shareable_link::Args
    ) -> get_bastion_shareable_link::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "mypkg::getBastionShareableLink".into(),
            object: vec![
                ObjectField { name: "bastionHostName".into(), value: args.bastion_host_name },
                ObjectField { name: "resourceGroupName".into(), value: args.resource_group_name },
                ObjectField { name: "vms".into(), value: args.vms },
            ],
            results: vec![
                ResultField { name: "nextLink".into() },
            ],
        };

        let o = invoke(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        get_bastion_shareable_link::Res {
            next_link: hashmap.remove("nextLink").unwrap(),
        }
    }
}
