use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::certificate_pack;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl certificate_pack::Guest for Component {
    fn invoke(name: String, args: certificate_pack::Args) -> certificate_pack::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/certificatePack:CertificatePack".into(),
            name,
            object: vec![
                ObjectField { name: "certificateAuthority".into(), value: args.certificate_authority },
                ObjectField { name: "cloudflareBranding".into(), value: args.cloudflare_branding },
                ObjectField { name: "hosts".into(), value: args.hosts },
                ObjectField { name: "type".into(), value: args.type_ },
                ObjectField { name: "validationErrors".into(), value: args.validation_errors },
                ObjectField { name: "validationMethod".into(), value: args.validation_method },
                ObjectField { name: "validationRecords".into(), value: args.validation_records },
                ObjectField { name: "validityDays".into(), value: args.validity_days },
                ObjectField { name: "waitForActiveStatus".into(), value: args.wait_for_active_status },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "certificateAuthority".into() },
                ResultField { name: "cloudflareBranding".into() },
                ResultField { name: "hosts".into() },
                ResultField { name: "type".into() },
                ResultField { name: "validationErrors".into() },
                ResultField { name: "validationMethod".into() },
                ResultField { name: "validationRecords".into() },
                ResultField { name: "validityDays".into() },
                ResultField { name: "waitForActiveStatus".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        certificate_pack::Res {
            certificate_authority: hashmap.remove("certificateAuthority").unwrap(),
            cloudflare_branding: hashmap.remove("cloudflareBranding").unwrap(),
            hosts: hashmap.remove("hosts").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            validation_errors: hashmap.remove("validationErrors").unwrap(),
            validation_method: hashmap.remove("validationMethod").unwrap(),
            validation_records: hashmap.remove("validationRecords").unwrap(),
            validity_days: hashmap.remove("validityDays").unwrap(),
            wait_for_active_status: hashmap.remove("waitForActiveStatus").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
