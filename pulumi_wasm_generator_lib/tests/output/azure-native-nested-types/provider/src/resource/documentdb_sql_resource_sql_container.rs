use std::collections::HashMap;
use crate::bindings::exports::pulumi::azure_native::documentdb_sql_resource_sql_container;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl documentdb_sql_resource_sql_container::Guest for Component {
    fn invoke(
        name: String,
    ) -> documentdb_sql_resource_sql_container::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "azure-native:documentdb:SqlResourceSqlContainer".into(),
            name,
            object: vec![
            ],
            results: vec![
                ResultField { name: "resource".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        documentdb_sql_resource_sql_container::Res {
            resource: hashmap.remove("resource").unwrap(),
        }

    }
}
