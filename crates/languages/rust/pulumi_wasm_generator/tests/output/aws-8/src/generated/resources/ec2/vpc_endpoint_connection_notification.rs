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
pub mod vpc_endpoint_connection_notification {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointConnectionNotificationArgs {
        /// One or more endpoint [events](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateVpcEndpointConnectionNotification.html#API_CreateVpcEndpointConnectionNotification_RequestParameters) for which to receive notifications.
        ///
        /// > **NOTE:** One of `vpc_endpoint_service_id` or `vpc_endpoint_id` must be specified.
        #[builder(into)]
        pub connection_events: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The ARN of the SNS topic for the notifications.
        #[builder(into)]
        pub connection_notification_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the VPC Endpoint to receive notifications for.
        #[builder(into, default)]
        pub vpc_endpoint_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the VPC Endpoint Service to receive notifications for.
        #[builder(into, default)]
        pub vpc_endpoint_service_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointConnectionNotificationResult {
        /// One or more endpoint [events](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateVpcEndpointConnectionNotification.html#API_CreateVpcEndpointConnectionNotification_RequestParameters) for which to receive notifications.
        ///
        /// > **NOTE:** One of `vpc_endpoint_service_id` or `vpc_endpoint_id` must be specified.
        pub connection_events: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ARN of the SNS topic for the notifications.
        pub connection_notification_arn: pulumi_wasm_rust::Output<String>,
        /// The type of notification.
        pub notification_type: pulumi_wasm_rust::Output<String>,
        /// The state of the notification.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The ID of the VPC Endpoint to receive notifications for.
        pub vpc_endpoint_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the VPC Endpoint Service to receive notifications for.
        pub vpc_endpoint_service_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VpcEndpointConnectionNotificationArgs,
    ) -> VpcEndpointConnectionNotificationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let connection_events_binding = args
            .connection_events
            .get_output(context)
            .get_inner();
        let connection_notification_arn_binding = args
            .connection_notification_arn
            .get_output(context)
            .get_inner();
        let vpc_endpoint_id_binding = args
            .vpc_endpoint_id
            .get_output(context)
            .get_inner();
        let vpc_endpoint_service_id_binding = args
            .vpc_endpoint_service_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcEndpointConnectionNotification:VpcEndpointConnectionNotification"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "connectionEvents".into(),
                    value: &connection_events_binding,
                },
                register_interface::ObjectField {
                    name: "connectionNotificationArn".into(),
                    value: &connection_notification_arn_binding,
                },
                register_interface::ObjectField {
                    name: "vpcEndpointId".into(),
                    value: &vpc_endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpcEndpointServiceId".into(),
                    value: &vpc_endpoint_service_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpcEndpointConnectionNotificationResult {
            connection_events: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectionEvents"),
            ),
            connection_notification_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectionNotificationArn"),
            ),
            notification_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("notificationType"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            vpc_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpcEndpointId"),
            ),
            vpc_endpoint_service_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpcEndpointServiceId"),
            ),
        }
    }
}
