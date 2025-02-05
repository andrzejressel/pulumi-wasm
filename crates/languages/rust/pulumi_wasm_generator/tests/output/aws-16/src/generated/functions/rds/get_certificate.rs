pub mod get_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateArgs {
        /// Certificate identifier. For example, `rds-ca-2019`.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// When enabled, returns the certificate with the latest `ValidTill`.
        #[builder(into, default)]
        pub latest_valid_till: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetCertificateArgs,
    ) -> GetCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let latest_valid_till_binding = args
            .latest_valid_till
            .get_output(context)
            .get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCertificateResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            certificate_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificateType"),
            ),
            customer_override: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customerOverride"),
            ),
            customer_override_valid_till: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customerOverrideValidTill"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            latest_valid_till: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("latestValidTill"),
            ),
            thumbprint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("thumbprint"),
            ),
            valid_from: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("validFrom"),
            ),
            valid_till: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("validTill"),
            ),
        }
    }
}
