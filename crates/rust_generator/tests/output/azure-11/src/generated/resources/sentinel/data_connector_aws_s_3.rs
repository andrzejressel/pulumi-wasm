/// Manages a AWS S3 Data Connector.
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
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("example-workspace")
///             .resource_group_name("${example.name}")
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleDataConnectorAwsS3 = data_connector_aws_s_3::create(
///         "exampleDataConnectorAwsS3",
///         DataConnectorAwsS3Args::builder()
///             .aws_role_arn("arn:aws:iam::000000000000:role/role1")
///             .destination_table("AWSGuardDuty")
///             .log_analytics_workspace_id(
///                 "${exampleLogAnalyticsWorkspaceOnboarding.workspaceId}",
///             )
///             .name("example")
///             .sqs_urls(vec!["https://sqs.us-east-1.amazonaws.com/000000000000/example",])
///             .build_struct(),
///     );
///     let exampleLogAnalyticsWorkspaceOnboarding = log_analytics_workspace_onboarding::create(
///         "exampleLogAnalyticsWorkspaceOnboarding",
///         LogAnalyticsWorkspaceOnboardingArgs::builder()
///             .workspace_id("${exampleAnalyticsWorkspace.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// AWS S3 Data Connectors can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/dataConnectorAwsS3:DataConnectorAwsS3 example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/dataConnectors/dc1
/// ```
///
pub mod data_connector_aws_s_3 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataConnectorAwsS3Args {
        /// The ARN of the AWS role, which is connected to this AWS CloudTrail Data Connector. See the [Azure document](https://docs.microsoft.com/azure/sentinel/connect-aws?tabs=s3#create-an-aws-assumed-role-and-grant-access-to-the-aws-sentinel-account) for details.
        #[builder(into)]
        pub aws_role_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Log Analytics table that will store the ingested data.
        #[builder(into)]
        pub destination_table: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Log Analytics Workspace that this AWS S3 Data Connector resides in. Changing this forces a new AWS S3 Data Connector to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this AWS S3 Data Connector. Changing this forces a new AWS S3 Data Connector to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies a list of AWS SQS urls for the AWS S3 Data Connector.
        #[builder(into)]
        pub sqs_urls: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct DataConnectorAwsS3Result {
        /// The ARN of the AWS role, which is connected to this AWS CloudTrail Data Connector. See the [Azure document](https://docs.microsoft.com/azure/sentinel/connect-aws?tabs=s3#create-an-aws-assumed-role-and-grant-access-to-the-aws-sentinel-account) for details.
        pub aws_role_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the Log Analytics table that will store the ingested data.
        pub destination_table: pulumi_wasm_rust::Output<String>,
        /// The ID of the Log Analytics Workspace that this AWS S3 Data Connector resides in. Changing this forces a new AWS S3 Data Connector to be created.
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this AWS S3 Data Connector. Changing this forces a new AWS S3 Data Connector to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of AWS SQS urls for the AWS S3 Data Connector.
        pub sqs_urls: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DataConnectorAwsS3Args,
    ) -> DataConnectorAwsS3Result {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_role_arn_binding = args.aws_role_arn.get_output(context).get_inner();
        let destination_table_binding = args
            .destination_table
            .get_output(context)
            .get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let sqs_urls_binding = args.sqs_urls.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/dataConnectorAwsS3:DataConnectorAwsS3".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsRoleArn".into(),
                    value: &aws_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "destinationTable".into(),
                    value: &destination_table_binding,
                },
                register_interface::ObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "sqsUrls".into(),
                    value: &sqs_urls_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DataConnectorAwsS3Result {
            aws_role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("awsRoleArn"),
            ),
            destination_table: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("destinationTable"),
            ),
            log_analytics_workspace_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logAnalyticsWorkspaceId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            sqs_urls: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sqsUrls"),
            ),
        }
    }
}
