/// Provides an AWS Backup Report Plan resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:backup:ReportPlan
///     properties:
///       name: example_name
///       description: example description
///       reportDeliveryChannel:
///         formats:
///           - CSV
///           - JSON
///         s3BucketName: example-bucket-name
///       reportSetting:
///         reportTemplate: RESTORE_JOB_REPORT
///       tags:
///         Name: Example Report Plan
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Backup Report Plan using the `id` which corresponds to the name of the Backup Report Plan. For example:
///
/// ```sh
/// $ pulumi import aws:backup/reportPlan:ReportPlan test <id>
/// ```
pub mod report_plan {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReportPlanArgs {
        /// The description of the report plan with a maximum of 1,024 characters
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The unique name of the report plan. The name must be between 1 and 256 characters, starting with a letter, and consisting of letters, numbers, and underscores.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// An object that contains information about where and how to deliver your reports, specifically your Amazon S3 bucket name, S3 key prefix, and the formats of your reports. Detailed below.
        #[builder(into)]
        pub report_delivery_channel: pulumi_wasm_rust::InputOrOutput<
            super::super::types::backup::ReportPlanReportDeliveryChannel,
        >,
        /// An object that identifies the report template for the report. Reports are built using a report template. Detailed below.
        #[builder(into)]
        pub report_setting: pulumi_wasm_rust::InputOrOutput<
            super::super::types::backup::ReportPlanReportSetting,
        >,
        /// Metadata that you can assign to help organize the report plans you create. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ReportPlanResult {
        /// The ARN of the backup report plan.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The date and time that a report plan is created, in Unix format and Coordinated Universal Time (UTC).
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// The deployment status of a report plan. The statuses are: `CREATE_IN_PROGRESS` | `UPDATE_IN_PROGRESS` | `DELETE_IN_PROGRESS` | `COMPLETED`.
        pub deployment_status: pulumi_wasm_rust::Output<String>,
        /// The description of the report plan with a maximum of 1,024 characters
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique name of the report plan. The name must be between 1 and 256 characters, starting with a letter, and consisting of letters, numbers, and underscores.
        pub name: pulumi_wasm_rust::Output<String>,
        /// An object that contains information about where and how to deliver your reports, specifically your Amazon S3 bucket name, S3 key prefix, and the formats of your reports. Detailed below.
        pub report_delivery_channel: pulumi_wasm_rust::Output<
            super::super::types::backup::ReportPlanReportDeliveryChannel,
        >,
        /// An object that identifies the report template for the report. Reports are built using a report template. Detailed below.
        pub report_setting: pulumi_wasm_rust::Output<
            super::super::types::backup::ReportPlanReportSetting,
        >,
        /// Metadata that you can assign to help organize the report plans you create. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ReportPlanArgs,
    ) -> ReportPlanResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let report_delivery_channel_binding = args
            .report_delivery_channel
            .get_output(context)
            .get_inner();
        let report_setting_binding = args.report_setting.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:backup/reportPlan:ReportPlan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "reportDeliveryChannel".into(),
                    value: &report_delivery_channel_binding,
                },
                register_interface::ObjectField {
                    name: "reportSetting".into(),
                    value: &report_setting_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "deploymentStatus".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "reportDeliveryChannel".into(),
                },
                register_interface::ResultField {
                    name: "reportSetting".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ReportPlanResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            deployment_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentStatus").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            report_delivery_channel: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reportDeliveryChannel").unwrap(),
            ),
            report_setting: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reportSetting").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
