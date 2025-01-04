/// Provides a resource to manage an Amazon Kinesis Streams resource policy.
/// Use a resource policy to manage cross-account access to your data streams or consumers.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod resource_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourcePolicyArgs {
        /// The policy document.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the data stream or consumer.
        #[builder(into)]
        pub resource_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ResourcePolicyResult {
        /// The policy document.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the data stream or consumer.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ResourcePolicyArgs) -> ResourcePolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_binding = args.policy.get_inner();
        let resource_arn_binding = args.resource_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:kinesis/resourcePolicy:ResourcePolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "resourceArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResourcePolicyResult {
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArn").unwrap(),
            ),
        }
    }
}
