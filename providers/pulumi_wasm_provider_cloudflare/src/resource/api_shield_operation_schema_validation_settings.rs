use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::api_shield_operation_schema_validation_settings;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl api_shield_operation_schema_validation_settings::Guest for Component {
    fn invoke(name: String, args: api_shield_operation_schema_validation_settings::Args) -> api_shield_operation_schema_validation_settings::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/apiShieldOperationSchemaValidationSettings:ApiShieldOperationSchemaValidationSettings".into(),
            name,
            object: vec![
                ObjectField { name: "mitigationAction".into(), value: args.mitigation_action },
                ObjectField { name: "operationId".into(), value: args.operation_id },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "mitigationAction".into() },
                ResultField { name: "operationId".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        api_shield_operation_schema_validation_settings::Res {
            mitigation_action: hashmap.remove("mitigationAction").unwrap(),
            operation_id: hashmap.remove("operationId").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}