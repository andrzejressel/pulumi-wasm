use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::notification_policy;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl notification_policy::Guest for Component {
    fn invoke(
        name: String,
        args: notification_policy::Args
    ) -> notification_policy::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/notificationPolicy:NotificationPolicy".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "alertType".into(), value: args.alert_type },
                ObjectField { name: "description".into(), value: args.description },
                ObjectField { name: "emailIntegrations".into(), value: args.email_integrations },
                ObjectField { name: "enabled".into(), value: args.enabled },
                ObjectField { name: "filters".into(), value: args.filters },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "pagerdutyIntegrations".into(), value: args.pagerduty_integrations },
                ObjectField { name: "webhooksIntegrations".into(), value: args.webhooks_integrations },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "alertType".into() },
                ResultField { name: "created".into() },
                ResultField { name: "description".into() },
                ResultField { name: "emailIntegrations".into() },
                ResultField { name: "enabled".into() },
                ResultField { name: "filters".into() },
                ResultField { name: "modified".into() },
                ResultField { name: "name".into() },
                ResultField { name: "pagerdutyIntegrations".into() },
                ResultField { name: "webhooksIntegrations".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        notification_policy::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            alert_type: hashmap.remove("alertType").unwrap(),
            created: hashmap.remove("created").unwrap(),
            description: hashmap.remove("description").unwrap(),
            email_integrations: hashmap.remove("emailIntegrations").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            filters: hashmap.remove("filters").unwrap(),
            modified: hashmap.remove("modified").unwrap(),
            name: hashmap.remove("name").unwrap(),
            pagerduty_integrations: hashmap.remove("pagerdutyIntegrations").unwrap(),
            webhooks_integrations: hashmap.remove("webhooksIntegrations").unwrap(),
        }

    }
}
