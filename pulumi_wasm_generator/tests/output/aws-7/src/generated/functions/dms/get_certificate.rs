pub mod get_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateArgs {
        /// A customer-assigned name for the certificate. Identifiers must begin with a letter and must contain only ASCII letters, digits, and hyphens. They can't end with a hyphen or contain two consecutive hyphens.
        #[builder(into)]
        pub certificate_id: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetCertificateResult {
        /// The Amazon Resource Name (ARN) for the certificate.
        pub certificate_arn: pulumi_wasm_rust::Output<String>,
        /// The date that the certificate was created.
        pub certificate_creation_date: pulumi_wasm_rust::Output<String>,
        pub certificate_id: pulumi_wasm_rust::Output<String>,
        /// The owner of the certificate.
        pub certificate_owner: pulumi_wasm_rust::Output<String>,
        /// The contents of a .pem file, which contains an X.509 certificate.
        pub certificate_pem: pulumi_wasm_rust::Output<String>,
        /// The owner of the certificate.
        pub certificate_wallet: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The key length of the cryptographic algorithm being used.
        pub key_length: pulumi_wasm_rust::Output<i32>,
        /// The algorithm for the certificate.
        pub signing_algorithm: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The beginning date that the certificate is valid.
        pub valid_from_date: pulumi_wasm_rust::Output<String>,
        /// The final date that the certificate is valid.
        pub valid_to_date: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetCertificateArgs) -> GetCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_id_binding = args.certificate_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:dms/getCertificate:getCertificate".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateId".into(),
                    value: &certificate_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificateArn".into(),
                },
                register_interface::ResultField {
                    name: "certificateCreationDate".into(),
                },
                register_interface::ResultField {
                    name: "certificateId".into(),
                },
                register_interface::ResultField {
                    name: "certificateOwner".into(),
                },
                register_interface::ResultField {
                    name: "certificatePem".into(),
                },
                register_interface::ResultField {
                    name: "certificateWallet".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "keyLength".into(),
                },
                register_interface::ResultField {
                    name: "signingAlgorithm".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "validFromDate".into(),
                },
                register_interface::ResultField {
                    name: "validToDate".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCertificateResult {
            certificate_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateArn").unwrap(),
            ),
            certificate_creation_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateCreationDate").unwrap(),
            ),
            certificate_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateId").unwrap(),
            ),
            certificate_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateOwner").unwrap(),
            ),
            certificate_pem: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificatePem").unwrap(),
            ),
            certificate_wallet: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateWallet").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            key_length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyLength").unwrap(),
            ),
            signing_algorithm: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingAlgorithm").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            valid_from_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validFromDate").unwrap(),
            ),
            valid_to_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validToDate").unwrap(),
            ),
        }
    }
}
