/// Provides an Amazon Connect Queue resource. For more information see
/// [Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   test:
///     type: aws:connect:Queue
///     properties:
///       instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
///       name: Example Name
///       description: Example Description
///       hoursOfOperationId: 12345678-1234-1234-1234-123456789012
///       tags:
///         Name: Example Queue
/// ```
///
/// ### With Quick Connect IDs
///
/// ```yaml
/// resources:
///   test:
///     type: aws:connect:Queue
///     properties:
///       instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
///       name: Example Name
///       description: Example Description
///       hoursOfOperationId: 12345678-1234-1234-1234-123456789012
///       quickConnectIds:
///         - 12345678-abcd-1234-abcd-123456789012
///       tags:
///         Name: Example Queue with Quick Connect IDs
/// ```
///
/// ### With Outbound Caller Config
///
/// ```yaml
/// resources:
///   test:
///     type: aws:connect:Queue
///     properties:
///       instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
///       name: Example Name
///       description: Example Description
///       hoursOfOperationId: 12345678-1234-1234-1234-123456789012
///       outboundCallerConfig:
///         outboundCallerIdName: example
///         outboundCallerIdNumberId: 12345678-abcd-1234-abcd-123456789012
///         outboundFlowId: 87654321-defg-1234-defg-987654321234
///       tags:
///         Name: Example Queue with Outbound Caller Config
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Connect Queues using the `instance_id` and `queue_id` separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:connect/queue:Queue example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod queue {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QueueArgs {
        /// Specifies the description of the Queue.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the identifier of the Hours of Operation.
        #[builder(into)]
        pub hours_of_operation_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the maximum number of contacts that can be in the queue before it is considered full. Minimum value of 0.
        #[builder(into, default)]
        pub max_contacts: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the name of the Queue.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A block that defines the outbound caller ID name, number, and outbound whisper flow. The Outbound Caller Config block is documented below.
        #[builder(into, default)]
        pub outbound_caller_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::connect::QueueOutboundCallerConfig>,
        >,
        /// Specifies a list of quick connects ids that determine the quick connects available to agents who are working the queue.
        #[builder(into, default)]
        pub quick_connect_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the description of the Queue. Valid values are `ENABLED`, `DISABLED`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Tags to apply to the Queue. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct QueueResult {
        /// The Amazon Resource Name (ARN) of the Queue.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the description of the Queue.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the identifier of the Hours of Operation.
        pub hours_of_operation_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the maximum number of contacts that can be in the queue before it is considered full. Minimum value of 0.
        pub max_contacts: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the name of the Queue.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A block that defines the outbound caller ID name, number, and outbound whisper flow. The Outbound Caller Config block is documented below.
        pub outbound_caller_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::connect::QueueOutboundCallerConfig>,
        >,
        /// The identifier for the Queue.
        pub queue_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of quick connects ids that determine the quick connects available to agents who are working the queue.
        pub quick_connect_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the description of the Queue. Valid values are `ENABLED`, `DISABLED`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Tags to apply to the Queue. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: QueueArgs,
    ) -> QueueResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let hours_of_operation_id_binding = args
            .hours_of_operation_id
            .get_output(context);
        let instance_id_binding = args.instance_id.get_output(context);
        let max_contacts_binding = args.max_contacts.get_output(context);
        let name_binding = args.name.get_output(context);
        let outbound_caller_config_binding = args
            .outbound_caller_config
            .get_output(context);
        let quick_connect_ids_binding = args.quick_connect_ids.get_output(context);
        let status_binding = args.status.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:connect/queue:Queue".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hoursOfOperationId".into(),
                    value: hours_of_operation_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxContacts".into(),
                    value: max_contacts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outboundCallerConfig".into(),
                    value: outbound_caller_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "quickConnectIds".into(),
                    value: quick_connect_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: status_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        QueueResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            hours_of_operation_id: o.get_field("hoursOfOperationId"),
            instance_id: o.get_field("instanceId"),
            max_contacts: o.get_field("maxContacts"),
            name: o.get_field("name"),
            outbound_caller_config: o.get_field("outboundCallerConfig"),
            queue_id: o.get_field("queueId"),
            quick_connect_ids: o.get_field("quickConnectIds"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
