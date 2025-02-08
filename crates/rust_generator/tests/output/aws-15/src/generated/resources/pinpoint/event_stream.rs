/// Provides a Pinpoint Event Stream resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   stream:
///     type: aws:pinpoint:EventStream
///     properties:
///       applicationId: ${app.applicationId}
///       destinationStreamArn: ${testStream.arn}
///       roleArn: ${testRole.arn}
///   app:
///     type: aws:pinpoint:App
///   testStream:
///     type: aws:kinesis:Stream
///     name: test_stream
///     properties:
///       name: pinpoint-kinesis-test
///       shardCount: 1
///   testRole:
///     type: aws:iam:Role
///     name: test_role
///     properties:
///       assumeRolePolicy: ${assumeRole.json}
///   testRolePolicyRolePolicy:
///     type: aws:iam:RolePolicy
///     name: test_role_policy
///     properties:
///       name: test_policy
///       role: ${testRole.id}
///       policy: ${testRolePolicy.json}
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - pinpoint.us-east-1.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   testRolePolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - kinesis:PutRecords
///               - kinesis:DescribeStream
///             resources:
///               - arn:aws:kinesis:us-east-1:*:*/*
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Pinpoint Event Stream using the `application-id`. For example:
///
/// ```sh
/// $ pulumi import aws:pinpoint/eventStream:EventStream stream application-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_stream {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventStreamArgs {
        /// The application ID.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the Amazon Kinesis stream or Firehose delivery stream to which you want to publish events.
        #[builder(into)]
        pub destination_stream_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The IAM role that authorizes Amazon Pinpoint to publish events to the stream in your account.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EventStreamResult {
        /// The application ID.
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Amazon Kinesis stream or Firehose delivery stream to which you want to publish events.
        pub destination_stream_arn: pulumi_gestalt_rust::Output<String>,
        /// The IAM role that authorizes Amazon Pinpoint to publish events to the stream in your account.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EventStreamArgs,
    ) -> EventStreamResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_output(context).get_inner();
        let destination_stream_arn_binding = args
            .destination_stream_arn
            .get_output(context)
            .get_inner();
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:pinpoint/eventStream:EventStream".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        EventStreamResult {
            application_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationId"),
            ),
            destination_stream_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationStreamArn"),
            ),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
        }
    }
}
