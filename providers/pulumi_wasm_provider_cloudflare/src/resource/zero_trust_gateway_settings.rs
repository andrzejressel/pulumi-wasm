use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::zero_trust_gateway_settings;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl zero_trust_gateway_settings::Guest for Component {
    fn invoke(name: String, args: zero_trust_gateway_settings::Args) -> zero_trust_gateway_settings::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustGatewaySettings:ZeroTrustGatewaySettings".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "activityLogEnabled".into(), value: args.activity_log_enabled },
                ObjectField { name: "antivirus".into(), value: args.antivirus },
                ObjectField { name: "blockPage".into(), value: args.block_page },
                ObjectField { name: "bodyScanning".into(), value: args.body_scanning },
                ObjectField { name: "certificate".into(), value: args.certificate },
                ObjectField { name: "customCertificate".into(), value: args.custom_certificate },
                ObjectField { name: "extendedEmailMatching".into(), value: args.extended_email_matching },
                ObjectField { name: "fips".into(), value: args.fips },
                ObjectField { name: "logging".into(), value: args.logging },
                ObjectField { name: "nonIdentityBrowserIsolationEnabled".into(), value: args.non_identity_browser_isolation_enabled },
                ObjectField { name: "payloadLog".into(), value: args.payload_log },
                ObjectField { name: "protocolDetectionEnabled".into(), value: args.protocol_detection_enabled },
                ObjectField { name: "proxy".into(), value: args.proxy },
                ObjectField { name: "sshSessionLog".into(), value: args.ssh_session_log },
                ObjectField { name: "tlsDecryptEnabled".into(), value: args.tls_decrypt_enabled },
                ObjectField { name: "urlBrowserIsolationEnabled".into(), value: args.url_browser_isolation_enabled },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "activityLogEnabled".into() },
                ResultField { name: "antivirus".into() },
                ResultField { name: "blockPage".into() },
                ResultField { name: "bodyScanning".into() },
                ResultField { name: "certificate".into() },
                ResultField { name: "customCertificate".into() },
                ResultField { name: "extendedEmailMatching".into() },
                ResultField { name: "fips".into() },
                ResultField { name: "logging".into() },
                ResultField { name: "nonIdentityBrowserIsolationEnabled".into() },
                ResultField { name: "payloadLog".into() },
                ResultField { name: "protocolDetectionEnabled".into() },
                ResultField { name: "proxy".into() },
                ResultField { name: "sshSessionLog".into() },
                ResultField { name: "tlsDecryptEnabled".into() },
                ResultField { name: "urlBrowserIsolationEnabled".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zero_trust_gateway_settings::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            activity_log_enabled: hashmap.remove("activityLogEnabled").unwrap(),
            antivirus: hashmap.remove("antivirus").unwrap(),
            block_page: hashmap.remove("blockPage").unwrap(),
            body_scanning: hashmap.remove("bodyScanning").unwrap(),
            certificate: hashmap.remove("certificate").unwrap(),
            custom_certificate: hashmap.remove("customCertificate").unwrap(),
            extended_email_matching: hashmap.remove("extendedEmailMatching").unwrap(),
            fips: hashmap.remove("fips").unwrap(),
            logging: hashmap.remove("logging").unwrap(),
            non_identity_browser_isolation_enabled: hashmap.remove("nonIdentityBrowserIsolationEnabled").unwrap(),
            payload_log: hashmap.remove("payloadLog").unwrap(),
            protocol_detection_enabled: hashmap.remove("protocolDetectionEnabled").unwrap(),
            proxy: hashmap.remove("proxy").unwrap(),
            ssh_session_log: hashmap.remove("sshSessionLog").unwrap(),
            tls_decrypt_enabled: hashmap.remove("tlsDecryptEnabled").unwrap(),
            url_browser_isolation_enabled: hashmap.remove("urlBrowserIsolationEnabled").unwrap(),
        }

    }
}
