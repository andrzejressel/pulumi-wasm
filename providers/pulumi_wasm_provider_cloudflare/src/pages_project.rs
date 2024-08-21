use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::pages_project;
use crate::Component;
use std::collections::HashMap;

impl pages_project::Guest for Component {
    fn invoke(name: String, args: pages_project::Args) -> pages_project::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/pagesProject:PagesProject".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "buildConfig".into(),
                    value: args.build_config,
                },
                ObjectField {
                    name: "deploymentConfigs".into(),
                    value: args.deployment_configs,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "productionBranch".into(),
                    value: args.production_branch,
                },
                ObjectField {
                    name: "source".into(),
                    value: args.source,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "buildConfig".into(),
                },
                ResultField {
                    name: "createdOn".into(),
                },
                ResultField {
                    name: "deploymentConfigs".into(),
                },
                ResultField {
                    name: "domains".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "productionBranch".into(),
                },
                ResultField {
                    name: "source".into(),
                },
                ResultField {
                    name: "subdomain".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        pages_project::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            build_config: hashmap.remove("buildConfig").unwrap(),
            created_on: hashmap.remove("createdOn").unwrap(),
            deployment_configs: hashmap.remove("deploymentConfigs").unwrap(),
            domains: hashmap.remove("domains").unwrap(),
            name: hashmap.remove("name").unwrap(),
            production_branch: hashmap.remove("productionBranch").unwrap(),
            source: hashmap.remove("source").unwrap(),
            subdomain: hashmap.remove("subdomain").unwrap(),
        }
    }
}
