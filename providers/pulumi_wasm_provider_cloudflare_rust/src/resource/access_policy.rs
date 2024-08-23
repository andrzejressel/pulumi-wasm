pub struct AccessPolicyArgs {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub application_id: pulumi_wasm_rust::Output<String>,
    pub approval_groups:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessPolicyApprovalGroup>>>,
    pub approval_required: pulumi_wasm_rust::Output<Option<bool>>,
    pub decision: pulumi_wasm_rust::Output<String>,
    pub excludes: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessPolicyExclude>>>,
    pub includes: pulumi_wasm_rust::Output<Vec<crate::types::AccessPolicyInclude>>,
    pub isolation_required: pulumi_wasm_rust::Output<Option<bool>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub precedence: pulumi_wasm_rust::Output<i32>,
    pub purpose_justification_prompt: pulumi_wasm_rust::Output<Option<String>>,
    pub purpose_justification_required: pulumi_wasm_rust::Output<Option<bool>>,
    pub requires: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessPolicyRequire>>>,
    pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessPolicyResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub application_id: pulumi_wasm_rust::Output<String>,
    pub approval_groups:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessPolicyApprovalGroup>>>,
    pub approval_required: pulumi_wasm_rust::Output<Option<bool>>,
    pub decision: pulumi_wasm_rust::Output<String>,
    pub excludes: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessPolicyExclude>>>,
    pub includes: pulumi_wasm_rust::Output<Vec<crate::types::AccessPolicyInclude>>,
    pub isolation_required: pulumi_wasm_rust::Output<Option<bool>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub precedence: pulumi_wasm_rust::Output<i32>,
    pub purpose_justification_prompt: pulumi_wasm_rust::Output<Option<String>>,
    pub purpose_justification_required: pulumi_wasm_rust::Output<Option<bool>>,
    pub requires: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessPolicyRequire>>>,
    pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: AccessPolicyArgs) -> AccessPolicyResult {
    let result = crate::bindings::pulumi::cloudflare::access_policy::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::access_policy::Args {
            account_id: args.account_id.get_inner(),
            application_id: args.application_id.get_inner(),
            approval_groups: args.approval_groups.get_inner(),
            approval_required: args.approval_required.get_inner(),
            decision: args.decision.get_inner(),
            excludes: args.excludes.get_inner(),
            includes: args.includes.get_inner(),
            isolation_required: args.isolation_required.get_inner(),
            name: args.name.get_inner(),
            precedence: args.precedence.get_inner(),
            purpose_justification_prompt: args.purpose_justification_prompt.get_inner(),
            purpose_justification_required: args.purpose_justification_required.get_inner(),
            requires: args.requires.get_inner(),
            session_duration: args.session_duration.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    AccessPolicyResult {
        account_id: crate::into_domain(result.account_id),
        application_id: crate::into_domain(result.application_id),
        approval_groups: crate::into_domain(result.approval_groups),
        approval_required: crate::into_domain(result.approval_required),
        decision: crate::into_domain(result.decision),
        excludes: crate::into_domain(result.excludes),
        includes: crate::into_domain(result.includes),
        isolation_required: crate::into_domain(result.isolation_required),
        name: crate::into_domain(result.name),
        precedence: crate::into_domain(result.precedence),
        purpose_justification_prompt: crate::into_domain(result.purpose_justification_prompt),
        purpose_justification_required: crate::into_domain(result.purpose_justification_required),
        requires: crate::into_domain(result.requires),
        session_duration: crate::into_domain(result.session_duration),
        zone_id: crate::into_domain(result.zone_id),
    }
}
