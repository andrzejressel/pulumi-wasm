pub mod get_account_jwt {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountJwtArgs {
        /// Delegate chain of approvals needed to perform full impersonation. Specify the fully qualified service account name.
        #[builder(into, default)]
        pub delegates: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Number of seconds until the JWT expires. If set and non-zero an `exp` claim will be added to the payload derived from the current timestamp plus expires_in seconds.
        #[builder(into, default)]
        pub expires_in: pulumi_wasm_rust::Output<Option<i32>>,
        /// The JSON-encoded JWT claims set to include in the self-signed JWT.
        #[builder(into)]
        pub payload: pulumi_wasm_rust::Output<String>,
        /// The email of the service account that will sign the JWT.
        #[builder(into)]
        pub target_service_account: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccountJwtResult {
        pub delegates: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub expires_in: pulumi_wasm_rust::Output<Option<i32>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The signed JWT containing the JWT Claims Set from the `payload`.
        pub jwt: pulumi_wasm_rust::Output<String>,
        pub payload: pulumi_wasm_rust::Output<String>,
        pub target_service_account: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAccountJwtArgs) -> GetAccountJwtResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let delegates_binding = args.delegates.get_inner();
        let expires_in_binding = args.expires_in.get_inner();
        let payload_binding = args.payload.get_inner();
        let target_service_account_binding = args.target_service_account.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:serviceaccount/getAccountJwt:getAccountJwt".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "delegates".into(),
                    value: &delegates_binding,
                },
                register_interface::ObjectField {
                    name: "expiresIn".into(),
                    value: &expires_in_binding,
                },
                register_interface::ObjectField {
                    name: "payload".into(),
                    value: &payload_binding,
                },
                register_interface::ObjectField {
                    name: "targetServiceAccount".into(),
                    value: &target_service_account_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "delegates".into(),
                },
                register_interface::ResultField {
                    name: "expiresIn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "jwt".into(),
                },
                register_interface::ResultField {
                    name: "payload".into(),
                },
                register_interface::ResultField {
                    name: "targetServiceAccount".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAccountJwtResult {
            delegates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("delegates").unwrap(),
            ),
            expires_in: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiresIn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            jwt: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jwt").unwrap(),
            ),
            payload: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("payload").unwrap(),
            ),
            target_service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetServiceAccount").unwrap(),
            ),
        }
    }
}
