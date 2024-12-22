use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::bot_management;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl bot_management::Guest for Component {
    fn invoke(
        name: String,
        args: bot_management::Args
    ) -> bot_management::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/botManagement:BotManagement".into(),
            name,
            object: vec![
                ObjectField { name: "aiBotsProtection".into(), value: args.ai_bots_protection },
                ObjectField { name: "autoUpdateModel".into(), value: args.auto_update_model },
                ObjectField { name: "enableJs".into(), value: args.enable_js },
                ObjectField { name: "fightMode".into(), value: args.fight_mode },
                ObjectField { name: "optimizeWordpress".into(), value: args.optimize_wordpress },
                ObjectField { name: "sbfmDefinitelyAutomated".into(), value: args.sbfm_definitely_automated },
                ObjectField { name: "sbfmLikelyAutomated".into(), value: args.sbfm_likely_automated },
                ObjectField { name: "sbfmStaticResourceProtection".into(), value: args.sbfm_static_resource_protection },
                ObjectField { name: "sbfmVerifiedBots".into(), value: args.sbfm_verified_bots },
                ObjectField { name: "suppressSessionScore".into(), value: args.suppress_session_score },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "aiBotsProtection".into() },
                ResultField { name: "autoUpdateModel".into() },
                ResultField { name: "enableJs".into() },
                ResultField { name: "fightMode".into() },
                ResultField { name: "optimizeWordpress".into() },
                ResultField { name: "sbfmDefinitelyAutomated".into() },
                ResultField { name: "sbfmLikelyAutomated".into() },
                ResultField { name: "sbfmStaticResourceProtection".into() },
                ResultField { name: "sbfmVerifiedBots".into() },
                ResultField { name: "suppressSessionScore".into() },
                ResultField { name: "usingLatestModel".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        bot_management::Res {
            ai_bots_protection: hashmap.remove("aiBotsProtection").unwrap(),
            auto_update_model: hashmap.remove("autoUpdateModel").unwrap(),
            enable_js: hashmap.remove("enableJs").unwrap(),
            fight_mode: hashmap.remove("fightMode").unwrap(),
            optimize_wordpress: hashmap.remove("optimizeWordpress").unwrap(),
            sbfm_definitely_automated: hashmap.remove("sbfmDefinitelyAutomated").unwrap(),
            sbfm_likely_automated: hashmap.remove("sbfmLikelyAutomated").unwrap(),
            sbfm_static_resource_protection: hashmap.remove("sbfmStaticResourceProtection").unwrap(),
            sbfm_verified_bots: hashmap.remove("sbfmVerifiedBots").unwrap(),
            suppress_session_score: hashmap.remove("suppressSessionScore").unwrap(),
            using_latest_model: hashmap.remove("usingLatestModel").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
