use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::notification_policy_webhooks;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl notification_policy_webhooks::Guest for Component {
    fn invoke(
        name: String,
        args: notification_policy_webhooks::Args
    ) -> notification_policy_webhooks::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/notificationPolicyWebhooks:NotificationPolicyWebhooks".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "secret".into(), value: args.secret },
                ObjectField { name: "url".into(), value: args.url },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "createdAt".into() },
                ResultField { name: "lastFailure".into() },
                ResultField { name: "lastSuccess".into() },
                ResultField { name: "name".into() },
                ResultField { name: "secret".into() },
                ResultField { name: "type".into() },
                ResultField { name: "url".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        notification_policy_webhooks::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            created_at: hashmap.remove("createdAt").unwrap(),
            last_failure: hashmap.remove("lastFailure").unwrap(),
            last_success: hashmap.remove("lastSuccess").unwrap(),
            name: hashmap.remove("name").unwrap(),
            secret: hashmap.remove("secret").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            url: hashmap.remove("url").unwrap(),
        }
    }
}
