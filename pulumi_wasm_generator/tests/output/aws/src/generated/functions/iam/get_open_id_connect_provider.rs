pub mod get_open_id_connect_provider {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOpenIdConnectProviderArgs {
        /// ARN of the OpenID Connect provider.
        #[builder(into, default)]
        pub arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of resource tags for the IAM OIDC provider.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// URL of the OpenID Connect provider.
        #[builder(into, default)]
        pub url: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetOpenIdConnectProviderResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// List of client IDs (also known as audiences). When a mobile or web app registers with an OpenID Connect provider, they establish a value that identifies the application. (This is the value that's sent as the client_id parameter on OAuth requests.)
        pub client_id_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Map of resource tags for the IAM OIDC provider.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// List of server certificate thumbprints for the OpenID Connect (OIDC) identity provider's server certificate(s).
        pub thumbprint_lists: pulumi_wasm_rust::Output<Vec<String>>,
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetOpenIdConnectProviderArgs) -> GetOpenIdConnectProviderResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let url_binding = args.url.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iam/getOpenIdConnectProvider:getOpenIdConnectProvider".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "url".into(),
                    value: &url_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "clientIdLists".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "thumbprintLists".into(),
                },
                register_interface::ResultField {
                    name: "url".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetOpenIdConnectProviderResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            client_id_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientIdLists").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            thumbprint_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thumbprintLists").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}