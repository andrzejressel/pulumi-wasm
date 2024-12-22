//! Provides a Cloudflare Access Policy resource. Access Policies are
//! used in conjunction with Access Applications to restrict access to
//! a particular resource.
//! 
//! > It's required that an `account_id` or `zone_id` is provided and in most cases using either is fine.
//!    However, if you're using a scoped access token, you must provide the argument that matches the token's
//!    scope. For example, an access token that is scoped to the "example.com" zone needs to use the `zone_id` argument.
//!    If 'application_id' is omitted, the policy created can be reused by multiple access applications.
//!    Any cloudflare.AccessApplication resource can reference reusable policies through its `policies` argument.
//!    To destroy a reusable policy and remove it from all applications' policies lists on the same apply, preemptively set the
//!    lifecycle option `create_before_destroy` to true on the 'cloudflare_access_policy' resource.
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zeroTrustAccessPolicy:ZeroTrustAccessPolicy example account/<account_id>/<application_id>/<policy_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessPolicyArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The ID of the application the policy is associated with. Required when using `precedence`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub application_id: pulumi_wasm_rust::Output<Option<String>>,
    #[builder(into, default)]
    pub approval_groups: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustAccessPolicyApprovalGroup>>>,
    #[builder(into, default)]
    pub approval_required: pulumi_wasm_rust::Output<Option<bool>>,
    /// The rules that define how users may connect to the targets secured by your application. Only applicable to Infrastructure Applications, in which case this field is required.
    #[builder(into, default)]
    pub connection_rules: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustAccessPolicyConnectionRules>>,
    /// Defines the action Access will take if the policy matches the user. Available values: `allow`, `deny`, `non_identity`, `bypass`.
    #[builder(into)]
    pub decision: pulumi_wasm_rust::Output<String>,
    /// A series of access conditions, see Access Groups.
    #[builder(into, default)]
    pub excludes: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustAccessPolicyExclude>>>,
    /// A series of access conditions, see Access Groups.
    #[builder(into)]
    pub includes: pulumi_wasm_rust::Output<Vec<crate::types::ZeroTrustAccessPolicyInclude>>,
    /// Require this application to be served in an isolated browser for users matching this policy.
    #[builder(into, default)]
    pub isolation_required: pulumi_wasm_rust::Output<Option<bool>>,
    /// Friendly name of the Access Policy.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The unique precedence for policies on a single application. Required when using `application_id`.
    #[builder(into, default)]
    pub precedence: pulumi_wasm_rust::Output<Option<i32>>,
    /// The prompt to display to the user for a justification for accessing the resource. Required when using `purpose_justification_required`.
    #[builder(into, default)]
    pub purpose_justification_prompt: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to prompt the user for a justification for accessing the resource.
    #[builder(into, default)]
    pub purpose_justification_required: pulumi_wasm_rust::Output<Option<bool>>,
    /// A series of access conditions, see Access Groups.
    #[builder(into, default)]
    pub requires: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustAccessPolicyRequire>>>,
    /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`.
    #[builder(into, default)]
    pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    #[builder(into, default)]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct ZeroTrustAccessPolicyResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The ID of the application the policy is associated with. Required when using `precedence`. **Modifying this attribute will force creation of a new resource.**
    pub application_id: pulumi_wasm_rust::Output<Option<String>>,
    pub approval_groups: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustAccessPolicyApprovalGroup>>>,
    pub approval_required: pulumi_wasm_rust::Output<Option<bool>>,
    /// The rules that define how users may connect to the targets secured by your application. Only applicable to Infrastructure Applications, in which case this field is required.
    pub connection_rules: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustAccessPolicyConnectionRules>>,
    /// Defines the action Access will take if the policy matches the user. Available values: `allow`, `deny`, `non_identity`, `bypass`.
    pub decision: pulumi_wasm_rust::Output<String>,
    /// A series of access conditions, see Access Groups.
    pub excludes: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustAccessPolicyExclude>>>,
    /// A series of access conditions, see Access Groups.
    pub includes: pulumi_wasm_rust::Output<Vec<crate::types::ZeroTrustAccessPolicyInclude>>,
    /// Require this application to be served in an isolated browser for users matching this policy.
    pub isolation_required: pulumi_wasm_rust::Output<Option<bool>>,
    /// Friendly name of the Access Policy.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The unique precedence for policies on a single application. Required when using `application_id`.
    pub precedence: pulumi_wasm_rust::Output<Option<i32>>,
    /// The prompt to display to the user for a justification for accessing the resource. Required when using `purpose_justification_required`.
    pub purpose_justification_prompt: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to prompt the user for a justification for accessing the resource.
    pub purpose_justification_required: pulumi_wasm_rust::Output<Option<bool>>,
    /// A series of access conditions, see Access Groups.
    pub requires: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustAccessPolicyRequire>>>,
    /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`.
    pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: ZeroTrustAccessPolicyArgs
) -> ZeroTrustAccessPolicyResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_access_policy::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::zero_trust_access_policy::Args {
                account_id: &args.account_id.get_inner(),
                application_id: &args.application_id.get_inner(),
                approval_groups: &args.approval_groups.get_inner(),
                approval_required: &args.approval_required.get_inner(),
                connection_rules: &args.connection_rules.get_inner(),
                decision: &args.decision.get_inner(),
                excludes: &args.excludes.get_inner(),
                includes: &args.includes.get_inner(),
                isolation_required: &args.isolation_required.get_inner(),
                name: &args.name.get_inner(),
                precedence: &args.precedence.get_inner(),
                purpose_justification_prompt: &args.purpose_justification_prompt.get_inner(),
                purpose_justification_required: &args.purpose_justification_required.get_inner(),
                requires: &args.requires.get_inner(),
                session_duration: &args.session_duration.get_inner(),
                zone_id: &args.zone_id.get_inner(),
        }
    );

    ZeroTrustAccessPolicyResult {
        account_id: crate::into_domain(result.account_id),
        application_id: crate::into_domain(result.application_id),
        approval_groups: crate::into_domain(result.approval_groups),
        approval_required: crate::into_domain(result.approval_required),
        connection_rules: crate::into_domain(result.connection_rules),
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
