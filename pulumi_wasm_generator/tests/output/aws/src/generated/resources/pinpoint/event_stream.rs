/// Provides a Pinpoint Event Stream resource.
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
///                     .identifiers(vec!["pinpoint.us-east-1.amazonaws.com",]). type
///                     ("Service").build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let testRolePolicy = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["kinesis:PutRecords", "kinesis:DescribeStream",])
///                     .effect("Allow").resources(vec!["arn:aws:kinesis:us-east-1:*:*/*",])
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let app = app::create("app", AppArgs::builder().build_struct());
///     let stream = event_stream::create(
///         "stream",
///         EventStreamArgs::builder()
///             .application_id("${app.applicationId}")
///             .destination_stream_arn("${testStream.arn}")
///             .role_arn("${testRole.arn}")
///             .build_struct(),
///     );
///     let testRole = role::create(
///         "testRole",
///         RoleArgs::builder().assume_role_policy("${assumeRole.json}").build_struct(),
///     );
///     let testRolePolicyRolePolicy = role_policy::create(
///         "testRolePolicyRolePolicy",
///         RolePolicyArgs::builder()
///             .name("test_policy")
///             .policy("${testRolePolicy.json}")
///             .role("${testRole.id}")
///             .build_struct(),
///     );
///     let testStream = stream::create(
///         "testStream",
///         StreamArgs::builder().name("pinpoint-kinesis-test").shard_count(1).build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Pinpoint Event Stream using the `application-id`. For example:
///
/// ```sh
/// $ pulumi import aws:pinpoint/eventStream:EventStream stream application-id
/// ```
pub mod event_stream {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventStreamArgs {
        /// The application ID.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Amazon Kinesis stream or Firehose delivery stream to which you want to publish events.
        #[builder(into)]
        pub destination_stream_arn: pulumi_wasm_rust::Output<String>,
        /// The IAM role that authorizes Amazon Pinpoint to publish events to the stream in your account.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct EventStreamResult {
        /// The application ID.
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Amazon Kinesis stream or Firehose delivery stream to which you want to publish events.
        pub destination_stream_arn: pulumi_wasm_rust::Output<String>,
        /// The IAM role that authorizes Amazon Pinpoint to publish events to the stream in your account.
        pub role_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EventStreamArgs) -> EventStreamResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_inner();
        let destination_stream_arn_binding = args.destination_stream_arn.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:pinpoint/eventStream:EventStream".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "destinationStreamArn".into(),
                    value: &destination_stream_arn_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationId".into(),
                },
                register_interface::ResultField {
                    name: "destinationStreamArn".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EventStreamResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationId").unwrap(),
            ),
            destination_stream_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationStreamArn").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
        }
    }
}
