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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CustomSslArgs,
    ) -> CustomSslResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let custom_ssl_options_binding = args.custom_ssl_options.get_output(context);
        let custom_ssl_priorities_binding = args
            .custom_ssl_priorities
            .get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/customSsl:CustomSsl".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customSslOptions".into(),
                    value: custom_ssl_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customSslPriorities".into(),
                    value: custom_ssl_priorities_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CustomSslResult {
            custom_ssl_options: o.get_field("customSslOptions"),
            custom_ssl_priorities: o.get_field("customSslPriorities"),
            expires_on: o.get_field("expiresOn"),
            hosts: o.get_field("hosts"),
            issuer: o.get_field("issuer"),
            modified_on: o.get_field("modifiedOn"),
            priority: o.get_field("priority"),
            signature: o.get_field("signature"),
            status: o.get_field("status"),
            uploaded_on: o.get_field("uploadedOn"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
