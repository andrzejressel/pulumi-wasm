/// Provides a Cloudflare custom SSL resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: cloudflare:CustomSsl
///     properties:
///       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
///       customSslOptions:
///         certificate: '-----INSERT CERTIFICATE-----'
///         privateKey: '-----INSERT PRIVATE KEY-----'
///         bundleMethod: ubiquitous
///         geoRestrictions: us
///         type: legacy_custom
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/customSsl:CustomSsl example <zone_id>/<certificate_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod custom_ssl {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomSslArgs {
        /// The certificate associated parameters. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub custom_ssl_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::CustomSslCustomSslOptions>,
        >,
        #[builder(into, default)]
        pub custom_ssl_priorities: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::CustomSslCustomSslPriority>>,
        >,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CustomSslResult {
        /// The certificate associated parameters. **Modifying this attribute will force creation of a new resource.**
        pub custom_ssl_options: pulumi_gestalt_rust::Output<
            Option<super::types::CustomSslCustomSslOptions>,
        >,
        pub custom_ssl_priorities: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::CustomSslCustomSslPriority>>,
        >,
        pub expires_on: pulumi_gestalt_rust::Output<String>,
        pub hosts: pulumi_gestalt_rust::Output<Vec<String>>,
        pub issuer: pulumi_gestalt_rust::Output<String>,
        pub modified_on: pulumi_gestalt_rust::Output<String>,
        pub priority: pulumi_gestalt_rust::Output<i32>,
        pub signature: pulumi_gestalt_rust::Output<String>,
        pub status: pulumi_gestalt_rust::Output<String>,
        pub uploaded_on: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CustomSslArgs,
    ) -> CustomSslResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let custom_ssl_options_binding = args
            .custom_ssl_options
            .get_output(context)
            .get_inner();
        let custom_ssl_priorities_binding = args
            .custom_ssl_priorities
            .get_output(context)
            .get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/customSsl:CustomSsl".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customSslOptions".into(),
                    value: &custom_ssl_options_binding,
                },
                register_interface::ObjectField {
                    name: "customSslPriorities".into(),
                    value: &custom_ssl_priorities_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CustomSslResult {
            custom_ssl_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customSslOptions"),
            ),
            custom_ssl_priorities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customSslPriorities"),
            ),
            expires_on: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expiresOn"),
            ),
            hosts: pulumi_gestalt_rust::__private::into_domain(o.extract_field("hosts")),
            issuer: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("issuer"),
            ),
            modified_on: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("modifiedOn"),
            ),
            priority: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            signature: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("signature"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            uploaded_on: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("uploadedOn"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
