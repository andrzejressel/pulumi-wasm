//! Provides a Cloudflare DLP Profile resource. Data Loss Prevention profiles
//! are a set of entries that can be matched in HTTP bodies or files.
//! They are referenced in Zero Trust Gateway rules.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let creds = dlp_profile::create(
//!         "creds",
//!         DlpProfileArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .allowed_match_count(3)
//!             .entries(
//!                 vec![
//!                     DlpProfileEntry::builder().enabled(true)
//!                     .id("d8fcfc9c-773c-405e-8426-21ecbb67ba93")
//!                     .name("Amazon AWS Access Key ID").build_struct(),
//!                     DlpProfileEntry::builder().enabled(false)
//!                     .id("2c0e33e1-71da-40c8-aad3-32e674ad3d96")
//!                     .name("Amazon AWS Secret Access Key").build_struct(),
//!                     DlpProfileEntry::builder().enabled(true)
//!                     .id("4e92c006-3802-4dff-bbe1-8e1513b1c92a")
//!                     .name("Microsoft Azure Client Secret").build_struct(),
//!                     DlpProfileEntry::builder().enabled(false)
//!                     .id("5c713294-2375-4904-abcf-e4a15be4d592").name("SSH Private Key")
//!                     .build_struct(), DlpProfileEntry::builder().enabled(true)
//!                     .id("6c6579e4-d832-42d5-905c-8e53340930f2")
//!                     .name("Google GCP API Key").build_struct(),
//!                 ],
//!             )
//!             .name("Credentials and Secrets")
//!             .type_("predefined")
//!             .build_struct(),
//!     );
//!     let exampleCustom = dlp_profile::create(
//!         "exampleCustom",
//!         DlpProfileArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .allowed_match_count(0)
//!             .description("A profile with example entries")
//!             .entries(
//!                 vec![
//!                     DlpProfileEntry::builder().enabled(true)
//!                     .name("Matches visa credit cards")
//!                     .pattern(DlpProfileEntryPattern::builder()
//!                     .regex("4\\d{3}([-\\. ])?\\d{4}([-\\. ])?\\d{4}([-\\. ])?\\d{4}")
//!                     .validation("luhn").build_struct()).build_struct(),
//!                     DlpProfileEntry::builder().enabled(true)
//!                     .name("Matches diners club card")
//!                     .pattern(DlpProfileEntryPattern::builder()
//!                     .regex("(?:0[0-5]|[68][0-9])[0-9]{11}").validation("luhn")
//!                     .build_struct()).build_struct(),
//!                 ],
//!             )
//!             .name("Example Custom Profile")
//!             .type_("custom")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/dlpProfile:DlpProfile example <account_id>/<dlp_profile_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct DlpProfileArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Related DLP policies will trigger when the match count exceeds the number set.
    #[builder(into)]
    pub allowed_match_count: pulumi_wasm_rust::Output<i32>,
    /// Scan the context of predefined entries to only return matches surrounded by keywords.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub context_awareness: pulumi_wasm_rust::Output<Option<crate::types::DlpProfileContextAwareness>>,
    /// Brief summary of the profile and its intended use.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// List of entries to apply to the profile.
    #[builder(into)]
    pub entries: pulumi_wasm_rust::Output<Vec<crate::types::DlpProfileEntry>>,
    /// Name of the profile. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// If true, scan images via OCR to determine if any text present matches filters.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ocr_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The type of the profile. Available values: `custom`, `predefined`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
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
    /// Name of the profile. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// If true, scan images via OCR to determine if any text present matches filters.
    pub ocr_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The type of the profile. Available values: `custom`, `predefined`. **Modifying this attribute will force creation of a new resource.**
    pub type_: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: DlpProfileArgs) -> DlpProfileResult {

    let result = crate::bindings::pulumi::cloudflare::dlp_profile::invoke(name, &crate::bindings::pulumi::cloudflare::dlp_profile::Args {
        account_id: &args.account_id.get_inner(),
        allowed_match_count: &args.allowed_match_count.get_inner(),
        context_awareness: &args.context_awareness.get_inner(),
        description: &args.description.get_inner(),
        entries: &args.entries.get_inner(),
        name: &args.name.get_inner(),
        ocr_enabled: &args.ocr_enabled.get_inner(),
        type_: &args.type_.get_inner(),
    });

    DlpProfileResult {
        account_id: crate::into_domain(result.account_id),
        allowed_match_count: crate::into_domain(result.allowed_match_count),
        context_awareness: crate::into_domain(result.context_awareness),
        description: crate::into_domain(result.description),
        entries: crate::into_domain(result.entries),
        name: crate::into_domain(result.name),
        ocr_enabled: crate::into_domain(result.ocr_enabled),
        type_: crate::into_domain(result.type_),
    }
}
