/// Provides a Cloudflare device policy certificates resource. Device
/// policy certificate resources enable client device certificate
/// generation.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = device_policy_certificates::create(
///         "example",
///         DevicePolicyCertificatesArgs::builder()
///             .enabled(true)
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/devicePolicyCertificates:DevicePolicyCertificates example <zone_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod device_policy_certificates {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DevicePolicyCertificatesArgs {
        /// `true` if certificate generation is enabled.
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DevicePolicyCertificatesResult {
        /// `true` if certificate generation is enabled.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
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
        args: DevicePolicyCertificatesArgs,
    ) -> DevicePolicyCertificatesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/devicePolicyCertificates:DevicePolicyCertificates"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DevicePolicyCertificatesResult {
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
