use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::magic_wan_ipsec_tunnel;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl magic_wan_ipsec_tunnel::Guest for Component {
    fn invoke(name: String, args: magic_wan_ipsec_tunnel::Args) -> magic_wan_ipsec_tunnel::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/magicWanIpsecTunnel:MagicWanIpsecTunnel".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "allowNullCipher".into(), value: args.allow_null_cipher },
                ObjectField { name: "cloudflareEndpoint".into(), value: args.cloudflare_endpoint },
                ObjectField { name: "customerEndpoint".into(), value: args.customer_endpoint },
                ObjectField { name: "description".into(), value: args.description },
                ObjectField { name: "fqdnId".into(), value: args.fqdn_id },
                ObjectField { name: "healthCheckDirection".into(), value: args.health_check_direction },
                ObjectField { name: "healthCheckEnabled".into(), value: args.health_check_enabled },
                ObjectField { name: "healthCheckRate".into(), value: args.health_check_rate },
                ObjectField { name: "healthCheckTarget".into(), value: args.health_check_target },
                ObjectField { name: "healthCheckType".into(), value: args.health_check_type },
                ObjectField { name: "hexId".into(), value: args.hex_id },
                ObjectField { name: "interfaceAddress".into(), value: args.interface_address },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "psk".into(), value: args.psk },
                ObjectField { name: "remoteId".into(), value: args.remote_id },
                ObjectField { name: "replayProtection".into(), value: args.replay_protection },
                ObjectField { name: "userId".into(), value: args.user_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "allowNullCipher".into() },
                ResultField { name: "cloudflareEndpoint".into() },
                ResultField { name: "customerEndpoint".into() },
                ResultField { name: "description".into() },
                ResultField { name: "fqdnId".into() },
                ResultField { name: "healthCheckDirection".into() },
                ResultField { name: "healthCheckEnabled".into() },
                ResultField { name: "healthCheckRate".into() },
                ResultField { name: "healthCheckTarget".into() },
                ResultField { name: "healthCheckType".into() },
                ResultField { name: "hexId".into() },
                ResultField { name: "interfaceAddress".into() },
                ResultField { name: "name".into() },
                ResultField { name: "psk".into() },
                ResultField { name: "remoteId".into() },
                ResultField { name: "replayProtection".into() },
                ResultField { name: "userId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        magic_wan_ipsec_tunnel::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            allow_null_cipher: hashmap.remove("allowNullCipher").unwrap(),
            cloudflare_endpoint: hashmap.remove("cloudflareEndpoint").unwrap(),
            customer_endpoint: hashmap.remove("customerEndpoint").unwrap(),
            description: hashmap.remove("description").unwrap(),
            fqdn_id: hashmap.remove("fqdnId").unwrap(),
            health_check_direction: hashmap.remove("healthCheckDirection").unwrap(),
            health_check_enabled: hashmap.remove("healthCheckEnabled").unwrap(),
            health_check_rate: hashmap.remove("healthCheckRate").unwrap(),
            health_check_target: hashmap.remove("healthCheckTarget").unwrap(),
            health_check_type: hashmap.remove("healthCheckType").unwrap(),
            hex_id: hashmap.remove("hexId").unwrap(),
            interface_address: hashmap.remove("interfaceAddress").unwrap(),
            name: hashmap.remove("name").unwrap(),
            psk: hashmap.remove("psk").unwrap(),
            remote_id: hashmap.remove("remoteId").unwrap(),
            replay_protection: hashmap.remove("replayProtection").unwrap(),
            user_id: hashmap.remove("userId").unwrap(),
        }

    }
}
