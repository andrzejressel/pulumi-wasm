pub mod get_queues {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetQueuesArgs {
        /// A string to use for filtering the list results. Only those queues whose name begins with the specified string are returned. Queue URLs and names are case-sensitive.
        #[builder(into, default)]
        pub queue_name_prefix: pulumi_wasm_rust::Output<Option<String>>,
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
    pub fn invoke(args: GetQueuesArgs) -> GetQueuesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let queue_name_prefix_binding = args.queue_name_prefix.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:sqs/getQueues:getQueues".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "queueNamePrefix".into(),
                    value: &queue_name_prefix_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "queueNamePrefix".into(),
                },
                register_interface::ResultField {
                    name: "queueUrls".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetQueuesResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            queue_name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queueNamePrefix").unwrap(),
            ),
            queue_urls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queueUrls").unwrap(),
            ),
        }
    }
}
