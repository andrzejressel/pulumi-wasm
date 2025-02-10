/// Provides a CloudWatch Dashboard resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   main:
///     type: aws:cloudwatch:Dashboard
///     properties:
///       dashboardName: my-dashboard
///       dashboardBody:
///         fn::toJSON:
///           widgets:
///             - type: metric
///               x: 0
///               y: 0
///               width: 12
///               height: 6
///               properties:
///                 metrics:
///                   - - AWS/EC2
///                     - CPUUtilization
///                     - InstanceId
///                     - i-012345
///                 period: 300
///                 stat: Average
///                 region: us-east-1
///                 title: EC2 Instance CPU
///             - type: text
///               x: 0
///               y: 7
///               width: 3
///               height: 3
///               properties:
///                 markdown: Hello world
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch dashboards using the `dashboard_name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/dashboard:Dashboard sample dashboard_name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dashboard {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DashboardArgs {
        /// The detailed information about the dashboard, including what widgets are included and their location on the dashboard. You can read more about the body structure in the [documentation](https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/CloudWatch-Dashboard-Body-Structure.html).
        #[builder(into)]
        pub dashboard_body: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the dashboard.
        #[builder(into)]
        pub dashboard_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DashboardResult {
        /// The Amazon Resource Name (ARN) of the dashboard.
        pub dashboard_arn: pulumi_gestalt_rust::Output<String>,
        /// The detailed information about the dashboard, including what widgets are included and their location on the dashboard. You can read more about the body structure in the [documentation](https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/CloudWatch-Dashboard-Body-Structure.html).
        pub dashboard_body: pulumi_gestalt_rust::Output<String>,
        /// The name of the dashboard.
        pub dashboard_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DashboardArgs,
    ) -> DashboardResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dashboard_body_binding = args.dashboard_body.get_output(context);
        let dashboard_name_binding = args.dashboard_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/dashboard:Dashboard".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dashboardBody".into(),
                    value: dashboard_body_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dashboardName".into(),
                    value: dashboard_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DashboardResult {
            dashboard_arn: o.get_field("dashboardArn"),
            dashboard_body: o.get_field("dashboardBody"),
            dashboard_name: o.get_field("dashboardName"),
        }
    }
}
