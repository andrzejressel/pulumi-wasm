use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::zero_trust_device_profiles;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl zero_trust_device_profiles::Guest for Component {
    fn invoke(
        name: String,
        args: zero_trust_device_profiles::Args
    ) -> zero_trust_device_profiles::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustDeviceProfiles:ZeroTrustDeviceProfiles".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "allowModeSwitch".into(), value: args.allow_mode_switch },
                ObjectField { name: "allowUpdates".into(), value: args.allow_updates },
                ObjectField { name: "allowedToLeave".into(), value: args.allowed_to_leave },
                ObjectField { name: "autoConnect".into(), value: args.auto_connect },
                ObjectField { name: "captivePortal".into(), value: args.captive_portal },
                ObjectField { name: "default".into(), value: args.default },
                ObjectField { name: "description".into(), value: args.description },
                ObjectField { name: "disableAutoFallback".into(), value: args.disable_auto_fallback },
                ObjectField { name: "enabled".into(), value: args.enabled },
                ObjectField { name: "excludeOfficeIps".into(), value: args.exclude_office_ips },
                ObjectField { name: "match".into(), value: args.match_ },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "precedence".into(), value: args.precedence },
                ObjectField { name: "serviceModeV2Mode".into(), value: args.service_mode_v2_mode },
                ObjectField { name: "serviceModeV2Port".into(), value: args.service_mode_v2_port },
                ObjectField { name: "supportUrl".into(), value: args.support_url },
                ObjectField { name: "switchLocked".into(), value: args.switch_locked },
                ObjectField { name: "tunnelProtocol".into(), value: args.tunnel_protocol },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "allowModeSwitch".into() },
                ResultField { name: "allowUpdates".into() },
                ResultField { name: "allowedToLeave".into() },
                ResultField { name: "autoConnect".into() },
                ResultField { name: "captivePortal".into() },
                ResultField { name: "default".into() },
                ResultField { name: "description".into() },
                ResultField { name: "disableAutoFallback".into() },
                ResultField { name: "enabled".into() },
                ResultField { name: "excludeOfficeIps".into() },
                ResultField { name: "match".into() },
                ResultField { name: "name".into() },
                ResultField { name: "precedence".into() },
                ResultField { name: "serviceModeV2Mode".into() },
                ResultField { name: "serviceModeV2Port".into() },
                ResultField { name: "supportUrl".into() },
                ResultField { name: "switchLocked".into() },
                ResultField { name: "tunnelProtocol".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        zero_trust_device_profiles::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            allow_mode_switch: hashmap.remove("allowModeSwitch").unwrap(),
            allow_updates: hashmap.remove("allowUpdates").unwrap(),
            allowed_to_leave: hashmap.remove("allowedToLeave").unwrap(),
            auto_connect: hashmap.remove("autoConnect").unwrap(),
            captive_portal: hashmap.remove("captivePortal").unwrap(),
            default: hashmap.remove("default").unwrap(),
            description: hashmap.remove("description").unwrap(),
            disable_auto_fallback: hashmap.remove("disableAutoFallback").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            exclude_office_ips: hashmap.remove("excludeOfficeIps").unwrap(),
            match_: hashmap.remove("match").unwrap(),
            name: hashmap.remove("name").unwrap(),
            precedence: hashmap.remove("precedence").unwrap(),
            service_mode_v2_mode: hashmap.remove("serviceModeV2Mode").unwrap(),
            service_mode_v2_port: hashmap.remove("serviceModeV2Port").unwrap(),
            support_url: hashmap.remove("supportUrl").unwrap(),
            switch_locked: hashmap.remove("switchLocked").unwrap(),
            tunnel_protocol: hashmap.remove("tunnelProtocol").unwrap(),
        }
    }
}
