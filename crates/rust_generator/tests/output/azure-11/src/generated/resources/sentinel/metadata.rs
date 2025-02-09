/// Manages a Sentinel Metadata.
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
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAlertRuleNrt = alert_rule_nrt::create(
///         "exampleAlertRuleNrt",
///         AlertRuleNrtArgs::builder()
///             .display_name("example")
///             .log_analytics_workspace_id(
///                 "${exampleAnalyticsSolution.workspaceResourceId}",
///             )
///             .name("example")
///             .query(
///                 "AzureActivity |\n  where OperationName == \"Create or Update Virtual Machine\" or OperationName ==\"Create Deployment\" |\n  where ActivityStatus == \"Succeeded\" |\n  make-series dcount(ResourceId) default=0 on EventSubmissionTimestamp in range(ago(7d), now(), 1d) by Caller\n",
///             )
///             .severity("High")
///             .build_struct(),
///     );
///     let exampleAnalyticsSolution = analytics_solution::create(
///         "exampleAnalyticsSolution",
///         AnalyticsSolutionArgs::builder()
///             .location("${example.location}")
///             .plan(
///                 AnalyticsSolutionPlan::builder()
///                     .product("OMSGallery/SecurityInsights")
///                     .publisher("Microsoft")
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .solution_name("SecurityInsights")
///             .workspace_name("${exampleAnalyticsWorkspace.name}")
///             .workspace_resource_id("${exampleAnalyticsWorkspace.id}")
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("example-workspace")
///             .resource_group_name("${example.name}")
///             .sku("pergb2018")
///             .build_struct(),
///     );
///     let exampleMetadata = metadata::create(
///         "exampleMetadata",
///         MetadataArgs::builder()
///             .content_id("${exampleAlertRuleNrt.name}")
///             .kind("AnalyticsRule")
///             .name("exampl")
///             .parent_id("${exampleAlertRuleNrt.id}")
///             .workspace_id("${exampleAnalyticsSolution.workspaceResourceId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Sentinel Metadata can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/metadata:Metadata example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourcegroup1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/metadata/metadata1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod metadata {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MetadataArgs {
        /// An `author` blocks as defined below.
        #[builder(into, default)]
        pub author: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sentinel::MetadataAuthor>,
        >,
        /// A `category` block as defined below.
        #[builder(into, default)]
        pub category: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sentinel::MetadataCategory>,
        >,
        /// The ID of the content. Used to identify dependencies and content from solutions or community.
        #[builder(into)]
        pub content_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Schema version of the content. Can be used to distinguish between flow based on the schema version.
        #[builder(into, default)]
        pub content_schema_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Custom version of the content.
        #[builder(into, default)]
        pub custom_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A JSON formatted `dependency` block as defined below. Dependency for the content item, what other content items it requires to work.
        #[builder(into, default)]
        pub dependency: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The first publish date of solution content item.
        #[builder(into, default)]
        pub first_publish_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the icon, this id can be fetched from the solution template.
        #[builder(into, default)]
        pub icon_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The kind of content the metadata is for. Possible values are `AnalyticsRule`, `AnalyticsRuleTemplate`, `AutomationRule`, `AzureFunction`, `DataConnector`, `DataType`, `HuntingQuery`, `InvestigationQuery`, `LogicAppsCustomConnector`, `Parser`, `Playbook`, `PlaybookTemplate`, `Solution`, `Watchlist`, `WatchlistTemplate`, `Workbook` and `WorkbookTemplate`.
        #[builder(into)]
        pub kind: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The last publish date of solution content item.
        #[builder(into, default)]
        pub last_publish_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Sentinel Metadata. Changing this forces a new Sentinel Metadata to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the parent resource ID of the content item, which the metadata belongs to.
        #[builder(into)]
        pub parent_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies a list of preview image file names. These will be taken from solution artifacts.
        #[builder(into, default)]
        pub preview_images: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies a list of preview image file names used for dark theme. These will be taken from solution artifacts.
        #[builder(into, default)]
        pub preview_images_darks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Specifies a list of providers for the solution content item.
        #[builder(into, default)]
        pub providers: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A `source` block as defined below.
        #[builder(into, default)]
        pub source: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sentinel::MetadataSource>,
        >,
        /// A `support` block as defined below.
        #[builder(into, default)]
        pub support: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sentinel::MetadataSupport>,
        >,
        /// Specifies a list of tactics the resource covers. Possible values are `Reconnaissance`, `ResourceDevelopment`, `InitialAccess`, `Execution`, `Persistence`, `PrivilegeEscalation`, `DefenseEvasion`, `CredentialAccess`, `Discovery`, `LateralMovement`, `Collection`, `CommandAndControl`, `Exfiltration`, `Impact`, `ImpairProcessControl` and `InhibitResponseFunction`.
        #[builder(into, default)]
        pub threat_analysis_tactics: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Specifies a list of techniques the resource covers.
        #[builder(into, default)]
        pub threat_analysis_techniques: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Version of the content.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Log Analytics Workspace. Changing this forces a new Sentinel Metadata to be created.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MetadataResult {
        /// An `author` blocks as defined below.
        pub author: pulumi_gestalt_rust::Output<
            Option<super::super::types::sentinel::MetadataAuthor>,
        >,
        /// A `category` block as defined below.
        pub category: pulumi_gestalt_rust::Output<
            Option<super::super::types::sentinel::MetadataCategory>,
        >,
        /// The ID of the content. Used to identify dependencies and content from solutions or community.
        pub content_id: pulumi_gestalt_rust::Output<String>,
        /// Schema version of the content. Can be used to distinguish between flow based on the schema version.
        pub content_schema_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Custom version of the content.
        pub custom_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// A JSON formatted `dependency` block as defined below. Dependency for the content item, what other content items it requires to work.
        pub dependency: pulumi_gestalt_rust::Output<Option<String>>,
        /// The first publish date of solution content item.
        pub first_publish_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the icon, this id can be fetched from the solution template.
        pub icon_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The kind of content the metadata is for. Possible values are `AnalyticsRule`, `AnalyticsRuleTemplate`, `AutomationRule`, `AzureFunction`, `DataConnector`, `DataType`, `HuntingQuery`, `InvestigationQuery`, `LogicAppsCustomConnector`, `Parser`, `Playbook`, `PlaybookTemplate`, `Solution`, `Watchlist`, `WatchlistTemplate`, `Workbook` and `WorkbookTemplate`.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The last publish date of solution content item.
        pub last_publish_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Sentinel Metadata. Changing this forces a new Sentinel Metadata to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the parent resource ID of the content item, which the metadata belongs to.
        pub parent_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of preview image file names. These will be taken from solution artifacts.
        pub preview_images: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies a list of preview image file names used for dark theme. These will be taken from solution artifacts.
        pub preview_images_darks: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies a list of providers for the solution content item.
        pub providers: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A `source` block as defined below.
        pub source: pulumi_gestalt_rust::Output<
            super::super::types::sentinel::MetadataSource,
        >,
        /// A `support` block as defined below.
        pub support: pulumi_gestalt_rust::Output<
            Option<super::super::types::sentinel::MetadataSupport>,
        >,
        /// Specifies a list of tactics the resource covers. Possible values are `Reconnaissance`, `ResourceDevelopment`, `InitialAccess`, `Execution`, `Persistence`, `PrivilegeEscalation`, `DefenseEvasion`, `CredentialAccess`, `Discovery`, `LateralMovement`, `Collection`, `CommandAndControl`, `Exfiltration`, `Impact`, `ImpairProcessControl` and `InhibitResponseFunction`.
        pub threat_analysis_tactics: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies a list of techniques the resource covers.
        pub threat_analysis_techniques: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Version of the content.
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Log Analytics Workspace. Changing this forces a new Sentinel Metadata to be created.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MetadataArgs,
    ) -> MetadataResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let author_binding = args.author.get_output(context);
        let category_binding = args.category.get_output(context);
        let content_id_binding = args.content_id.get_output(context);
        let content_schema_version_binding = args
            .content_schema_version
            .get_output(context);
        let custom_version_binding = args.custom_version.get_output(context);
        let dependency_binding = args.dependency.get_output(context);
        let first_publish_date_binding = args.first_publish_date.get_output(context);
        let icon_id_binding = args.icon_id.get_output(context);
        let kind_binding = args.kind.get_output(context);
        let last_publish_date_binding = args.last_publish_date.get_output(context);
        let name_binding = args.name.get_output(context);
        let parent_id_binding = args.parent_id.get_output(context);
        let preview_images_binding = args.preview_images.get_output(context);
        let preview_images_darks_binding = args.preview_images_darks.get_output(context);
        let providers_binding = args.providers.get_output(context);
        let source_binding = args.source.get_output(context);
        let support_binding = args.support.get_output(context);
        let threat_analysis_tactics_binding = args
            .threat_analysis_tactics
            .get_output(context);
        let threat_analysis_techniques_binding = args
            .threat_analysis_techniques
            .get_output(context);
        let version_binding = args.version.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:sentinel/metadata:Metadata".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "author".into(),
                    value: author_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "category".into(),
                    value: category_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentId".into(),
                    value: content_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentSchemaVersion".into(),
                    value: content_schema_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customVersion".into(),
                    value: custom_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dependency".into(),
                    value: dependency_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firstPublishDate".into(),
                    value: first_publish_date_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iconId".into(),
                    value: icon_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kind".into(),
                    value: kind_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lastPublishDate".into(),
                    value: last_publish_date_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentId".into(),
                    value: parent_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "previewImages".into(),
                    value: preview_images_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "previewImagesDarks".into(),
                    value: preview_images_darks_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "providers".into(),
                    value: providers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "support".into(),
                    value: support_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "threatAnalysisTactics".into(),
                    value: threat_analysis_tactics_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "threatAnalysisTechniques".into(),
                    value: threat_analysis_techniques_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: workspace_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MetadataResult {
            author: o.get_field("author"),
            category: o.get_field("category"),
            content_id: o.get_field("contentId"),
            content_schema_version: o.get_field("contentSchemaVersion"),
            custom_version: o.get_field("customVersion"),
            dependency: o.get_field("dependency"),
            first_publish_date: o.get_field("firstPublishDate"),
            icon_id: o.get_field("iconId"),
            kind: o.get_field("kind"),
            last_publish_date: o.get_field("lastPublishDate"),
            name: o.get_field("name"),
            parent_id: o.get_field("parentId"),
            preview_images: o.get_field("previewImages"),
            preview_images_darks: o.get_field("previewImagesDarks"),
            providers: o.get_field("providers"),
            source: o.get_field("source"),
            support: o.get_field("support"),
            threat_analysis_tactics: o.get_field("threatAnalysisTactics"),
            threat_analysis_techniques: o.get_field("threatAnalysisTechniques"),
            version: o.get_field("version"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
