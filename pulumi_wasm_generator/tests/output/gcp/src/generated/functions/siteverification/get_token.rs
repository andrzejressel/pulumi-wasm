pub mod get_token {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTokenArgs {
        /// The site identifier. If the type is set to SITE, the identifier is a URL. If the type is
        /// set to INET_DOMAIN, the identifier is a domain name.
        #[builder(into)]
        pub identifier: pulumi_wasm_rust::Output<String>,
        /// The type of resource to be verified, either a domain or a web site.
        /// Possible values are: `INET_DOMAIN`, `SITE`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The verification method for the Site Verification system to use to verify
        /// this site or domain.
        /// Possible values are: `ANALYTICS`, `DNS_CNAME`, `DNS_TXT`, `FILE`, `META`, `TAG_MANAGER`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub verification_method: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetTokenResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub identifier: pulumi_wasm_rust::Output<String>,
        /// The generated token for use in subsequent verification steps.
        pub token: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub verification_method: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetTokenArgs) -> GetTokenResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identifier_binding = args.identifier.get_inner();
        let type__binding = args.type_.get_inner();
        let verification_method_binding = args.verification_method.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:siteverification/getToken:getToken".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identifier".into(),
                    value: &identifier_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "verificationMethod".into(),
                    value: &verification_method_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identifier".into(),
                },
                register_interface::ResultField {
                    name: "token".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "verificationMethod".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTokenResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identifier").unwrap(),
            ),
            token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("token").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            verification_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("verificationMethod").unwrap(),
            ),
        }
    }
}
