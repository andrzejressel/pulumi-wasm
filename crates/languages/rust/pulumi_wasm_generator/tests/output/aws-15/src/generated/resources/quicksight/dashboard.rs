/// Resource for managing a QuickSight Dashboard.
///
/// ## Example Usage
///
/// ### From Source Template
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = dashboard::create(
///         "example",
///         DashboardArgs::builder()
///             .dashboard_id("example-id")
///             .name("example-name")
///             .source_entity(
///                 DashboardSourceEntity::builder()
///                     .sourceTemplate(
///                         DashboardSourceEntitySourceTemplate::builder()
///                             .arn("${source.arn}")
///                             .dataSetReferences(
///                                 vec![
///                                     DashboardSourceEntitySourceTemplateDataSetReference::builder()
///                                     .dataSetArn("${dataset.arn}").dataSetPlaceholder("1")
///                                     .build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .version_description("version")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Definition
///
/// ```yaml
/// resources:
///   example:
///     type: aws:quicksight:Dashboard
///     properties:
///       dashboardId: example-id
///       name: example-name
///       versionDescription: version
///       definition:
///         dataSetIdentifiersDeclarations:
///           - dataSetArn: ${dataset.arn}
///             identifier: '1'
///         sheets:
///           - title: Example
///             sheetId: Example1
///             visuals:
///               - lineChartVisual:
///                   visualId: LineChart
///                   title:
///                     formatText:
///                       plainText: Line Chart Example
///                   chartConfiguration:
///                     fieldWells:
///                       lineChartAggregatedFieldWells:
///                         categories:
///                           - categoricalDimensionField:
///                               fieldId: '1'
///                               column:
///                                 dataSetIdentifier: '1'
///                                 columnName: Column1
///                         values:
///                           - categoricalMeasureField:
///                               fieldId: '2'
///                               column:
///                                 dataSetIdentifier: '1'
///                                 columnName: Column1
///                               aggregationFunction: COUNT
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a QuickSight Dashboard using the AWS account ID and dashboard ID separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/dashboard:Dashboard example 123456789012,example-id
/// ```
pub mod dashboard {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DashboardArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Identifier for the dashboard.
        #[builder(into)]
        pub dashboard_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Options for publishing the dashboard. See dashboard_publish_options.
        #[builder(into, default)]
        pub dashboard_publish_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::quicksight::DashboardDashboardPublishOptions>,
        >,
        /// Display name for the dashboard.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The parameters for the creation of the dashboard, which you want to use to override the default settings. A dashboard can have any type of parameters, and some parameters might accept multiple values. See parameters.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::quicksight::DashboardParameters>,
        >,
        /// A set of resource permissions on the dashboard. Maximum of 64 items. See permissions.
        #[builder(into, default)]
        pub permissions: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::quicksight::DashboardPermission>>,
        >,
        /// The entity that you are using as a source when you create the dashboard (template). Only one of `definition` or `source_entity` should be configured. See source_entity.
        #[builder(into, default)]
        pub source_entity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::quicksight::DashboardSourceEntity>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Amazon Resource Name (ARN) of the theme that is being used for this dashboard. The theme ARN must exist in the same AWS account where you create the dashboard.
        #[builder(into, default)]
        pub theme_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A description of the current dashboard version being created/updated.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub version_description: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DashboardResult {
        /// ARN of the dashboard.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// AWS account ID.
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        /// The time that the dashboard was created.
        pub created_time: pulumi_wasm_rust::Output<String>,
        /// Identifier for the dashboard.
        pub dashboard_id: pulumi_wasm_rust::Output<String>,
        /// Options for publishing the dashboard. See dashboard_publish_options.
        pub dashboard_publish_options: pulumi_wasm_rust::Output<
            super::super::types::quicksight::DashboardDashboardPublishOptions,
        >,
        pub last_published_time: pulumi_wasm_rust::Output<String>,
        /// The time that the dashboard was last updated.
        pub last_updated_time: pulumi_wasm_rust::Output<String>,
        /// Display name for the dashboard.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The parameters for the creation of the dashboard, which you want to use to override the default settings. A dashboard can have any type of parameters, and some parameters might accept multiple values. See parameters.
        pub parameters: pulumi_wasm_rust::Output<
            super::super::types::quicksight::DashboardParameters,
        >,
        /// A set of resource permissions on the dashboard. Maximum of 64 items. See permissions.
        pub permissions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::quicksight::DashboardPermission>>,
        >,
        /// The entity that you are using as a source when you create the dashboard (template). Only one of `definition` or `source_entity` should be configured. See source_entity.
        pub source_entity: pulumi_wasm_rust::Output<
            Option<super::super::types::quicksight::DashboardSourceEntity>,
        >,
        /// Amazon Resource Name (ARN) of a template that was used to create this dashboard.
        pub source_entity_arn: pulumi_wasm_rust::Output<String>,
        /// The dashboard creation status.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Amazon Resource Name (ARN) of the theme that is being used for this dashboard. The theme ARN must exist in the same AWS account where you create the dashboard.
        pub theme_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// A description of the current dashboard version being created/updated.
        ///
        /// The following arguments are optional:
        pub version_description: pulumi_wasm_rust::Output<String>,
        /// The version number of the dashboard version.
        pub version_number: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DashboardArgs,
    ) -> DashboardResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_account_id_binding = args.aws_account_id.get_output(context).get_inner();
        let dashboard_id_binding = args.dashboard_id.get_output(context).get_inner();
        let dashboard_publish_options_binding = args
            .dashboard_publish_options
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let permissions_binding = args.permissions.get_output(context).get_inner();
        let source_entity_binding = args.source_entity.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let theme_arn_binding = args.theme_arn.get_output(context).get_inner();
        let version_description_binding = args
            .version_description
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:quicksight/dashboard:Dashboard".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "dashboardId".into(),
                    value: &dashboard_id_binding,
                },
                register_interface::ObjectField {
                    name: "dashboardPublishOptions".into(),
                    value: &dashboard_publish_options_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding,
                },
                register_interface::ObjectField {
                    name: "sourceEntity".into(),
                    value: &source_entity_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "themeArn".into(),
                    value: &theme_arn_binding,
                },
                register_interface::ObjectField {
                    name: "versionDescription".into(),
                    value: &version_description_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DashboardResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("awsAccountId"),
            ),
            created_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdTime"),
            ),
            dashboard_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dashboardId"),
            ),
            dashboard_publish_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dashboardPublishOptions"),
            ),
            last_published_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastPublishedTime"),
            ),
            last_updated_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastUpdatedTime"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            parameters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            permissions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("permissions"),
            ),
            source_entity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceEntity"),
            ),
            source_entity_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceEntityArn"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            theme_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("themeArn"),
            ),
            version_description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("versionDescription"),
            ),
            version_number: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("versionNumber"),
            ),
        }
    }
}
