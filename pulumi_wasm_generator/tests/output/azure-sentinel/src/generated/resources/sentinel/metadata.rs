/// Manages a Sentinel Metadata.
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
pub mod metadata {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MetadataArgs {
        /// An `author` blocks as defined below.
        #[builder(into, default)]
        pub author: pulumi_wasm_rust::Output<
            Option<super::super::types::sentinel::MetadataAuthor>,
        >,
        /// A `category` block as defined below.
        #[builder(into, default)]
        pub category: pulumi_wasm_rust::Output<
            Option<super::super::types::sentinel::MetadataCategory>,
        >,
        /// The ID of the content. Used to identify dependencies and content from solutions or community.
        #[builder(into)]
        pub content_id: pulumi_wasm_rust::Output<String>,
        /// Schema version of the content. Can be used to distinguish between flow based on the schema version.
        #[builder(into, default)]
        pub content_schema_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The Custom version of the content.
        #[builder(into, default)]
        pub custom_version: pulumi_wasm_rust::Output<Option<String>>,
        /// A JSON formatted `dependency` block as defined below. Dependency for the content item, what other content items it requires to work.
        #[builder(into, default)]
        pub dependency: pulumi_wasm_rust::Output<Option<String>>,
        /// The first publish date of solution content item.
        #[builder(into, default)]
        pub first_publish_date: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the icon, this id can be fetched from the solution template.
        #[builder(into, default)]
        pub icon_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The kind of content the metadata is for. Possible values are `AnalyticsRule`, `AnalyticsRuleTemplate`, `AutomationRule`, `AzureFunction`, `DataConnector`, `DataType`, `HuntingQuery`, `InvestigationQuery`, `LogicAppsCustomConnector`, `Parser`, `Playbook`, `PlaybookTemplate`, `Solution`, `Watchlist`, `WatchlistTemplate`, `Workbook` and `WorkbookTemplate`.
        #[builder(into)]
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The last publish date of solution content item.
        #[builder(into, default)]
        pub last_publish_date: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Sentinel Metadata. Changing this forces a new Sentinel Metadata to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the parent resource ID of the content item, which the metadata belongs to.
        #[builder(into)]
        pub parent_id: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of preview image file names. These will be taken from solution artifacts.
        #[builder(into, default)]
        pub preview_images: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies a list of preview image file names used for dark theme. These will be taken from solution artifacts.
        #[builder(into, default)]
        pub preview_images_darks: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies a list of providers for the solution content item.
        #[builder(into, default)]
        pub providers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `source` block as defined below.
        #[builder(into, default)]
        pub source: pulumi_wasm_rust::Output<
            Option<super::super::types::sentinel::MetadataSource>,
        >,
        /// A `support` block as defined below.
        #[builder(into, default)]
        pub support: pulumi_wasm_rust::Output<
            Option<super::super::types::sentinel::MetadataSupport>,
        >,
        /// Specifies a list of tactics the resource covers. Possible values are `Reconnaissance`, `ResourceDevelopment`, `InitialAccess`, `Execution`, `Persistence`, `PrivilegeEscalation`, `DefenseEvasion`, `CredentialAccess`, `Discovery`, `LateralMovement`, `Collection`, `CommandAndControl`, `Exfiltration`, `Impact`, `ImpairProcessControl` and `InhibitResponseFunction`.
        #[builder(into, default)]
        pub threat_analysis_tactics: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies a list of techniques the resource covers.
        #[builder(into, default)]
        pub threat_analysis_techniques: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Version of the content.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Log Analytics Workspace. Changing this forces a new Sentinel Metadata to be created.
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct MetadataResult {
        /// An `author` blocks as defined below.
        pub author: pulumi_wasm_rust::Output<
            Option<super::super::types::sentinel::MetadataAuthor>,
        >,
        /// A `category` block as defined below.
        pub category: pulumi_wasm_rust::Output<
            Option<super::super::types::sentinel::MetadataCategory>,
        >,
        /// The ID of the content. Used to identify dependencies and content from solutions or community.
        pub content_id: pulumi_wasm_rust::Output<String>,
        /// Schema version of the content. Can be used to distinguish between flow based on the schema version.
        pub content_schema_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The Custom version of the content.
        pub custom_version: pulumi_wasm_rust::Output<Option<String>>,
        /// A JSON formatted `dependency` block as defined below. Dependency for the content item, what other content items it requires to work.
        pub dependency: pulumi_wasm_rust::Output<Option<String>>,
        /// The first publish date of solution content item.
        pub first_publish_date: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the icon, this id can be fetched from the solution template.
        pub icon_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The kind of content the metadata is for. Possible values are `AnalyticsRule`, `AnalyticsRuleTemplate`, `AutomationRule`, `AzureFunction`, `DataConnector`, `DataType`, `HuntingQuery`, `InvestigationQuery`, `LogicAppsCustomConnector`, `Parser`, `Playbook`, `PlaybookTemplate`, `Solution`, `Watchlist`, `WatchlistTemplate`, `Workbook` and `WorkbookTemplate`.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The last publish date of solution content item.
        pub last_publish_date: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Sentinel Metadata. Changing this forces a new Sentinel Metadata to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the parent resource ID of the content item, which the metadata belongs to.
        pub parent_id: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of preview image file names. These will be taken from solution artifacts.
        pub preview_images: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies a list of preview image file names used for dark theme. These will be taken from solution artifacts.
        pub preview_images_darks: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies a list of providers for the solution content item.
        pub providers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `source` block as defined below.
        pub source: pulumi_wasm_rust::Output<
            super::super::types::sentinel::MetadataSource,
        >,
        /// A `support` block as defined below.
        pub support: pulumi_wasm_rust::Output<
            Option<super::super::types::sentinel::MetadataSupport>,
        >,
        /// Specifies a list of tactics the resource covers. Possible values are `Reconnaissance`, `ResourceDevelopment`, `InitialAccess`, `Execution`, `Persistence`, `PrivilegeEscalation`, `DefenseEvasion`, `CredentialAccess`, `Discovery`, `LateralMovement`, `Collection`, `CommandAndControl`, `Exfiltration`, `Impact`, `ImpairProcessControl` and `InhibitResponseFunction`.
        pub threat_analysis_tactics: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies a list of techniques the resource covers.
        pub threat_analysis_techniques: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Version of the content.
        pub version: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Log Analytics Workspace. Changing this forces a new Sentinel Metadata to be created.
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MetadataArgs) -> MetadataResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let author_binding = args.author.get_inner();
        let category_binding = args.category.get_inner();
        let content_id_binding = args.content_id.get_inner();
        let content_schema_version_binding = args.content_schema_version.get_inner();
        let custom_version_binding = args.custom_version.get_inner();
        let dependency_binding = args.dependency.get_inner();
        let first_publish_date_binding = args.first_publish_date.get_inner();
        let icon_id_binding = args.icon_id.get_inner();
        let kind_binding = args.kind.get_inner();
        let last_publish_date_binding = args.last_publish_date.get_inner();
        let name_binding = args.name.get_inner();
        let parent_id_binding = args.parent_id.get_inner();
        let preview_images_binding = args.preview_images.get_inner();
        let preview_images_darks_binding = args.preview_images_darks.get_inner();
        let providers_binding = args.providers.get_inner();
        let source_binding = args.source.get_inner();
        let support_binding = args.support.get_inner();
        let threat_analysis_tactics_binding = args.threat_analysis_tactics.get_inner();
        let threat_analysis_techniques_binding = args
            .threat_analysis_techniques
            .get_inner();
        let version_binding = args.version.get_inner();
        let workspace_id_binding = args.workspace_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/metadata:Metadata".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "author".into(),
                },
                register_interface::ResultField {
                    name: "category".into(),
                },
                register_interface::ResultField {
                    name: "contentId".into(),
                },
                register_interface::ResultField {
                    name: "contentSchemaVersion".into(),
                },
                register_interface::ResultField {
                    name: "customVersion".into(),
                },
                register_interface::ResultField {
                    name: "dependency".into(),
                },
                register_interface::ResultField {
                    name: "firstPublishDate".into(),
                },
                register_interface::ResultField {
                    name: "iconId".into(),
                },
                register_interface::ResultField {
                    name: "kind".into(),
                },
                register_interface::ResultField {
                    name: "lastPublishDate".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parentId".into(),
                },
                register_interface::ResultField {
                    name: "previewImages".into(),
                },
                register_interface::ResultField {
                    name: "previewImagesDarks".into(),
                },
                register_interface::ResultField {
                    name: "providers".into(),
                },
                register_interface::ResultField {
                    name: "source".into(),
                },
                register_interface::ResultField {
                    name: "support".into(),
                },
                register_interface::ResultField {
                    name: "threatAnalysisTactics".into(),
                },
                register_interface::ResultField {
                    name: "threatAnalysisTechniques".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
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
        MetadataResult {
            author: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("author").unwrap(),
            ),
            category: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("category").unwrap(),
            ),
            content_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentId").unwrap(),
            ),
            content_schema_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentSchemaVersion").unwrap(),
            ),
            custom_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customVersion").unwrap(),
            ),
            dependency: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dependency").unwrap(),
            ),
            first_publish_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firstPublishDate").unwrap(),
            ),
            icon_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iconId").unwrap(),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
            ),
            last_publish_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastPublishDate").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parentId").unwrap(),
            ),
            preview_images: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("previewImages").unwrap(),
            ),
            preview_images_darks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("previewImagesDarks").unwrap(),
            ),
            providers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providers").unwrap(),
            ),
            source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("source").unwrap(),
            ),
            support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("support").unwrap(),
            ),
            threat_analysis_tactics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("threatAnalysisTactics").unwrap(),
            ),
            threat_analysis_techniques: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("threatAnalysisTechniques").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
        }
    }
}