/// An inspect job template.
///
///
/// To get more information about InspectTemplate, see:
///
/// * [API documentation](https://cloud.google.com/dlp/docs/reference/rest/v2/projects.inspectTemplates)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dlp/docs/creating-templates-inspect)
///
/// ## Example Usage
///
/// ### Dlp Inspect Template Basic
///
///
/// ```yaml
/// resources:
///   basic:
///     type: gcp:dataloss:PreventionInspectTemplate
///     properties:
///       parent: projects/my-project-name
///       description: My description
///       displayName: display_name
///       inspectConfig:
///         infoTypes:
///           - name: EMAIL_ADDRESS
///           - name: PERSON_NAME
///           - name: LAST_NAME
///           - name: DOMAIN_NAME
///           - name: PHONE_NUMBER
///           - name: FIRST_NAME
///         minLikelihood: UNLIKELY
///         ruleSets:
///           - infoTypes:
///               - name: EMAIL_ADDRESS
///             rules:
///               - exclusionRule:
///                   regex:
///                     pattern: .+@example.com
///                   matchingType: MATCHING_TYPE_FULL_MATCH
///           - infoTypes:
///               - name: EMAIL_ADDRESS
///               - name: DOMAIN_NAME
///               - name: PHONE_NUMBER
///               - name: PERSON_NAME
///               - name: FIRST_NAME
///             rules:
///               - exclusionRule:
///                   dictionary:
///                     wordList:
///                       words:
///                         - TEST
///                   matchingType: MATCHING_TYPE_PARTIAL_MATCH
///           - infoTypes:
///               - name: PERSON_NAME
///             rules:
///               - hotwordRule:
///                   hotwordRegex:
///                     pattern: patient
///                   proximity:
///                     windowBefore: 50
///                   likelihoodAdjustment:
///                     fixedLikelihood: VERY_LIKELY
///         limits:
///           maxFindingsPerItem: 10
///           maxFindingsPerRequest: 50
///           maxFindingsPerInfoTypes:
///             - maxFindings: '75'
///               infoType:
///                 name: PERSON_NAME
///             - maxFindings: '80'
///               infoType:
///                 name: LAST_NAME
/// ```
/// ### Dlp Inspect Template Custom Type
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let custom = prevention_inspect_template::create(
///         "custom",
///         PreventionInspectTemplateArgs::builder()
///             .description("My description")
///             .display_name("display_name")
///             .inspect_config(
///                 PreventionInspectTemplateInspectConfig::builder()
///                     .customInfoTypes(
///                         vec![
///                             PreventionInspectTemplateInspectConfigCustomInfoType::builder()
///                             .infoType(PreventionInspectTemplateInspectConfigCustomInfoTypeInfoType::builder()
///                             .name("MY_CUSTOM_TYPE").build_struct())
///                             .likelihood("UNLIKELY")
///                             .regex(PreventionInspectTemplateInspectConfigCustomInfoTypeRegex::builder()
///                             .pattern("test*").build_struct()).build_struct(),
///                         ],
///                     )
///                     .infoTypes(
///                         vec![
///                             PreventionInspectTemplateInspectConfigInfoType::builder()
///                             .name("EMAIL_ADDRESS").build_struct(),
///                         ],
///                     )
///                     .limits(
///                         PreventionInspectTemplateInspectConfigLimits::builder()
///                             .maxFindingsPerItem(10)
///                             .maxFindingsPerRequest(50)
///                             .build_struct(),
///                     )
///                     .minLikelihood("UNLIKELY")
///                     .ruleSets(
///                         vec![
///                             PreventionInspectTemplateInspectConfigRuleSet::builder()
///                             .infoTypes(vec![PreventionInspectTemplateInspectConfigRuleSetInfoType::builder()
///                             .name("EMAIL_ADDRESS").build_struct(),])
///                             .rules(vec![PreventionInspectTemplateInspectConfigRuleSetRule::builder()
///                             .exclusionRule(PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRule::builder()
///                             .matchingType("MATCHING_TYPE_FULL_MATCH")
///                             .regex(PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRuleRegex::builder()
///                             .pattern(".+@example.com").build_struct()).build_struct())
///                             .build_struct(),]).build_struct(),
///                             PreventionInspectTemplateInspectConfigRuleSet::builder()
///                             .infoTypes(vec![PreventionInspectTemplateInspectConfigRuleSetInfoType::builder()
///                             .name("MY_CUSTOM_TYPE").build_struct(),])
///                             .rules(vec![PreventionInspectTemplateInspectConfigRuleSetRule::builder()
///                             .hotwordRule(PreventionInspectTemplateInspectConfigRuleSetRuleHotwordRule::builder()
///                             .hotwordRegex(PreventionInspectTemplateInspectConfigRuleSetRuleHotwordRuleHotwordRegex::builder()
///                             .pattern("example*").build_struct())
///                             .likelihoodAdjustment(PreventionInspectTemplateInspectConfigRuleSetRuleHotwordRuleLikelihoodAdjustment::builder()
///                             .fixedLikelihood("VERY_LIKELY").build_struct())
///                             .proximity(PreventionInspectTemplateInspectConfigRuleSetRuleHotwordRuleProximity::builder()
///                             .windowBefore(50).build_struct()).build_struct())
///                             .build_struct(),]).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .parent("projects/my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dlp Inspect Template Custom Type Surrogate
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let customTypeSurrogate = prevention_inspect_template::create(
///         "customTypeSurrogate",
///         PreventionInspectTemplateArgs::builder()
///             .description("My description")
///             .display_name("display_name")
///             .inspect_config(
///                 PreventionInspectTemplateInspectConfig::builder()
///                     .customInfoTypes(
///                         vec![
///                             PreventionInspectTemplateInspectConfigCustomInfoType::builder()
///                             .infoType(PreventionInspectTemplateInspectConfigCustomInfoTypeInfoType::builder()
///                             .name("MY_CUSTOM_TYPE").build_struct())
///                             .likelihood("UNLIKELY")
///                             .surrogateType(PreventionInspectTemplateInspectConfigCustomInfoTypeSurrogateType::builder()
///                             .build_struct()).build_struct(),
///                         ],
///                     )
///                     .infoTypes(
///                         vec![
///                             PreventionInspectTemplateInspectConfigInfoType::builder()
///                             .name("EMAIL_ADDRESS").build_struct(),
///                         ],
///                     )
///                     .limits(
///                         PreventionInspectTemplateInspectConfigLimits::builder()
///                             .maxFindingsPerItem(10)
///                             .maxFindingsPerRequest(50)
///                             .build_struct(),
///                     )
///                     .minLikelihood("UNLIKELY")
///                     .ruleSets(
///                         vec![
///                             PreventionInspectTemplateInspectConfigRuleSet::builder()
///                             .infoTypes(vec![PreventionInspectTemplateInspectConfigRuleSetInfoType::builder()
///                             .name("EMAIL_ADDRESS").build_struct(),])
///                             .rules(vec![PreventionInspectTemplateInspectConfigRuleSetRule::builder()
///                             .exclusionRule(PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRule::builder()
///                             .matchingType("MATCHING_TYPE_FULL_MATCH")
///                             .regex(PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRuleRegex::builder()
///                             .pattern(".+@example.com").build_struct()).build_struct())
///                             .build_struct(),]).build_struct(),
///                             PreventionInspectTemplateInspectConfigRuleSet::builder()
///                             .infoTypes(vec![PreventionInspectTemplateInspectConfigRuleSetInfoType::builder()
///                             .name("MY_CUSTOM_TYPE").build_struct(),])
///                             .rules(vec![PreventionInspectTemplateInspectConfigRuleSetRule::builder()
///                             .hotwordRule(PreventionInspectTemplateInspectConfigRuleSetRuleHotwordRule::builder()
///                             .hotwordRegex(PreventionInspectTemplateInspectConfigRuleSetRuleHotwordRuleHotwordRegex::builder()
///                             .pattern("example*").build_struct())
///                             .likelihoodAdjustment(PreventionInspectTemplateInspectConfigRuleSetRuleHotwordRuleLikelihoodAdjustment::builder()
///                             .fixedLikelihood("VERY_LIKELY").build_struct())
///                             .proximity(PreventionInspectTemplateInspectConfigRuleSetRuleHotwordRuleProximity::builder()
///                             .windowBefore(50).build_struct()).build_struct())
///                             .build_struct(),]).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .parent("projects/my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dlp Inspect Template Max Infotype Per Finding Default
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let maxInfotypePerFindingDefault = prevention_inspect_template::create(
///         "maxInfotypePerFindingDefault",
///         PreventionInspectTemplateArgs::builder()
///             .inspect_config(
///                 PreventionInspectTemplateInspectConfig::builder()
///                     .infoTypes(
///                         vec![
///                             PreventionInspectTemplateInspectConfigInfoType::builder()
///                             .name("EMAIL_ADDRESS").build_struct(),
///                             PreventionInspectTemplateInspectConfigInfoType::builder()
///                             .name("PERSON_NAME").build_struct(),
///                         ],
///                     )
///                     .limits(
///                         PreventionInspectTemplateInspectConfigLimits::builder()
///                             .maxFindingsPerInfoTypes(
///                                 vec![
///                                     PreventionInspectTemplateInspectConfigLimitsMaxFindingsPerInfoType::builder()
///                                     .maxFindings(111).build_struct(),
///                                 ],
///                             )
///                             .maxFindingsPerItem(222)
///                             .maxFindingsPerRequest(333)
///                             .build_struct(),
///                     )
///                     .minLikelihood("UNLIKELY")
///                     .build_struct(),
///             )
///             .parent("projects/my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// InspectTemplate can be imported using any of these accepted formats:
///
/// * `{{parent}}/inspectTemplates/{{name}}`
///
/// * `{{parent}}/{{name}}`
///
/// When using the `pulumi import` command, InspectTemplate can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataloss/preventionInspectTemplate:PreventionInspectTemplate default {{parent}}/inspectTemplates/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataloss/preventionInspectTemplate:PreventionInspectTemplate default {{parent}}/{{name}}
/// ```
///
pub mod prevention_inspect_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PreventionInspectTemplateArgs {
        /// A description of the inspect template.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// User set display name of the inspect template.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The core content of the template.
        /// Structure is documented below.
        #[builder(into, default)]
        pub inspect_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataloss::PreventionInspectTemplateInspectConfig>,
        >,
        /// The parent of the inspect template in any of the following formats:
        /// * `projects/{{project}}`
        /// * `projects/{{project}}/locations/{{location}}`
        /// * `organizations/{{organization_id}}`
        /// * `organizations/{{organization_id}}/locations/{{location}}`
        ///
        ///
        /// - - -
        #[builder(into)]
        pub parent: pulumi_wasm_rust::Output<String>,
        /// The template id can contain uppercase and lowercase letters, numbers, and hyphens;
        /// that is, it must match the regular expression: [a-zA-Z\d-_]+. The maximum length is
        /// 100 characters. Can be empty to allow the system to generate one.
        #[builder(into, default)]
        pub template_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PreventionInspectTemplateResult {
        /// A description of the inspect template.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// User set display name of the inspect template.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The core content of the template.
        /// Structure is documented below.
        pub inspect_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataloss::PreventionInspectTemplateInspectConfig>,
        >,
        /// The resource name of the inspect template. Set by the server.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The parent of the inspect template in any of the following formats:
        /// * `projects/{{project}}`
        /// * `projects/{{project}}/locations/{{location}}`
        /// * `organizations/{{organization_id}}`
        /// * `organizations/{{organization_id}}/locations/{{location}}`
        ///
        ///
        /// - - -
        pub parent: pulumi_wasm_rust::Output<String>,
        /// The template id can contain uppercase and lowercase letters, numbers, and hyphens;
        /// that is, it must match the regular expression: [a-zA-Z\d-_]+. The maximum length is
        /// 100 characters. Can be empty to allow the system to generate one.
        pub template_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: PreventionInspectTemplateArgs,
    ) -> PreventionInspectTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let inspect_config_binding = args.inspect_config.get_inner();
        let parent_binding = args.parent.get_inner();
        let template_id_binding = args.template_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataloss/preventionInspectTemplate:PreventionInspectTemplate"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "inspectConfig".into(),
                    value: &inspect_config_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "templateId".into(),
                    value: &template_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "inspectConfig".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "templateId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PreventionInspectTemplateResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            inspect_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inspectConfig").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            template_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templateId").unwrap(),
            ),
        }
    }
}
