pub mod get_private_link_resource {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPrivateLinkResourceArgs {
        /// The ID of an existing Web Pubsub Resource which Private Link Resource should be retrieved for.
        #[builder(into)]
        pub web_pubsub_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetPrivateLinkResourceResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A `shared_private_link_resource_types` block as defined below.
        pub shared_private_link_resource_types: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::webpubsub::GetPrivateLinkResourceSharedPrivateLinkResourceType,
            >,
        >,
        pub web_pubsub_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPrivateLinkResourceArgs) -> GetPrivateLinkResourceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let web_pubsub_id_binding = args.web_pubsub_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:webpubsub/getPrivateLinkResource:getPrivateLinkResource"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "webPubsubId".into(),
                    value: &web_pubsub_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "sharedPrivateLinkResourceTypes".into(),
                },
                register_interface::ResultField {
                    name: "webPubsubId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPrivateLinkResourceResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            shared_private_link_resource_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedPrivateLinkResourceTypes").unwrap(),
            ),
            web_pubsub_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webPubsubId").unwrap(),
            ),
        }
    }
}