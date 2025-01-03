pub mod get_key_ring_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKeyRingIamPolicyArgs {
        /// The key ring ID, in the form
        /// `{project_id}/{location_name}/{key_ring_name}` or
        /// `{location_name}/{key_ring_name}`. In the second form, the provider's
        /// project setting will be used as a fallback.
        #[builder(into)]
        pub key_ring_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetKeyRingIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub key_ring_id: pulumi_wasm_rust::Output<String>,
        /// (Computed) The policy data
        pub policy_data: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetKeyRingIamPolicyArgs) -> GetKeyRingIamPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let key_ring_id_binding = args.key_ring_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:kms/getKeyRingIamPolicy:getKeyRingIamPolicy".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "keyRingId".into(),
                    value: &key_ring_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "keyRingId".into(),
                },
                register_interface::ResultField {
                    name: "policyData".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetKeyRingIamPolicyResult {
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            key_ring_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyRingId").unwrap(),
            ),
            policy_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyData").unwrap(),
            ),
        }
    }
}
