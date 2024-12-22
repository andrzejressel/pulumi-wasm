use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::web_analytics_site;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl web_analytics_site::Guest for Component {
    fn invoke(
        name: String,
        args: web_analytics_site::Args
    ) -> web_analytics_site::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/webAnalyticsSite:WebAnalyticsSite".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "autoInstall".into(), value: args.auto_install },
                ObjectField { name: "host".into(), value: args.host },
                ObjectField { name: "zoneTag".into(), value: args.zone_tag },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "autoInstall".into() },
                ResultField { name: "host".into() },
                ResultField { name: "rulesetId".into() },
                ResultField { name: "siteTag".into() },
                ResultField { name: "siteToken".into() },
                ResultField { name: "snippet".into() },
                ResultField { name: "zoneTag".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        web_analytics_site::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            auto_install: hashmap.remove("autoInstall").unwrap(),
            host: hashmap.remove("host").unwrap(),
            ruleset_id: hashmap.remove("rulesetId").unwrap(),
            site_tag: hashmap.remove("siteTag").unwrap(),
            site_token: hashmap.remove("siteToken").unwrap(),
            snippet: hashmap.remove("snippet").unwrap(),
            zone_tag: hashmap.remove("zoneTag").unwrap(),
        }
    }
}
