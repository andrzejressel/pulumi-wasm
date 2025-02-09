#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_report_plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReportPlanArgs {
        /// Backup report plan name.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Metadata that you can assign to help organize the report plans you create.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetReportPlanResult {
        /// ARN of the backup report plan.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Date and time that a report plan is created, in Unix format and Coordinated Universal Time (UTC).
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        /// Deployment status of a report plan. The statuses are: `CREATE_IN_PROGRESS` | `UPDATE_IN_PROGRESS` | `DELETE_IN_PROGRESS` | `COMPLETED`.
        pub deployment_status: pulumi_gestalt_rust::Output<String>,
        /// Description of the report plan.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An object that contains information about where and how to deliver your reports, specifically your Amazon S3 bucket name, S3 key prefix, and the formats of your reports. Detailed below.
        pub report_delivery_channels: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::backup::GetReportPlanReportDeliveryChannel>,
        >,
        /// An object that identifies the report template for the report. Reports are built using a report template. Detailed below.
        pub report_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::backup::GetReportPlanReportSetting>,
        >,
        /// Metadata that you can assign to help organize the report plans you create.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetReportPlanArgs,
    ) -> GetReportPlanResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:backup/getReportPlan:getReportPlan".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetReportPlanResult {
            arn: o.get_field("arn"),
            creation_time: o.get_field("creationTime"),
            deployment_status: o.get_field("deploymentStatus"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            report_delivery_channels: o.get_field("reportDeliveryChannels"),
            report_settings: o.get_field("reportSettings"),
            tags: o.get_field("tags"),
        }
    }
}
