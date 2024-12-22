use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::account_member;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl account_member::Guest for Component {
    fn invoke(
        name: String,
        args: account_member::Args
    ) -> account_member::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accountMember:AccountMember".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "emailAddress".into(), value: args.email_address },
                ObjectField { name: "roleIds".into(), value: args.role_ids },
                ObjectField { name: "status".into(), value: args.status },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "emailAddress".into() },
                ResultField { name: "roleIds".into() },
                ResultField { name: "status".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        account_member::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            email_address: hashmap.remove("emailAddress").unwrap(),
            role_ids: hashmap.remove("roleIds").unwrap(),
            status: hashmap.remove("status").unwrap(),
        }
    }
}
