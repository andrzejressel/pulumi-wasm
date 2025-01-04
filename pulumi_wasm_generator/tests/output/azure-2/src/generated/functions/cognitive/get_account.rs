pub mod get_account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountArgs {
        /// Specifies the name of the Cognitive Services Account.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the resource group where the Cognitive Services Account resides.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetAccountResult {
        /// The endpoint of the Cognitive Services Account
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cognitive::GetAccountIdentity>,
        >,
        /// The kind of the Cognitive Services Account
        pub kind: pulumi_wasm_rust::Output<String>,
        /// Whether local authentication methods is enabled for the Cognitive Account.
        pub local_auth_enabled: pulumi_wasm_rust::Output<bool>,
        /// The Azure location where the Cognitive Services Account exists
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The primary access key of the Cognitive Services Account
        pub primary_access_key: pulumi_wasm_rust::Output<String>,
        /// If `kind` is `QnAMaker` the link to the QNA runtime.
        pub qna_runtime_endpoint: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The secondary access key of the Cognitive Services Account
        pub secondary_access_key: pulumi_wasm_rust::Output<String>,
        /// The SKU name of the Cognitive Services Account
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAccountArgs) -> GetAccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:cognitive/getAccount:getAccount".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "kind".into(),
                },
                register_interface::ResultField {
                    name: "localAuthEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "primaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "qnaRuntimeEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAccountResult {
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
            ),
            local_auth_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localAuthEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            primary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryAccessKey").unwrap(),
            ),
            qna_runtime_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("qnaRuntimeEndpoint").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryAccessKey").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
