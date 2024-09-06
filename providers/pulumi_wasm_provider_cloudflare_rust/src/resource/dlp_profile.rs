//! Provides a Cloudflare DLP Profile resource. Data Loss Prevention profiles
//! are a set of entries that can be matched in HTTP bodies or files.
//! They are referenced in Zero Trust Gateway rules.
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/dlpProfile:DlpProfile example <account_id>/<dlp_profile_id>
//! ```
//!

pub struct DlpProfileArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Related DLP policies will trigger when the match count exceeds the number set.
    pub allowed_match_count: pulumi_wasm_rust::Output<i32>,
    /// Scan the context of predefined entries to only return matches surrounded by keywords.
    pub context_awareness:
        pulumi_wasm_rust::Output<Option<crate::types::DlpProfileContextAwareness>>,
    /// Brief summary of the profile and its intended use.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// List of entries to apply to the profile.
    pub entries: pulumi_wasm_rust::Output<Vec<crate::types::DlpProfileEntry>>,
    /// Name of the entry to deploy.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The type of the profile. Available values: `custom`, `predefined`. **Modifying this attribute will force creation of a new resource.**
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub struct DlpProfileResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Related DLP policies will trigger when the match count exceeds the number set.
    pub allowed_match_count: pulumi_wasm_rust::Output<i32>,
    /// Scan the context of predefined entries to only return matches surrounded by keywords.
    pub context_awareness: pulumi_wasm_rust::Output<crate::types::DlpProfileContextAwareness>,
    /// Brief summary of the profile and its intended use.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// List of entries to apply to the profile.
    pub entries: pulumi_wasm_rust::Output<Vec<crate::types::DlpProfileEntry>>,
    /// Name of the entry to deploy.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The type of the profile. Available values: `custom`, `predefined`. **Modifying this attribute will force creation of a new resource.**
    pub type_: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: DlpProfileArgs) -> DlpProfileResult {
    let result = crate::bindings::pulumi::cloudflare::dlp_profile::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::dlp_profile::Args {
            account_id: &args.account_id.get_inner(),
            allowed_match_count: &args.allowed_match_count.get_inner(),
            context_awareness: &args.context_awareness.get_inner(),
            description: &args.description.get_inner(),
            entries: &args.entries.get_inner(),
            name: &args.name.get_inner(),
            type_: &args.type_.get_inner(),
        },
    );

    DlpProfileResult {
        account_id: crate::into_domain(result.account_id),
        allowed_match_count: crate::into_domain(result.allowed_match_count),
        context_awareness: crate::into_domain(result.context_awareness),
        description: crate::into_domain(result.description),
        entries: crate::into_domain(result.entries),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
    }
}
