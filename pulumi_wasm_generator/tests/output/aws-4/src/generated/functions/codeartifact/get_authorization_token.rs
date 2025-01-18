pub mod get_authorization_token {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAuthorizationTokenArgs {
        /// Name of the domain that is in scope for the generated authorization token.
        #[builder(into)]
        pub domain: pulumi_wasm_rust::Output<String>,
        /// Account number of the AWS account that owns the domain.
        #[builder(into, default)]
        pub domain_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// Time, in seconds, that the generated authorization token is valid. Valid values are `0` and between `900` and `43200`.
        #[builder(into, default)]
        pub duration_seconds: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct GetAuthorizationTokenResult {
        /// Temporary authorization token.
        pub authorization_token: pulumi_wasm_rust::Output<String>,
        pub domain: pulumi_wasm_rust::Output<String>,
        pub domain_owner: pulumi_wasm_rust::Output<String>,
        pub duration_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Time in UTC RFC3339 format when the authorization token expires.
        pub expiration: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAuthorizationTokenArgs) -> GetAuthorizationTokenResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_binding = args.domain.get_inner();
        let domain_owner_binding = args.domain_owner.get_inner();
        let duration_seconds_binding = args.duration_seconds.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:codeartifact/getAuthorizationToken:getAuthorizationToken".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
                register_interface::ObjectField {
                    name: "domainOwner".into(),
                    value: &domain_owner_binding,
                },
                register_interface::ObjectField {
                    name: "durationSeconds".into(),
                    value: &duration_seconds_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authorizationToken".into(),
                },
                register_interface::ResultField {
                    name: "domain".into(),
                },
                register_interface::ResultField {
                    name: "domainOwner".into(),
                },
                register_interface::ResultField {
                    name: "durationSeconds".into(),
                },
                register_interface::ResultField {
                    name: "expiration".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAuthorizationTokenResult {
            authorization_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizationToken").unwrap(),
            ),
            domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domain").unwrap(),
            ),
            domain_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainOwner").unwrap(),
            ),
            duration_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("durationSeconds").unwrap(),
            ),
            expiration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiration").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
        }
    }
}
