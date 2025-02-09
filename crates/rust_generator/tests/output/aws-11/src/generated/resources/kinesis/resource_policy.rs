/// Provides a resource to manage an Amazon Kinesis Streams resource policy.
/// Use a resource policy to manage cross-account access to your data streams or consumers.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_policy::create(
///         "example",
///         ResourcePolicyArgs::builder()
///             .policy(
///                 "{\n  \"Version\": \"2012-10-17\",\n  \"Id\": \"writePolicy\",\n  \"Statement\": [{\n    \"Sid\": \"writestatement\",\n    \"Effect\": \"Allow\",\n    \"Principal\": {\n      \"AWS\": \"123456789456\"\n    },\n    \"Action\": [\n      \"kinesis:DescribeStreamSummary\",\n      \"kinesis:ListShards\",\n      \"kinesis:PutRecord\",\n      \"kinesis:PutRecords\"\n    ],\n    \"Resource\": \"${exampleAwsKinesisStream.arn}\"\n  }]\n}",
///             )
///             .resource_arn("${exampleAwsKinesisStream.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Kinesis resource policies using the `resource_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:kinesis/resourcePolicy:ResourcePolicy example arn:aws:kinesis:us-west-2:123456789012:stream/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourcePolicyArgs {
        /// The policy document.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the data stream or consumer.
        #[builder(into)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourcePolicyResult {
        /// The policy document.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the data stream or consumer.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourcePolicyArgs,
    ) -> ResourcePolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_binding = args.policy.get_output(context);
        let resource_arn_binding = args.resource_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kinesis/resourcePolicy:ResourcePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceArn".into(),
                    value: resource_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourcePolicyResult {
            policy: o.get_field("policy"),
            resource_arn: o.get_field("resourceArn"),
        }
    }
}
