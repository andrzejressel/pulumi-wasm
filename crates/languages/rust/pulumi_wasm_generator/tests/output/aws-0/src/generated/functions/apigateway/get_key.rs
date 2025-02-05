pub mod get_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKeyArgs {
        /// ID of the API Key to look up.
        #[builder(into)]
        pub id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Map of tags for the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetKeyResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Date and time when the API Key was created.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// Amazon Web Services Marketplace customer identifier, when integrating with the Amazon Web Services SaaS Marketplace.
        pub customer_id: pulumi_wasm_rust::Output<String>,
        /// Description of the API Key.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Whether the API Key is enabled.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// Set to the ID of the API Key.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Date and time when the API Key was last updated.
        pub last_updated_date: pulumi_wasm_rust::Output<String>,
        /// Set to the name of the API Key.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Map of tags for the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Set to the value of the API Key.
        pub value: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetKeyArgs,
    ) -> GetKeyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:apigateway/getKey:getKey".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetKeyResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            created_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            customer_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customerId"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            last_updated_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastUpdatedDate"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            value: pulumi_wasm_rust::__private::into_domain(o.extract_field("value")),
        }
    }
}
