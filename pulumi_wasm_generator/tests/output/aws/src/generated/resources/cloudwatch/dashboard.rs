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
pub mod dashboard {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DashboardArgs {
        /// The detailed information about the dashboard, including what widgets are included and their location on the dashboard. You can read more about the body structure in the [documentation](https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/CloudWatch-Dashboard-Body-Structure.html).
        #[builder(into)]
        pub dashboard_body: pulumi_wasm_rust::Output<String>,
        /// The name of the dashboard.
        #[builder(into)]
        pub dashboard_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DashboardResult {
        /// The Amazon Resource Name (ARN) of the dashboard.
        pub dashboard_arn: pulumi_wasm_rust::Output<String>,
        /// The detailed information about the dashboard, including what widgets are included and their location on the dashboard. You can read more about the body structure in the [documentation](https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/CloudWatch-Dashboard-Body-Structure.html).
        pub dashboard_body: pulumi_wasm_rust::Output<String>,
        /// The name of the dashboard.
        pub dashboard_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DashboardArgs) -> DashboardResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dashboard_body_binding = args.dashboard_body.get_inner();
        let dashboard_name_binding = args.dashboard_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/dashboard:Dashboard".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dashboardBody".into(),
                    value: &dashboard_body_binding,
                },
                register_interface::ObjectField {
                    name: "dashboardName".into(),
                    value: &dashboard_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dashboardArn".into(),
                },
                register_interface::ResultField {
                    name: "dashboardBody".into(),
                },
                register_interface::ResultField {
                    name: "dashboardName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DashboardResult {
            dashboard_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dashboardArn").unwrap(),
            ),
            dashboard_body: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dashboardBody").unwrap(),
            ),
            dashboard_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dashboardName").unwrap(),
            ),
        }
    }
}