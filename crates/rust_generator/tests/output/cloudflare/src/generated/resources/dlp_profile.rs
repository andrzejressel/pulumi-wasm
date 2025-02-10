/// Provides a Cloudflare DLP Profile resource. Data Loss Prevention profiles
/// are a set of entries that can be matched in HTTP bodies or files.
/// They are referenced in Zero Trust Gateway rules.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let creds = dlp_profile::create(
///         "creds",
///         DlpProfileArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .allowed_match_count(3)
///             .entries(
///                 vec![
///                     DlpProfileEntry::builder().enabled(true)
///                     .id("d8fcfc9c-773c-405e-8426-21ecbb67ba93")
///                     .name("Amazon AWS Access Key ID").build_struct(),
///                     DlpProfileEntry::builder().enabled(false)
///                     .id("2c0e33e1-71da-40c8-aad3-32e674ad3d96")
///                     .name("Amazon AWS Secret Access Key").build_struct(),
///                     DlpProfileEntry::builder().enabled(true)
///                     .id("4e92c006-3802-4dff-bbe1-8e1513b1c92a")
///                     .name("Microsoft Azure Client Secret").build_struct(),
///                     DlpProfileEntry::builder().enabled(false)
///                     .id("5c713294-2375-4904-abcf-e4a15be4d592").name("SSH Private Key")
///                     .build_struct(), DlpProfileEntry::builder().enabled(true)
///                     .id("6c6579e4-d832-42d5-905c-8e53340930f2")
///                     .name("Google GCP API Key").build_struct(),
///                 ],
///             )
///             .name("Credentials and Secrets")
///             .type_("predefined")
///             .build_struct(),
///     );
///     let exampleCustom = dlp_profile::create(
///         "exampleCustom",
///         DlpProfileArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .allowed_match_count(0)
///             .description("A profile with example entries")
///             .entries(
///                 vec![
///                     DlpProfileEntry::builder().enabled(true)
///                     .name("Matches visa credit cards")
///                     .pattern(DlpProfileEntryPattern::builder()
///                     .regex("4\\d{3}([-\\. ])?\\d{4}([-\\. ])?\\d{4}([-\\. ])?\\d{4}")
///                     .validation("luhn").build_struct()).build_struct(),
///                     DlpProfileEntry::builder().enabled(true)
///                     .name("Matches diners club card")
///                     .pattern(DlpProfileEntryPattern::builder()
///                     .regex("(?:0[0-5]|[68][0-9])[0-9]{11}").validation("luhn")
///                     .build_struct()).build_struct(),
///                 ],
///             )
///             .name("Example Custom Profile")
///             .type_("custom")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/dlpProfile:DlpProfile example <account_id>/<dlp_profile_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dlp_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DlpProfileArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Related DLP policies will trigger when the match count exceeds the number set.
        #[builder(into)]
        pub allowed_match_count: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Scan the context of predefined entries to only return matches surrounded by keywords.
        #[builder(into, default)]
        pub context_awareness: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::DlpProfileContextAwareness>,
        >,
        /// Brief summary of the profile and its intended use.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of entries to apply to the profile.
        #[builder(into)]
        pub entries: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::types::DlpProfileEntry>,
        >,
        /// Name of the profile. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If true, scan images via OCR to determine if any text present matches filters.
        #[builder(into, default)]
        pub ocr_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The type of the profile. Available values: `custom`, `predefined`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DlpProfileResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Related DLP policies will trigger when the match count exceeds the number set.
        pub allowed_match_count: pulumi_gestalt_rust::Output<i32>,
        /// Scan the context of predefined entries to only return matches surrounded by keywords.
        pub context_awareness: pulumi_gestalt_rust::Output<
            super::types::DlpProfileContextAwareness,
        >,
        /// Brief summary of the profile and its intended use.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of entries to apply to the profile.
        pub entries: pulumi_gestalt_rust::Output<Vec<super::types::DlpProfileEntry>>,
        /// Name of the profile. **Modifying this attribute will force creation of a new resource.**
        pub name: pulumi_gestalt_rust::Output<String>,
        /// If true, scan images via OCR to determine if any text present matches filters.
        pub ocr_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The type of the profile. Available values: `custom`, `predefined`. **Modifying this attribute will force creation of a new resource.**
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DlpProfileArgs,
    ) -> DlpProfileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let allowed_match_count_binding = args.allowed_match_count.get_output(context);
        let context_awareness_binding = args.context_awareness.get_output(context);
        let description_binding = args.description.get_output(context);
        let entries_binding = args.entries.get_output(context);
        let name_binding = args.name.get_output(context);
        let ocr_enabled_binding = args.ocr_enabled.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/dlpProfile:DlpProfile".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowedMatchCount".into(),
                    value: allowed_match_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contextAwareness".into(),
                    value: context_awareness_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "entries".into(),
                    value: entries_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ocrEnabled".into(),
                    value: ocr_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DlpProfileResult {
            account_id: o.get_field("accountId"),
            allowed_match_count: o.get_field("allowedMatchCount"),
            context_awareness: o.get_field("contextAwareness"),
            description: o.get_field("description"),
            entries: o.get_field("entries"),
            name: o.get_field("name"),
            ocr_enabled: o.get_field("ocrEnabled"),
            type_: o.get_field("type"),
        }
    }
}
