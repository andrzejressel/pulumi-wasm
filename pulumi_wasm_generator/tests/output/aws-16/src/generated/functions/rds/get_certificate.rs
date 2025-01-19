pub mod get_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateArgs {
        /// Certificate identifier. For example, `rds-ca-2019`.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        /// When enabled, returns the certificate with the latest `ValidTill`.
        #[builder(into, default)]
        pub latest_valid_till: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetCertificateResult {
        /// ARN of the certificate.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Type of certificate. For example, `CA`.
        pub certificate_type: pulumi_wasm_rust::Output<String>,
        /// Boolean whether there is an override for the default certificate identifier.
        pub customer_override: pulumi_wasm_rust::Output<bool>,
        /// If there is an override for the default certificate identifier, when the override expires.
        pub customer_override_valid_till: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        pub latest_valid_till: pulumi_wasm_rust::Output<Option<bool>>,
        /// Thumbprint of the certificate.
        pub thumbprint: pulumi_wasm_rust::Output<String>,
        /// [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) of certificate starting validity date.
        pub valid_from: pulumi_wasm_rust::Output<String>,
        /// [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) of certificate ending validity date.
        pub valid_till: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetCertificateArgs) -> GetCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_inner();
        let latest_valid_till_binding = args.latest_valid_till.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:rds/getCertificate:getCertificate".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "latestValidTill".into(),
                    value: &latest_valid_till_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "certificateType".into(),
                },
                register_interface::ResultField {
                    name: "customerOverride".into(),
                },
                register_interface::ResultField {
                    name: "customerOverrideValidTill".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "latestValidTill".into(),
                },
                register_interface::ResultField {
                    name: "thumbprint".into(),
                },
                register_interface::ResultField {
                    name: "validFrom".into(),
                },
                register_interface::ResultField {
                    name: "validTill".into(),
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
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            certificate_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateType").unwrap(),
            ),
            customer_override: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerOverride").unwrap(),
            ),
            customer_override_valid_till: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerOverrideValidTill").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            latest_valid_till: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latestValidTill").unwrap(),
            ),
            thumbprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thumbprint").unwrap(),
            ),
            valid_from: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validFrom").unwrap(),
            ),
            valid_till: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validTill").unwrap(),
            ),
        }
    }
}
