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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MetadataArgs,
    ) -> MetadataResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let author_binding_1 = args.author.get_output(context);
        let author_binding = author_binding_1.get_inner();
        let category_binding_1 = args.category.get_output(context);
        let category_binding = category_binding_1.get_inner();
        let content_id_binding_1 = args.content_id.get_output(context);
        let content_id_binding = content_id_binding_1.get_inner();
        let content_schema_version_binding_1 = args
            .content_schema_version
            .get_output(context);
        let content_schema_version_binding = content_schema_version_binding_1
            .get_inner();
        let custom_version_binding_1 = args.custom_version.get_output(context);
        let custom_version_binding = custom_version_binding_1.get_inner();
        let dependency_binding_1 = args.dependency.get_output(context);
        let dependency_binding = dependency_binding_1.get_inner();
        let first_publish_date_binding_1 = args.first_publish_date.get_output(context);
        let first_publish_date_binding = first_publish_date_binding_1.get_inner();
        let icon_id_binding_1 = args.icon_id.get_output(context);
        let icon_id_binding = icon_id_binding_1.get_inner();
        let kind_binding_1 = args.kind.get_output(context);
        let kind_binding = kind_binding_1.get_inner();
        let last_publish_date_binding_1 = args.last_publish_date.get_output(context);
        let last_publish_date_binding = last_publish_date_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let parent_id_binding_1 = args.parent_id.get_output(context);
        let parent_id_binding = parent_id_binding_1.get_inner();
        let preview_images_binding_1 = args.preview_images.get_output(context);
        let preview_images_binding = preview_images_binding_1.get_inner();
        let preview_images_darks_binding_1 = args
            .preview_images_darks
            .get_output(context);
        let preview_images_darks_binding = preview_images_darks_binding_1.get_inner();
        let providers_binding_1 = args.providers.get_output(context);
        let providers_binding = providers_binding_1.get_inner();
        let source_binding_1 = args.source.get_output(context);
        let source_binding = source_binding_1.get_inner();
        let support_binding_1 = args.support.get_output(context);
        let support_binding = support_binding_1.get_inner();
        let threat_analysis_tactics_binding_1 = args
            .threat_analysis_tactics
            .get_output(context);
        let threat_analysis_tactics_binding = threat_analysis_tactics_binding_1
            .get_inner();
        let threat_analysis_techniques_binding_1 = args
            .threat_analysis_techniques
            .get_output(context);
        let threat_analysis_techniques_binding = threat_analysis_techniques_binding_1
            .get_inner();
        let version_binding_1 = args.version.get_output(context);
        let version_binding = version_binding_1.get_inner();
        let workspace_id_binding_1 = args.workspace_id.get_output(context);
        let workspace_id_binding = workspace_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/metadata:Metadata".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "author".into(),
                    value: &author_binding,
                },
                register_interface::ObjectField {
                    name: "category".into(),
                    value: &category_binding,
                },
                register_interface::ObjectField {
                    name: "contentId".into(),
                    value: &content_id_binding,
                },
                register_interface::ObjectField {
                    name: "contentSchemaVersion".into(),
                    value: &content_schema_version_binding,
                },
                register_interface::ObjectField {
                    name: "customVersion".into(),
                    value: &custom_version_binding,
                },
                register_interface::ObjectField {
                    name: "dependency".into(),
                    value: &dependency_binding,
                },
                register_interface::ObjectField {
                    name: "firstPublishDate".into(),
                    value: &first_publish_date_binding,
                },
                register_interface::ObjectField {
                    name: "iconId".into(),
                    value: &icon_id_binding,
                },
                register_interface::ObjectField {
                    name: "kind".into(),
                    value: &kind_binding,
                },
                register_interface::ObjectField {
                    name: "lastPublishDate".into(),
                    value: &last_publish_date_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parentId".into(),
                    value: &parent_id_binding,
                },
                register_interface::ObjectField {
                    name: "previewImages".into(),
                    value: &preview_images_binding,
                },
                register_interface::ObjectField {
                    name: "previewImagesDarks".into(),
                    value: &preview_images_darks_binding,
                },
                register_interface::ObjectField {
                    name: "providers".into(),
                    value: &providers_binding,
                },
                register_interface::ObjectField {
                    name: "source".into(),
                    value: &source_binding,
                },
                register_interface::ObjectField {
                    name: "support".into(),
                    value: &support_binding,
                },
                register_interface::ObjectField {
                    name: "threatAnalysisTactics".into(),
                    value: &threat_analysis_tactics_binding,
                },
                register_interface::ObjectField {
                    name: "threatAnalysisTechniques".into(),
                    value: &threat_analysis_techniques_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MetadataResult {
            author: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("author"),
            ),
            category: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("category"),
            ),
            content_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentId"),
            ),
            content_schema_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentSchemaVersion"),
            ),
            custom_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customVersion"),
            ),
            dependency: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dependency"),
            ),
            first_publish_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("firstPublishDate"),
            ),
            icon_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iconId"),
            ),
            kind: pulumi_gestalt_rust::__private::into_domain(o.extract_field("kind")),
            last_publish_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastPublishDate"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parent_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parentId"),
            ),
            preview_images: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("previewImages"),
            ),
            preview_images_darks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("previewImagesDarks"),
            ),
            providers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("providers"),
            ),
            source: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("source"),
            ),
            support: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("support"),
            ),
            threat_analysis_tactics: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("threatAnalysisTactics"),
            ),
            threat_analysis_techniques: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("threatAnalysisTechniques"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceId"),
            ),
        }
    }
}
