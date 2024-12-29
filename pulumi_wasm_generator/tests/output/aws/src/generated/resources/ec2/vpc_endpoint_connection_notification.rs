/// Provides a VPC Endpoint connection notification resource.
/// Connection notifications notify subscribers of VPC Endpoint events.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let topic = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder().actions(vec!["SNS:Publish",])
///                     .effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["vpce.amazonaws.com",]). type ("Service")
///                     .build_struct(),])
///                     .resources(vec!["arn:aws:sns:*:*:vpce-notification-topic",])
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let foo = vpc_endpoint_service::create(
///         "foo",
///         VpcEndpointServiceArgs::builder()
///             .acceptance_required(false)
///             .network_load_balancer_arns(vec!["${test.arn}",])
///             .build_struct(),
///     );
///     let fooVpcEndpointConnectionNotification = vpc_endpoint_connection_notification::create(
///         "fooVpcEndpointConnectionNotification",
///         VpcEndpointConnectionNotificationArgs::builder()
///             .connection_events(vec!["Accept", "Reject",])
///             .connection_notification_arn("${topicTopic.arn}")
///             .vpc_endpoint_service_id("${foo.id}")
///             .build_struct(),
///     );
///     let topicTopic = topic::create(
///         "topicTopic",
///         TopicArgs::builder()
///             .name("vpce-notification-topic")
///             .policy("${topic.json}")
///             .build_struct(),
///     );
/// }
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointConnectionNotificationArgs {
        /// One or more endpoint [events](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateVpcEndpointConnectionNotification.html#API_CreateVpcEndpointConnectionNotification_RequestParameters) for which to receive notifications.
        ///
        /// > **NOTE:** One of `vpc_endpoint_service_id` or `vpc_endpoint_id` must be specified.
        #[builder(into)]
        pub connection_events: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ARN of the SNS topic for the notifications.
        #[builder(into)]
        pub connection_notification_arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the VPC Endpoint to receive notifications for.
        #[builder(into, default)]
        pub vpc_endpoint_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the VPC Endpoint Service to receive notifications for.
        #[builder(into, default)]
        pub vpc_endpoint_service_id: pulumi_wasm_rust::Output<Option<String>>,
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
        name: &str,
        args: VpcEndpointConnectionNotificationArgs,
    ) -> VpcEndpointConnectionNotificationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let connection_events_binding = args.connection_events.get_inner();
        let connection_notification_arn_binding = args
            .connection_notification_arn
            .get_inner();
        let vpc_endpoint_id_binding = args.vpc_endpoint_id.get_inner();
        let vpc_endpoint_service_id_binding = args.vpc_endpoint_service_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcEndpointConnectionNotification:VpcEndpointConnectionNotification"
                .into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "connectionEvents".into(),
                },
                register_interface::ResultField {
                    name: "connectionNotificationArn".into(),
                },
                register_interface::ResultField {
                    name: "notificationType".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "vpcEndpointId".into(),
                },
                register_interface::ResultField {
                    name: "vpcEndpointServiceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcEndpointConnectionNotificationResult {
            connection_events: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionEvents").unwrap(),
            ),
            connection_notification_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionNotificationArn").unwrap(),
            ),
            notification_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationType").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            vpc_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcEndpointId").unwrap(),
            ),
            vpc_endpoint_service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcEndpointServiceId").unwrap(),
            ),
        }
    }
}
