use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::api_shield_schema_validation_settings;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl api_shield_schema_validation_settings::Guest for Component {
    fn invoke(
        name: String,
        args: api_shield_schema_validation_settings::Args
    ) -> api_shield_schema_validation_settings::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/apiShieldSchemaValidationSettings:ApiShieldSchemaValidationSettings".into(),
            name,
            object: vec![
                ObjectField { name: "validationDefaultMitigationAction".into(), value: args.validation_default_mitigation_action },
                ObjectField { name: "validationOverrideMitigationAction".into(), value: args.validation_override_mitigation_action },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "validationDefaultMitigationAction".into() },
                ResultField { name: "validationOverrideMitigationAction".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        api_shield_schema_validation_settings::Res {
            validation_default_mitigation_action: hashmap.remove("validationDefaultMitigationAction").unwrap(),
            validation_override_mitigation_action: hashmap.remove("validationOverrideMitigationAction").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
