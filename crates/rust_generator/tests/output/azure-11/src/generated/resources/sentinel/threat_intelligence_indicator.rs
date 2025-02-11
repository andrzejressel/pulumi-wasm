/// Manages a Sentinel Threat Intelligence Indicator.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod threat_intelligence_indicator {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ThreatIntelligenceIndicatorArgs {
        /// Confidence levels of the Threat Intelligence Indicator.
        #[builder(into, default)]
        pub confidence: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The creator of the Threat Intelligence Indicator.
        #[builder(into, default)]
        pub created_by: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description of the Threat Intelligence Indicator.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The display name of the Threat Intelligence Indicator.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The extension config of the Threat Intelligence Indicator in JSON format.
        #[builder(into, default)]
        pub extension: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `external_reference` blocks as defined below.
        #[builder(into, default)]
        pub external_references: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::sentinel::ThreatIntelligenceIndicatorExternalReference,
                >,
            >,
        >,
        /// One or more `granular_marking` blocks as defined below.
        #[builder(into, default)]
        pub granular_markings: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::sentinel::ThreatIntelligenceIndicatorGranularMarking,
                >,
            >,
        >,
        /// One or more `kill_chain_phase` blocks as defined below.
        #[builder(into, default)]
        pub kill_chain_phases: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::sentinel::ThreatIntelligenceIndicatorKillChainPhase,
                >,
            >,
        >,
        /// The language of the Threat Intelligence Indicator.
        #[builder(into, default)]
        pub language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a list of Threat Intelligence marking references.
        #[builder(into, default)]
        pub object_marking_refs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The pattern used by the Threat Intelligence Indicator. When `pattern_type` set to `file`, `pattern` must be specified with `<HashName>:<Value>` format, such as `MD5:78ecc5c05cd8b79af480df2f8fba0b9d`.
        #[builder(into)]
        pub pattern: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of pattern used by the Threat Intelligence Indicator. Possible values are `domain-name`, `file`, `ipv4-addr`, `ipv6-addr` and `url`.
        #[builder(into)]
        pub pattern_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The version of a Threat Intelligence entity.
        #[builder(into, default)]
        pub pattern_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the Threat Intelligence entity revoked.
        #[builder(into, default)]
        pub revoked: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Source of the Threat Intelligence Indicator. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies a list of tags of the Threat Intelligence Indicator.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies a list of threat types of this Threat Intelligence Indicator.
        #[builder(into, default)]
        pub threat_types: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The start of validate date in RFC3339.
        #[builder(into)]
        pub validate_from_utc: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The end of validate date of the Threat Intelligence Indicator in RFC3339 format.
        #[builder(into, default)]
        pub validate_until_utc: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Log Analytics Workspace. Changing this forces a new Sentinel Threat Intelligence Indicator to be created.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ThreatIntelligenceIndicatorResult {
        /// Confidence levels of the Threat Intelligence Indicator.
        pub confidence: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The creator of the Threat Intelligence Indicator.
        pub created_by: pulumi_gestalt_rust::Output<Option<String>>,
        /// The date of this Threat Intelligence Indicator created.
        pub created_on: pulumi_gestalt_rust::Output<String>,
        /// Whether the Threat Intelligence entity is defanged?
        pub defanged: pulumi_gestalt_rust::Output<bool>,
        /// The description of the Threat Intelligence Indicator.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The display name of the Threat Intelligence Indicator.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The extension config of the Threat Intelligence Indicator in JSON format.
        pub extension: pulumi_gestalt_rust::Output<String>,
        /// The external ID of the Threat Intelligence Indicator.
        pub external_id: pulumi_gestalt_rust::Output<String>,
        /// the External last updated time in UTC.
        pub external_last_updated_time_utc: pulumi_gestalt_rust::Output<String>,
        /// One or more `external_reference` blocks as defined below.
        pub external_references: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::sentinel::ThreatIntelligenceIndicatorExternalReference,
                >,
            >,
        >,
        /// One or more `granular_marking` blocks as defined below.
        pub granular_markings: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::sentinel::ThreatIntelligenceIndicatorGranularMarking,
                >,
            >,
        >,
        /// The guid of this Sentinel Threat Intelligence Indicator.
        pub guid: pulumi_gestalt_rust::Output<String>,
        /// A list of indicator types of this Threat Intelligence Indicator.
        pub indicator_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// One or more `kill_chain_phase` blocks as defined below.
        pub kill_chain_phases: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::sentinel::ThreatIntelligenceIndicatorKillChainPhase,
                >,
            >,
        >,
        /// The language of the Threat Intelligence Indicator.
        pub language: pulumi_gestalt_rust::Output<Option<String>>,
        /// The last updated time of the Threat Intelligence Indicator in UTC.
        pub last_updated_time_utc: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of Threat Intelligence marking references.
        pub object_marking_refs: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A `parsed_pattern` block as defined below.
        pub parsed_patterns: pulumi_gestalt_rust::Output<
            Vec<super::super::types::sentinel::ThreatIntelligenceIndicatorParsedPattern>,
        >,
        /// The pattern used by the Threat Intelligence Indicator. When `pattern_type` set to `file`, `pattern` must be specified with `<HashName>:<Value>` format, such as `MD5:78ecc5c05cd8b79af480df2f8fba0b9d`.
        pub pattern: pulumi_gestalt_rust::Output<String>,
        /// The type of pattern used by the Threat Intelligence Indicator. Possible values are `domain-name`, `file`, `ipv4-addr`, `ipv6-addr` and `url`.
        pub pattern_type: pulumi_gestalt_rust::Output<String>,
        /// The version of a Threat Intelligence entity.
        pub pattern_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the Threat Intelligence entity revoked.
        pub revoked: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Source of the Threat Intelligence Indicator. Changing this forces a new resource to be created.
        pub source: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of tags of the Threat Intelligence Indicator.
        pub tags: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies a list of threat types of this Threat Intelligence Indicator.
        pub threat_types: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The start of validate date in RFC3339.
        pub validate_from_utc: pulumi_gestalt_rust::Output<String>,
        /// The end of validate date of the Threat Intelligence Indicator in RFC3339 format.
        pub validate_until_utc: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Log Analytics Workspace. Changing this forces a new Sentinel Threat Intelligence Indicator to be created.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ThreatIntelligenceIndicatorArgs,
    ) -> ThreatIntelligenceIndicatorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let confidence_binding = args.confidence.get_output(context);
        let created_by_binding = args.created_by.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let extension_binding = args.extension.get_output(context);
        let external_references_binding = args.external_references.get_output(context);
        let granular_markings_binding = args.granular_markings.get_output(context);
        let kill_chain_phases_binding = args.kill_chain_phases.get_output(context);
        let language_binding = args.language.get_output(context);
        let object_marking_refs_binding = args.object_marking_refs.get_output(context);
        let pattern_binding = args.pattern.get_output(context);
        let pattern_type_binding = args.pattern_type.get_output(context);
        let pattern_version_binding = args.pattern_version.get_output(context);
        let revoked_binding = args.revoked.get_output(context);
        let source_binding = args.source.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let threat_types_binding = args.threat_types.get_output(context);
        let validate_from_utc_binding = args.validate_from_utc.get_output(context);
        let validate_until_utc_binding = args.validate_until_utc.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:sentinel/threatIntelligenceIndicator:ThreatIntelligenceIndicator"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "confidence".into(),
                    value: &confidence_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createdBy".into(),
                    value: &created_by_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "extension".into(),
                    value: &extension_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "externalReferences".into(),
                    value: &external_references_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "granularMarkings".into(),
                    value: &granular_markings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "killChainPhases".into(),
                    value: &kill_chain_phases_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "language".into(),
                    value: &language_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "objectMarkingRefs".into(),
                    value: &object_marking_refs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pattern".into(),
                    value: &pattern_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "patternType".into(),
                    value: &pattern_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "patternVersion".into(),
                    value: &pattern_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "revoked".into(),
                    value: &revoked_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: &source_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "threatTypes".into(),
                    value: &threat_types_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validateFromUtc".into(),
                    value: &validate_from_utc_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validateUntilUtc".into(),
                    value: &validate_until_utc_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ThreatIntelligenceIndicatorResult {
            confidence: o.get_field("confidence"),
            created_by: o.get_field("createdBy"),
            created_on: o.get_field("createdOn"),
            defanged: o.get_field("defanged"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            extension: o.get_field("extension"),
            external_id: o.get_field("externalId"),
            external_last_updated_time_utc: o.get_field("externalLastUpdatedTimeUtc"),
            external_references: o.get_field("externalReferences"),
            granular_markings: o.get_field("granularMarkings"),
            guid: o.get_field("guid"),
            indicator_types: o.get_field("indicatorTypes"),
            kill_chain_phases: o.get_field("killChainPhases"),
            language: o.get_field("language"),
            last_updated_time_utc: o.get_field("lastUpdatedTimeUtc"),
            object_marking_refs: o.get_field("objectMarkingRefs"),
            parsed_patterns: o.get_field("parsedPatterns"),
            pattern: o.get_field("pattern"),
            pattern_type: o.get_field("patternType"),
            pattern_version: o.get_field("patternVersion"),
            revoked: o.get_field("revoked"),
            source: o.get_field("source"),
            tags: o.get_field("tags"),
            threat_types: o.get_field("threatTypes"),
            validate_from_utc: o.get_field("validateFromUtc"),
            validate_until_utc: o.get_field("validateUntilUtc"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
