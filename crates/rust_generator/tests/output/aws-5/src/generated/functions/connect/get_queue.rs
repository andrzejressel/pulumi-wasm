#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_queue {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetQueueArgs {
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Returns information on a specific Queue by name
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Returns information on a specific Queue by Queue id
        #[builder(into, default)]
        pub queue_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags assigned to the Queue.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetQueueResult {
        /// ARN of the Queue.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the Queue.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Specifies the identifier of the Hours of Operation.
        pub hours_of_operation_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// Maximum number of contacts that can be in the queue before it is considered full. Minimum value of 0.
        pub max_contacts: pulumi_gestalt_rust::Output<i32>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A block that defines the outbound caller ID name, number, and outbound whisper flow. The Outbound Caller Config block is documented below.
        pub outbound_caller_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::connect::GetQueueOutboundCallerConfig>,
        >,
        /// Identifier for the Queue.
        pub queue_id: pulumi_gestalt_rust::Output<String>,
        /// Description of the Queue. Values are `ENABLED` or `DISABLED`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Map of tags assigned to the Queue.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetQueueArgs,
    ) -> GetQueueResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_id_binding = args.instance_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let queue_id_binding = args.queue_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:connect/getQueue:getQueue".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queueId".into(),
                    value: queue_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetQueueResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            hours_of_operation_id: o.get_field("hoursOfOperationId"),
            id: o.get_field("id"),
            instance_id: o.get_field("instanceId"),
            max_contacts: o.get_field("maxContacts"),
            name: o.get_field("name"),
            outbound_caller_configs: o.get_field("outboundCallerConfigs"),
            queue_id: o.get_field("queueId"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
        }
    }
}
