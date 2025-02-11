/// ## Example Usage
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/certificatePack:CertificatePack example <zone_id>/<certificate_pack_id>
/// ```
///
/// While supported, importing isn't recommended and it is advised to replace the
///
/// certificate entirely instead.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certificate_pack {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificatePackArgs {
        /// Which certificate authority to issue the certificate pack. Available values: `digicert`, `lets_encrypt`, `google`, `ssl_com`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub certificate_authority: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether or not to include Cloudflare branding. This will add `sni.cloudflaressl.com` as the Common Name if set to `true`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub cloudflare_branding: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of hostnames to provision the certificate pack for. The zone name must be included as a host. Note: If using Let's Encrypt, you cannot use individual subdomains and only a wildcard for subdomain is available. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub hosts: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Certificate pack configuration type. Available values: `advanced`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub validation_errors: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::CertificatePackValidationError>>,
        >,
        /// Which validation method to use in order to prove domain ownership. Available values: `txt`, `http`, `email`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub validation_method: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub validation_records: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::CertificatePackValidationRecord>>,
        >,
        /// How long the certificate is valid for. Note: If using Let's Encrypt, this value can only be 90 days. Available values: `14`, `30`, `90`, `365`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub validity_days: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Whether or not to wait for a certificate pack to reach status `active` during creation. Defaults to `false`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub wait_for_active_status: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CertificatePackResult {
        /// Which certificate authority to issue the certificate pack. Available values: `digicert`, `lets_encrypt`, `google`, `ssl_com`. **Modifying this attribute will force creation of a new resource.**
        pub certificate_authority: pulumi_gestalt_rust::Output<String>,
        /// Whether or not to include Cloudflare branding. This will add `sni.cloudflaressl.com` as the Common Name if set to `true`. **Modifying this attribute will force creation of a new resource.**
        pub cloudflare_branding: pulumi_gestalt_rust::Output<Option<bool>>,
        /// List of hostnames to provision the certificate pack for. The zone name must be included as a host. Note: If using Let's Encrypt, you cannot use individual subdomains and only a wildcard for subdomain is available. **Modifying this attribute will force creation of a new resource.**
        pub hosts: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Certificate pack configuration type. Available values: `advanced`. **Modifying this attribute will force creation of a new resource.**
        pub type_: pulumi_gestalt_rust::Output<String>,
        pub validation_errors: pulumi_gestalt_rust::Output<
            Vec<super::types::CertificatePackValidationError>,
        >,
        /// Which validation method to use in order to prove domain ownership. Available values: `txt`, `http`, `email`. **Modifying this attribute will force creation of a new resource.**
        pub validation_method: pulumi_gestalt_rust::Output<String>,
        pub validation_records: pulumi_gestalt_rust::Output<
            Vec<super::types::CertificatePackValidationRecord>,
        >,
        /// How long the certificate is valid for. Note: If using Let's Encrypt, this value can only be 90 days. Available values: `14`, `30`, `90`, `365`. **Modifying this attribute will force creation of a new resource.**
        pub validity_days: pulumi_gestalt_rust::Output<i32>,
        /// Whether or not to wait for a certificate pack to reach status `active` during creation. Defaults to `false`. **Modifying this attribute will force creation of a new resource.**
        pub wait_for_active_status: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CertificatePackArgs,
    ) -> CertificatePackResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_authority_binding = args
            .certificate_authority
            .get_output(context);
        let cloudflare_branding_binding = args.cloudflare_branding.get_output(context);
        let hosts_binding = args.hosts.get_output(context);
        let type__binding = args.type_.get_output(context);
        let validation_errors_binding = args.validation_errors.get_output(context);
        let validation_method_binding = args.validation_method.get_output(context);
        let validation_records_binding = args.validation_records.get_output(context);
        let validity_days_binding = args.validity_days.get_output(context);
        let wait_for_active_status_binding = args
            .wait_for_active_status
            .get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/certificatePack:CertificatePack".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateAuthority".into(),
                    value: &certificate_authority_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudflareBranding".into(),
                    value: &cloudflare_branding_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hosts".into(),
                    value: &hosts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validationErrors".into(),
                    value: &validation_errors_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validationMethod".into(),
                    value: &validation_method_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validationRecords".into(),
                    value: &validation_records_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validityDays".into(),
                    value: &validity_days_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "waitForActiveStatus".into(),
                    value: &wait_for_active_status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CertificatePackResult {
            certificate_authority: o.get_field("certificateAuthority"),
            cloudflare_branding: o.get_field("cloudflareBranding"),
            hosts: o.get_field("hosts"),
            type_: o.get_field("type"),
            validation_errors: o.get_field("validationErrors"),
            validation_method: o.get_field("validationMethod"),
            validation_records: o.get_field("validationRecords"),
            validity_days: o.get_field("validityDays"),
            wait_for_active_status: o.get_field("waitForActiveStatus"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
