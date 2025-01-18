/// Manages a Sentinel Threat Intelligence Indicator.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("east us")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("example-law")
///             .resource_group_name("${example.name}")
///             .retention_in_days(30)
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleLogAnalyticsWorkspaceOnboarding = log_analytics_workspace_onboarding::create(
///         "exampleLogAnalyticsWorkspaceOnboarding",
///         LogAnalyticsWorkspaceOnboardingArgs::builder()
///             .workspace_id("${exampleAnalyticsWorkspace.id}")
///             .build_struct(),
///     );
///     let exampleThreatIntelligenceIndicator = threat_intelligence_indicator::create(
///         "exampleThreatIntelligenceIndicator",
///         ThreatIntelligenceIndicatorArgs::builder()
///             .display_name("example-indicator")
///             .pattern("http://example.com")
///             .pattern_type("domain-name")
///             .source("Microsoft Sentinel")
///             .validate_from_utc("2022-12-14T16:00:00Z")
///             .workspace_id("${exampleLogAnalyticsWorkspaceOnboarding.workspaceId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Sentinel Threat Intelligence Indicators can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/threatIntelligenceIndicator:ThreatIntelligenceIndicator example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourcegroup1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/threatIntelligence/main/indicators/indicator1
/// ```
///
pub mod threat_intelligence_indicator {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ThreatIntelligenceIndicatorArgs {
        /// Confidence levels of the Threat Intelligence Indicator.
        #[builder(into, default)]
        pub confidence: pulumi_wasm_rust::Output<Option<i32>>,
        /// The creator of the Threat Intelligence Indicator.
        #[builder(into, default)]
        pub created_by: pulumi_wasm_rust::Output<Option<String>>,
        /// The description of the Threat Intelligence Indicator.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The display name of the Threat Intelligence Indicator.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The extension config of the Threat Intelligence Indicator in JSON format.
        #[builder(into, default)]
        pub extension: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `external_reference` blocks as defined below.
        #[builder(into, default)]
        pub external_references: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::sentinel::ThreatIntelligenceIndicatorExternalReference,
                >,
            >,
        >,
        /// One or more `granular_marking` blocks as defined below.
        #[builder(into, default)]
        pub granular_markings: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::sentinel::ThreatIntelligenceIndicatorGranularMarking,
                >,
            >,
        >,
        /// One or more `kill_chain_phase` blocks as defined below.
        #[builder(into, default)]
        pub kill_chain_phases: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::sentinel::ThreatIntelligenceIndicatorKillChainPhase,
                >,
            >,
        >,
        /// The language of the Threat Intelligence Indicator.
        #[builder(into, default)]
        pub language: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies a list of Threat Intelligence marking references.
        #[builder(into, default)]
        pub object_marking_refs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The pattern used by the Threat Intelligence Indicator. When `pattern_type` set to `file`, `pattern` must be specified with `<HashName>:<Value>` format, such as `MD5:78ecc5c05cd8b79af480df2f8fba0b9d`.
        #[builder(into)]
        pub pattern: pulumi_wasm_rust::Output<String>,
        /// The type of pattern used by the Threat Intelligence Indicator. Possible values are `domain-name`, `file`, `ipv4-addr`, `ipv6-addr` and `url`.
        #[builder(into)]
        pub pattern_type: pulumi_wasm_rust::Output<String>,
        /// The version of a Threat Intelligence entity.
        #[builder(into, default)]
        pub pattern_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the Threat Intelligence entity revoked.
        #[builder(into, default)]
        pub revoked: pulumi_wasm_rust::Output<Option<bool>>,
        /// Source of the Threat Intelligence Indicator. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of tags of the Threat Intelligence Indicator.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies a list of threat types of this Threat Intelligence Indicator.
        #[builder(into, default)]
        pub threat_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The start of validate date in RFC3339.
        #[builder(into)]
        pub validate_from_utc: pulumi_wasm_rust::Output<String>,
        /// The end of validate date of the Threat Intelligence Indicator in RFC3339 format.
        #[builder(into, default)]
        pub validate_until_utc: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Log Analytics Workspace. Changing this forces a new Sentinel Threat Intelligence Indicator to be created.
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ThreatIntelligenceIndicatorResult {
        /// Confidence levels of the Threat Intelligence Indicator.
        pub confidence: pulumi_wasm_rust::Output<Option<i32>>,
        /// The creator of the Threat Intelligence Indicator.
        pub created_by: pulumi_wasm_rust::Output<Option<String>>,
        /// The date of this Threat Intelligence Indicator created.
        pub created_on: pulumi_wasm_rust::Output<String>,
        /// Whether the Threat Intelligence entity is defanged?
        pub defanged: pulumi_wasm_rust::Output<bool>,
        /// The description of the Threat Intelligence Indicator.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The display name of the Threat Intelligence Indicator.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The extension config of the Threat Intelligence Indicator in JSON format.
        pub extension: pulumi_wasm_rust::Output<String>,
        /// The external ID of the Threat Intelligence Indicator.
        pub external_id: pulumi_wasm_rust::Output<String>,
        /// the External last updated time in UTC.
        pub external_last_updated_time_utc: pulumi_wasm_rust::Output<String>,
        /// One or more `external_reference` blocks as defined below.
        pub external_references: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::sentinel::ThreatIntelligenceIndicatorExternalReference,
                >,
            >,
        >,
        /// One or more `granular_marking` blocks as defined below.
        pub granular_markings: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::sentinel::ThreatIntelligenceIndicatorGranularMarking,
                >,
            >,
        >,
        /// The guid of this Sentinel Threat Intelligence Indicator.
        pub guid: pulumi_wasm_rust::Output<String>,
        /// A list of indicator types of this Threat Intelligence Indicator.
        pub indicator_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// One or more `kill_chain_phase` blocks as defined below.
        pub kill_chain_phases: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::sentinel::ThreatIntelligenceIndicatorKillChainPhase,
                >,
            >,
        >,
        /// The language of the Threat Intelligence Indicator.
        pub language: pulumi_wasm_rust::Output<Option<String>>,
        /// The last updated time of the Threat Intelligence Indicator in UTC.
        pub last_updated_time_utc: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of Threat Intelligence marking references.
        pub object_marking_refs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `parsed_pattern` block as defined below.
        pub parsed_patterns: pulumi_wasm_rust::Output<
            Vec<super::super::types::sentinel::ThreatIntelligenceIndicatorParsedPattern>,
        >,
        /// The pattern used by the Threat Intelligence Indicator. When `pattern_type` set to `file`, `pattern` must be specified with `<HashName>:<Value>` format, such as `MD5:78ecc5c05cd8b79af480df2f8fba0b9d`.
        pub pattern: pulumi_wasm_rust::Output<String>,
        /// The type of pattern used by the Threat Intelligence Indicator. Possible values are `domain-name`, `file`, `ipv4-addr`, `ipv6-addr` and `url`.
        pub pattern_type: pulumi_wasm_rust::Output<String>,
        /// The version of a Threat Intelligence entity.
        pub pattern_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the Threat Intelligence entity revoked.
        pub revoked: pulumi_wasm_rust::Output<Option<bool>>,
        /// Source of the Threat Intelligence Indicator. Changing this forces a new resource to be created.
        pub source: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of tags of the Threat Intelligence Indicator.
        pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies a list of threat types of this Threat Intelligence Indicator.
        pub threat_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The start of validate date in RFC3339.
        pub validate_from_utc: pulumi_wasm_rust::Output<String>,
        /// The end of validate date of the Threat Intelligence Indicator in RFC3339 format.
        pub validate_until_utc: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Log Analytics Workspace. Changing this forces a new Sentinel Threat Intelligence Indicator to be created.
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ThreatIntelligenceIndicatorArgs,
    ) -> ThreatIntelligenceIndicatorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let confidence_binding = args.confidence.get_inner();
        let created_by_binding = args.created_by.get_inner();
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let extension_binding = args.extension.get_inner();
        let external_references_binding = args.external_references.get_inner();
        let granular_markings_binding = args.granular_markings.get_inner();
        let kill_chain_phases_binding = args.kill_chain_phases.get_inner();
        let language_binding = args.language.get_inner();
        let object_marking_refs_binding = args.object_marking_refs.get_inner();
        let pattern_binding = args.pattern.get_inner();
        let pattern_type_binding = args.pattern_type.get_inner();
        let pattern_version_binding = args.pattern_version.get_inner();
        let revoked_binding = args.revoked.get_inner();
        let source_binding = args.source.get_inner();
        let tags_binding = args.tags.get_inner();
        let threat_types_binding = args.threat_types.get_inner();
        let validate_from_utc_binding = args.validate_from_utc.get_inner();
        let validate_until_utc_binding = args.validate_until_utc.get_inner();
        let workspace_id_binding = args.workspace_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/threatIntelligenceIndicator:ThreatIntelligenceIndicator"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "confidence".into(),
                    value: &confidence_binding,
                },
                register_interface::ObjectField {
                    name: "createdBy".into(),
                    value: &created_by_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "extension".into(),
                    value: &extension_binding,
                },
                register_interface::ObjectField {
                    name: "externalReferences".into(),
                    value: &external_references_binding,
                },
                register_interface::ObjectField {
                    name: "granularMarkings".into(),
                    value: &granular_markings_binding,
                },
                register_interface::ObjectField {
                    name: "killChainPhases".into(),
                    value: &kill_chain_phases_binding,
                },
                register_interface::ObjectField {
                    name: "language".into(),
                    value: &language_binding,
                },
                register_interface::ObjectField {
                    name: "objectMarkingRefs".into(),
                    value: &object_marking_refs_binding,
                },
                register_interface::ObjectField {
                    name: "pattern".into(),
                    value: &pattern_binding,
                },
                register_interface::ObjectField {
                    name: "patternType".into(),
                    value: &pattern_type_binding,
                },
                register_interface::ObjectField {
                    name: "patternVersion".into(),
                    value: &pattern_version_binding,
                },
                register_interface::ObjectField {
                    name: "revoked".into(),
                    value: &revoked_binding,
                },
                register_interface::ObjectField {
                    name: "source".into(),
                    value: &source_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "threatTypes".into(),
                    value: &threat_types_binding,
                },
                register_interface::ObjectField {
                    name: "validateFromUtc".into(),
                    value: &validate_from_utc_binding,
                },
                register_interface::ObjectField {
                    name: "validateUntilUtc".into(),
                    value: &validate_until_utc_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "confidence".into(),
                },
                register_interface::ResultField {
                    name: "createdBy".into(),
                },
                register_interface::ResultField {
                    name: "createdOn".into(),
                },
                register_interface::ResultField {
                    name: "defanged".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "extension".into(),
                },
                register_interface::ResultField {
                    name: "externalId".into(),
                },
                register_interface::ResultField {
                    name: "externalLastUpdatedTimeUtc".into(),
                },
                register_interface::ResultField {
                    name: "externalReferences".into(),
                },
                register_interface::ResultField {
                    name: "granularMarkings".into(),
                },
                register_interface::ResultField {
                    name: "guid".into(),
                },
                register_interface::ResultField {
                    name: "indicatorTypes".into(),
                },
                register_interface::ResultField {
                    name: "killChainPhases".into(),
                },
                register_interface::ResultField {
                    name: "language".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedTimeUtc".into(),
                },
                register_interface::ResultField {
                    name: "objectMarkingRefs".into(),
                },
                register_interface::ResultField {
                    name: "parsedPatterns".into(),
                },
                register_interface::ResultField {
                    name: "pattern".into(),
                },
                register_interface::ResultField {
                    name: "patternType".into(),
                },
                register_interface::ResultField {
                    name: "patternVersion".into(),
                },
                register_interface::ResultField {
                    name: "revoked".into(),
                },
                register_interface::ResultField {
                    name: "source".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "threatTypes".into(),
                },
                register_interface::ResultField {
                    name: "validateFromUtc".into(),
                },
                register_interface::ResultField {
                    name: "validateUntilUtc".into(),
                },
                register_interface::ResultField {
                    name: "workspaceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ThreatIntelligenceIndicatorResult {
            confidence: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("confidence").unwrap(),
            ),
            created_by: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdBy").unwrap(),
            ),
            created_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdOn").unwrap(),
            ),
            defanged: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defanged").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            extension: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extension").unwrap(),
            ),
            external_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalId").unwrap(),
            ),
            external_last_updated_time_utc: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalLastUpdatedTimeUtc").unwrap(),
            ),
            external_references: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalReferences").unwrap(),
            ),
            granular_markings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("granularMarkings").unwrap(),
            ),
            guid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("guid").unwrap(),
            ),
            indicator_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("indicatorTypes").unwrap(),
            ),
            kill_chain_phases: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("killChainPhases").unwrap(),
            ),
            language: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("language").unwrap(),
            ),
            last_updated_time_utc: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedTimeUtc").unwrap(),
            ),
            object_marking_refs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("objectMarkingRefs").unwrap(),
            ),
            parsed_patterns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parsedPatterns").unwrap(),
            ),
            pattern: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pattern").unwrap(),
            ),
            pattern_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("patternType").unwrap(),
            ),
            pattern_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("patternVersion").unwrap(),
            ),
            revoked: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revoked").unwrap(),
            ),
            source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("source").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            threat_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("threatTypes").unwrap(),
            ),
            validate_from_utc: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validateFromUtc").unwrap(),
            ),
            validate_until_utc: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validateUntilUtc").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
        }
    }
}
