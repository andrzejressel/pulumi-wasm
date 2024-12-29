/// Provides a CloudFront real-time log configuration resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let assumeRole = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["sts:AssumeRole",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["cloudfront.amazonaws.com",]). type ("Service")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let example = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["kinesis:DescribeStreamSummary",
///                     "kinesis:DescribeStream", "kinesis:PutRecord",
///                     "kinesis:PutRecords",]).effect("Allow")
///                     .resources(vec!["${exampleAwsKinesisStream.arn}",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleRealtimeLogConfig = realtime_log_config::create(
///         "exampleRealtimeLogConfig",
///         RealtimeLogConfigArgs::builder()
///             .endpoint(
///                 RealtimeLogConfigEndpoint::builder()
///                     .kinesisStreamConfig(
///                         RealtimeLogConfigEndpointKinesisStreamConfig::builder()
///                             .roleArn("${exampleRole.arn}")
///                             .streamArn("${exampleAwsKinesisStream.arn}")
///                             .build_struct(),
///                     )
///                     .streamType("Kinesis")
///                     .build_struct(),
///             )
///             .fields(vec!["timestamp", "c-ip",])
///             .name("example")
///             .sampling_rate(75)
///             .build_struct(),
///     );
///     let exampleRole = role::create(
///         "exampleRole",
///         RoleArgs::builder()
///             .assume_role_policy("${assumeRole.json}")
///             .name("cloudfront-realtime-log-config-example")
///             .build_struct(),
///     );
///     let exampleRolePolicy = role_policy::create(
///         "exampleRolePolicy",
///         RolePolicyArgs::builder()
///             .name("cloudfront-realtime-log-config-example")
///             .policy("${example.json}")
///             .role("${exampleRole.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudFront real-time log configurations using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/realtimeLogConfig:RealtimeLogConfig example arn:aws:cloudfront::111122223333:realtime-log-config/ExampleNameForRealtimeLogConfig
/// ```
pub mod realtime_log_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RealtimeLogConfigArgs {
        /// The Amazon Kinesis data streams where real-time log data is sent.
        #[builder(into)]
        pub endpoint: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::RealtimeLogConfigEndpoint,
        >,
        /// The fields that are included in each real-time log record. See the [AWS documentation](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-fields) for supported values.
        #[builder(into)]
        pub fields: pulumi_wasm_rust::Output<Vec<String>>,
        /// The unique name to identify this real-time log configuration.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The sampling rate for this real-time log configuration. The sampling rate determines the percentage of viewer requests that are represented in the real-time log data. An integer between `1` and `100`, inclusive.
        #[builder(into)]
        pub sampling_rate: pulumi_wasm_rust::Output<i32>,
    }
    #[allow(dead_code)]
    pub struct RealtimeLogConfigResult {
        /// The ARN (Amazon Resource Name) of the CloudFront real-time log configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Kinesis data streams where real-time log data is sent.
        pub endpoint: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::RealtimeLogConfigEndpoint,
        >,
        /// The fields that are included in each real-time log record. See the [AWS documentation](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-fields) for supported values.
        pub fields: pulumi_wasm_rust::Output<Vec<String>>,
        /// The unique name to identify this real-time log configuration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The sampling rate for this real-time log configuration. The sampling rate determines the percentage of viewer requests that are represented in the real-time log data. An integer between `1` and `100`, inclusive.
        pub sampling_rate: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RealtimeLogConfigArgs) -> RealtimeLogConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let endpoint_binding = args.endpoint.get_inner();
        let fields_binding = args.fields.get_inner();
        let name_binding = args.name.get_inner();
        let sampling_rate_binding = args.sampling_rate.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudfront/realtimeLogConfig:RealtimeLogConfig".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "endpoint".into(),
                    value: &endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "fields".into(),
                    value: &fields_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "samplingRate".into(),
                    value: &sampling_rate_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "fields".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "samplingRate".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RealtimeLogConfigResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            fields: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fields").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            sampling_rate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("samplingRate").unwrap(),
            ),
        }
    }
}
