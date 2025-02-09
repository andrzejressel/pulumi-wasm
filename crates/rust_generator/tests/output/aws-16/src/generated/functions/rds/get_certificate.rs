#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateArgs {
        /// Certificate identifier. For example, `rds-ca-2019`.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// When enabled, returns the certificate with the latest `ValidTill`.
        #[builder(into, default)]
        pub latest_valid_till: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetCertificateResult {
        /// ARN of the certificate.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Type of certificate. For example, `CA`.
        pub certificate_type: pulumi_gestalt_rust::Output<String>,
        /// Boolean whether there is an override for the default certificate identifier.
        pub customer_override: pulumi_gestalt_rust::Output<bool>,
        /// If there is an override for the default certificate identifier, when the override expires.
        pub customer_override_valid_till: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        pub latest_valid_till: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Thumbprint of the certificate.
        pub thumbprint: pulumi_gestalt_rust::Output<String>,
        /// [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) of certificate starting validity date.
        pub valid_from: pulumi_gestalt_rust::Output<String>,
        /// [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) of certificate ending validity date.
        pub valid_till: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCertificateArgs,
    ) -> GetCertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let id_binding = args.id.get_output(context);
        let latest_valid_till_binding = args.latest_valid_till.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:rds/getCertificate:getCertificate".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "latestValidTill".into(),
                    value: latest_valid_till_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCertificateResult {
            arn: o.get_field("arn"),
            certificate_type: o.get_field("certificateType"),
            customer_override: o.get_field("customerOverride"),
            customer_override_valid_till: o.get_field("customerOverrideValidTill"),
            id: o.get_field("id"),
            latest_valid_till: o.get_field("latestValidTill"),
            thumbprint: o.get_field("thumbprint"),
            valid_from: o.get_field("validFrom"),
            valid_till: o.get_field("validTill"),
        }
    }
}
