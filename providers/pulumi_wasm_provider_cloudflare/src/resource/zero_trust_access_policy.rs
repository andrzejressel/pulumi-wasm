use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::zero_trust_access_policy;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl zero_trust_access_policy::Guest for Component {
    fn invoke(
        name: String,
        args: zero_trust_access_policy::Args
    ) -> zero_trust_access_policy::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustAccessPolicy:ZeroTrustAccessPolicy".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "applicationId".into(), value: args.application_id },
                ObjectField { name: "approvalGroups".into(), value: args.approval_groups },
                ObjectField { name: "approvalRequired".into(), value: args.approval_required },
                ObjectField { name: "connectionRules".into(), value: args.connection_rules },
                ObjectField { name: "decision".into(), value: args.decision },
                ObjectField { name: "excludes".into(), value: args.excludes },
                ObjectField { name: "includes".into(), value: args.includes },
                ObjectField { name: "isolationRequired".into(), value: args.isolation_required },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "precedence".into(), value: args.precedence },
                ObjectField { name: "purposeJustificationPrompt".into(), value: args.purpose_justification_prompt },
                ObjectField { name: "purposeJustificationRequired".into(), value: args.purpose_justification_required },
                ObjectField { name: "requires".into(), value: args.requires },
                ObjectField { name: "sessionDuration".into(), value: args.session_duration },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "applicationId".into() },
                ResultField { name: "approvalGroups".into() },
                ResultField { name: "approvalRequired".into() },
                ResultField { name: "connectionRules".into() },
                ResultField { name: "decision".into() },
                ResultField { name: "excludes".into() },
                ResultField { name: "includes".into() },
                ResultField { name: "isolationRequired".into() },
                ResultField { name: "name".into() },
                ResultField { name: "precedence".into() },
                ResultField { name: "purposeJustificationPrompt".into() },
                ResultField { name: "purposeJustificationRequired".into() },
                ResultField { name: "requires".into() },
                ResultField { name: "sessionDuration".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        zero_trust_access_policy::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            application_id: hashmap.remove("applicationId").unwrap(),
            approval_groups: hashmap.remove("approvalGroups").unwrap(),
            approval_required: hashmap.remove("approvalRequired").unwrap(),
            connection_rules: hashmap.remove("connectionRules").unwrap(),
            decision: hashmap.remove("decision").unwrap(),
            excludes: hashmap.remove("excludes").unwrap(),
            includes: hashmap.remove("includes").unwrap(),
            isolation_required: hashmap.remove("isolationRequired").unwrap(),
            name: hashmap.remove("name").unwrap(),
            precedence: hashmap.remove("precedence").unwrap(),
            purpose_justification_prompt: hashmap.remove("purposeJustificationPrompt").unwrap(),
            purpose_justification_required: hashmap.remove("purposeJustificationRequired").unwrap(),
            requires: hashmap.remove("requires").unwrap(),
            session_duration: hashmap.remove("sessionDuration").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
