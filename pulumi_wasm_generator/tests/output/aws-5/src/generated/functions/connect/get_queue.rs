pub mod get_queue {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetQueueArgs {
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Returns information on a specific Queue by name
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Returns information on a specific Queue by Queue id
        #[builder(into, default)]
        pub queue_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags assigned to the Queue.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetQueueResult {
        /// ARN of the Queue.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of the Queue.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Specifies the identifier of the Hours of Operation.
        pub hours_of_operation_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// Maximum number of contacts that can be in the queue before it is considered full. Minimum value of 0.
        pub max_contacts: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A block that defines the outbound caller ID name, number, and outbound whisper flow. The Outbound Caller Config block is documented below.
        pub outbound_caller_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::connect::GetQueueOutboundCallerConfig>,
        >,
        /// Identifier for the Queue.
        pub queue_id: pulumi_wasm_rust::Output<String>,
        /// Description of the Queue. Values are `ENABLED` or `DISABLED`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Map of tags assigned to the Queue.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetQueueArgs,
    ) -> GetQueueResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let queue_id_binding = args.queue_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:connect/getQueue:getQueue".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "queueId".into(),
                    value: &queue_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetQueueResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            hours_of_operation_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hoursOfOperationId"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            max_contacts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maxContacts"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            outbound_caller_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outboundCallerConfigs"),
            ),
            queue_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("queueId"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
