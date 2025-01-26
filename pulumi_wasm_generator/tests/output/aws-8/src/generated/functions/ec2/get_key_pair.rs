pub mod get_key_pair {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKeyPairArgs {
        /// Custom filter block as described below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetKeyPairFilter>>,
        >,
        /// Whether to include the public key material in the response.
        #[builder(into, default)]
        pub include_public_key: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Key Pair name.
        #[builder(into, default)]
        pub key_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key Pair ID.
        #[builder(into, default)]
        pub key_pair_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Any tags assigned to the Key Pair.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetKeyPairResult {
        /// ARN of the Key Pair.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Timestamp for when the key pair was created in ISO 8601 format.
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetKeyPairFilter>>,
        >,
        /// SHA-1 digest of the DER encoded private key.
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub include_public_key: pulumi_wasm_rust::Output<Option<bool>>,
        pub key_name: pulumi_wasm_rust::Output<Option<String>>,
        pub key_pair_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Type of key pair.
        pub key_type: pulumi_wasm_rust::Output<String>,
        /// Public key material.
        pub public_key: pulumi_wasm_rust::Output<String>,
        /// Any tags assigned to the Key Pair.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetKeyPairArgs,
    ) -> GetKeyPairResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let include_public_key_binding = args
            .include_public_key
            .get_output(context)
            .get_inner();
        let key_name_binding = args.key_name.get_output(context).get_inner();
        let key_pair_id_binding = args.key_pair_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getKeyPair:getKeyPair".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "includePublicKey".into(),
                    value: &include_public_key_binding,
                },
                register_interface::ObjectField {
                    name: "keyName".into(),
                    value: &key_name_binding,
                },
                register_interface::ObjectField {
                    name: "keyPairId".into(),
                    value: &key_pair_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "fingerprint".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "includePublicKey".into(),
                },
                register_interface::ResultField {
                    name: "keyName".into(),
                },
                register_interface::ResultField {
                    name: "keyPairId".into(),
                },
                register_interface::ResultField {
                    name: "keyType".into(),
                },
                register_interface::ResultField {
                    name: "publicKey".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetKeyPairResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fingerprint").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            include_public_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includePublicKey").unwrap(),
            ),
            key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyName").unwrap(),
            ),
            key_pair_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyPairId").unwrap(),
            ),
            key_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyType").unwrap(),
            ),
            public_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicKey").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
