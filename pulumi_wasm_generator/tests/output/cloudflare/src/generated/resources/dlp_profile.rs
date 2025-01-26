/// Provides a Cloudflare DLP Profile resource. Data Loss Prevention profiles
/// are a set of entries that can be matched in HTTP bodies or files.
/// They are referenced in Zero Trust Gateway rules.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod dlp_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DlpProfileArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Related DLP policies will trigger when the match count exceeds the number set.
        #[builder(into)]
        pub allowed_match_count: pulumi_wasm_rust::InputOrOutput<i32>,
        /// Scan the context of predefined entries to only return matches surrounded by keywords.
        #[builder(into, default)]
        pub context_awareness: pulumi_wasm_rust::InputOrOutput<
            Option<super::types::DlpProfileContextAwareness>,
        >,
        /// Brief summary of the profile and its intended use.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of entries to apply to the profile.
        #[builder(into)]
        pub entries: pulumi_wasm_rust::InputOrOutput<Vec<super::types::DlpProfileEntry>>,
        /// Name of the profile. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// If true, scan images via OCR to determine if any text present matches filters.
        #[builder(into, default)]
        pub ocr_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The type of the profile. Available values: `custom`, `predefined`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DlpProfileResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Related DLP policies will trigger when the match count exceeds the number set.
        pub allowed_match_count: pulumi_wasm_rust::Output<i32>,
        /// Scan the context of predefined entries to only return matches surrounded by keywords.
        pub context_awareness: pulumi_wasm_rust::Output<
            super::types::DlpProfileContextAwareness,
        >,
        /// Brief summary of the profile and its intended use.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// List of entries to apply to the profile.
        pub entries: pulumi_wasm_rust::Output<Vec<super::types::DlpProfileEntry>>,
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DlpProfileArgs,
    ) -> DlpProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let allowed_match_count_binding = args
            .allowed_match_count
            .get_output(context)
            .get_inner();
        let context_awareness_binding = args
            .context_awareness
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let entries_binding = args.entries.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let ocr_enabled_binding = args.ocr_enabled.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/dlpProfile:DlpProfile".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "allowedMatchCount".into(),
                    value: &allowed_match_count_binding,
                },
                register_interface::ObjectField {
                    name: "contextAwareness".into(),
                    value: &context_awareness_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "entries".into(),
                    value: &entries_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "ocrEnabled".into(),
                    value: &ocr_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "allowedMatchCount".into(),
                },
                register_interface::ResultField {
                    name: "contextAwareness".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "entries".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ocrEnabled".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DlpProfileResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            allowed_match_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowedMatchCount").unwrap(),
            ),
            context_awareness: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contextAwareness").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            entries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("entries").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            ocr_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ocrEnabled").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
