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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventStreamArgs,
    ) -> EventStreamResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_id_binding = args.application_id.get_output(context);
        let destination_stream_arn_binding = args
            .destination_stream_arn
            .get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:pinpoint/eventStream:EventStream".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationStreamArn".into(),
                    value: &destination_stream_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventStreamResult {
            application_id: o.get_field("applicationId"),
            destination_stream_arn: o.get_field("destinationStreamArn"),
            role_arn: o.get_field("roleArn"),
        }
    }
}
