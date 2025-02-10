/// Provides a VPC Endpoint connection notification resource.
/// Connection notifications notify subscribers of VPC Endpoint events.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   topicTopic:
///     type: aws:sns:Topic
///     name: topic
///     properties:
///       name: vpce-notification-topic
///       policy: ${topic.json}
///   foo:
///     type: aws:ec2:VpcEndpointService
///     properties:
///       acceptanceRequired: false
///       networkLoadBalancerArns:
///         - ${test.arn}
///   fooVpcEndpointConnectionNotification:
///     type: aws:ec2:VpcEndpointConnectionNotification
///     name: foo
///     properties:
///       vpcEndpointServiceId: ${foo.id}
///       connectionNotificationArn: ${topicTopic.arn}
///       connectionEvents:
///         - Accept
///         - Reject
/// variables:
///   topic:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - vpce.amazonaws.com
///             actions:
///               - SNS:Publish
///             resources:
///               - arn:aws:sns:*:*:vpce-notification-topic
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Endpoint connection notifications using the VPC endpoint connection notification `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcEndpointConnectionNotification:VpcEndpointConnectionNotification foo vpce-nfn-09e6ed3b4efba2263
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_endpoint_connection_notification {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointConnectionNotificationArgs {
        /// One or more endpoint [events](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateVpcEndpointConnectionNotification.html#API_CreateVpcEndpointConnectionNotification_RequestParameters) for which to receive notifications.
        ///
        /// > **NOTE:** One of `vpc_endpoint_service_id` or `vpc_endpoint_id` must be specified.
        #[builder(into)]
        pub connection_events: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The ARN of the SNS topic for the notifications.
        #[builder(into)]
        pub connection_notification_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the VPC Endpoint to receive notifications for.
        #[builder(into, default)]
        pub vpc_endpoint_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the VPC Endpoint Service to receive notifications for.
        #[builder(into, default)]
        pub vpc_endpoint_service_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointConnectionNotificationResult {
        /// One or more endpoint [events](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateVpcEndpointConnectionNotification.html#API_CreateVpcEndpointConnectionNotification_RequestParameters) for which to receive notifications.
        ///
        /// > **NOTE:** One of `vpc_endpoint_service_id` or `vpc_endpoint_id` must be specified.
        pub connection_events: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ARN of the SNS topic for the notifications.
        pub connection_notification_arn: pulumi_gestalt_rust::Output<String>,
        /// The type of notification.
        pub notification_type: pulumi_gestalt_rust::Output<String>,
        /// The state of the notification.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The ID of the VPC Endpoint to receive notifications for.
        pub vpc_endpoint_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the VPC Endpoint Service to receive notifications for.
        pub vpc_endpoint_service_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointConnectionNotificationArgs,
    ) -> VpcEndpointConnectionNotificationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let connection_events_binding = args.connection_events.get_output(context);
        let connection_notification_arn_binding = args
            .connection_notification_arn
            .get_output(context);
        let vpc_endpoint_id_binding = args.vpc_endpoint_id.get_output(context);
        let vpc_endpoint_service_id_binding = args
            .vpc_endpoint_service_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcEndpointConnectionNotification:VpcEndpointConnectionNotification"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionEvents".into(),
                    value: connection_events_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionNotificationArn".into(),
                    value: connection_notification_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcEndpointId".into(),
                    value: vpc_endpoint_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcEndpointServiceId".into(),
                    value: vpc_endpoint_service_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcEndpointConnectionNotificationResult {
            connection_events: o.get_field("connectionEvents"),
            connection_notification_arn: o.get_field("connectionNotificationArn"),
            notification_type: o.get_field("notificationType"),
            state: o.get_field("state"),
            vpc_endpoint_id: o.get_field("vpcEndpointId"),
            vpc_endpoint_service_id: o.get_field("vpcEndpointServiceId"),
        }
    }
}
