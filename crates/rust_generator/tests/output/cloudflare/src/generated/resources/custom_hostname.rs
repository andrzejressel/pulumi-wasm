/// Provides a Cloudflare custom hostname (also known as SSL for SaaS) resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = custom_hostname::create(
///         "example",
///         CustomHostnameArgs::builder()
///             .hostname("hostname.example.com")
///             .ssls(vec![CustomHostnameSsl::builder().method("txt").build_struct(),])
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/customHostname:CustomHostname example 1d5fdc9e88c8a8c4518b068cd94331fe/0d89c70d-ad9f-4843-b99f-6cc0252067e9
/// ```
///
pub mod custom_hostname {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomHostnameArgs {
        /// Custom metadata associated with custom hostname. Only supports primitive string values, all other values are accessible via the API directly.
        #[builder(into, default)]
        pub custom_metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The custom origin server used for certificates.
        #[builder(into, default)]
        pub custom_origin_server: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The [custom origin SNI](https://developers.cloudflare.com/ssl/ssl-for-saas/hostname-specific-behavior/custom-origin) used for certificates.
        #[builder(into, default)]
        pub custom_origin_sni: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Hostname you intend to request a certificate for. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub hostname: pulumi_gestalt_rust::InputOrOutput<String>,
        /// SSL properties used when creating the custom hostname.
        #[builder(into, default)]
        pub ssls: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::CustomHostnameSsl>>,
        >,
        /// Whether to wait for a custom hostname SSL sub-object to reach status `pending_validation` during creation. Defaults to `false`.
        #[builder(into, default)]
        pub wait_for_ssl_pending_validation: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CustomHostnameResult {
        /// Custom metadata associated with custom hostname. Only supports primitive string values, all other values are accessible via the API directly.
        pub custom_metadata: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The custom origin server used for certificates.
        pub custom_origin_server: pulumi_gestalt_rust::Output<Option<String>>,
        /// The [custom origin SNI](https://developers.cloudflare.com/ssl/ssl-for-saas/hostname-specific-behavior/custom-origin) used for certificates.
        pub custom_origin_sni: pulumi_gestalt_rust::Output<Option<String>>,
        /// Hostname you intend to request a certificate for. **Modifying this attribute will force creation of a new resource.**
        pub hostname: pulumi_gestalt_rust::Output<String>,
        pub ownership_verification: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub ownership_verification_http: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// SSL properties used when creating the custom hostname.
        pub ssls: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::CustomHostnameSsl>>,
        >,
        /// Status of the certificate.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Whether to wait for a custom hostname SSL sub-object to reach status `pending_validation` during creation. Defaults to `false`.
        pub wait_for_ssl_pending_validation: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CustomHostnameArgs,
    ) -> CustomHostnameResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let custom_metadata_binding = args
            .custom_metadata
            .get_output(context)
            .get_inner();
        let custom_origin_server_binding = args
            .custom_origin_server
            .get_output(context)
            .get_inner();
        let custom_origin_sni_binding = args
            .custom_origin_sni
            .get_output(context)
            .get_inner();
        let hostname_binding = args.hostname.get_output(context).get_inner();
        let ssls_binding = args.ssls.get_output(context).get_inner();
        let wait_for_ssl_pending_validation_binding = args
            .wait_for_ssl_pending_validation
            .get_output(context)
            .get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/customHostname:CustomHostname".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customMetadata".into(),
                    value: &custom_metadata_binding,
                },
                register_interface::ObjectField {
                    name: "customOriginServer".into(),
                    value: &custom_origin_server_binding,
                },
                register_interface::ObjectField {
                    name: "customOriginSni".into(),
                    value: &custom_origin_sni_binding,
                },
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "ssls".into(),
                    value: &ssls_binding,
                },
                register_interface::ObjectField {
                    name: "waitForSslPendingValidation".into(),
                    value: &wait_for_ssl_pending_validation_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CustomHostnameResult {
            custom_metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customMetadata"),
            ),
            custom_origin_server: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customOriginServer"),
            ),
            custom_origin_sni: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customOriginSni"),
            ),
            hostname: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostname"),
            ),
            ownership_verification: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownershipVerification"),
            ),
            ownership_verification_http: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownershipVerificationHttp"),
            ),
            ssls: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ssls")),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            wait_for_ssl_pending_validation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("waitForSslPendingValidation"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
