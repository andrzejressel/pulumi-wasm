pub mod get_queues {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetQueuesArgs {
        /// A string to use for filtering the list results. Only those queues whose name begins with the specified string are returned. Queue URLs and names are case-sensitive.
        #[builder(into, default)]
        pub queue_name_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetQueuesResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub queue_name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of queue URLs.
        pub queue_urls: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetQueuesArgs,
    ) -> GetQueuesResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let queue_name_prefix_binding = args
            .queue_name_prefix
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:sqs/getQueues:getQueues".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "queueNamePrefix".into(),
                    value: &queue_name_prefix_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetQueuesResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            queue_name_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("queueNamePrefix"),
            ),
            queue_urls: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("queueUrls"),
            ),
        }
    }
}
