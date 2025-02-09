/// Provides an Amplify Domain Association resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = app::create(
///         "example",
///         AppArgs::builder()
///             .custom_rules(
///                 vec![
///                     AppCustomRule::builder().source("https://example.com").status("302")
///                     .target("https://www.example.com").build_struct(),
///                 ],
///             )
///             .name("app")
///             .build_struct(),
///     );
///     let exampleDomainAssociation = domain_association::create(
///         "exampleDomainAssociation",
///         DomainAssociationArgs::builder()
///             .app_id("${example.id}")
///             .domain_name("example.com")
///             .sub_domains(
///                 vec![
///                     DomainAssociationSubDomain::builder()
///                     .branchName("${master.branchName}").prefix("").build_struct(),
///                     DomainAssociationSubDomain::builder()
///                     .branchName("${master.branchName}").prefix("www").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let master = branch::create(
///         "master",
///         BranchArgs::builder()
///             .app_id("${example.id}")
///             .branch_name("master")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amplify domain association using `app_id` and `domain_name`. For example:
///
/// ```sh
/// $ pulumi import aws:amplify/domainAssociation:DomainAssociation app d2ypk4k47z8u6/example.com
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainAssociationArgs {
        /// Unique ID for an Amplify app.
        #[builder(into)]
        pub app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of SSL/TLS certificate to use for your custom domain. If you don't specify a certificate type, Amplify uses the default certificate that it provisions and manages for you.
        #[builder(into, default)]
        pub certificate_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::amplify::DomainAssociationCertificateSettings>,
        >,
        /// Domain name for the domain association.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Enables the automated creation of subdomains for branches.
        #[builder(into, default)]
        pub enable_auto_sub_domain: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Setting for the subdomain. Documented below.
        #[builder(into)]
        pub sub_domains: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::amplify::DomainAssociationSubDomain>,
        >,
        /// If enabled, the resource will wait for the domain association status to change to `PENDING_DEPLOYMENT` or `AVAILABLE`. Setting this to `false` will skip the process. Default: `true`.
        #[builder(into, default)]
        pub wait_for_verification: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct DomainAssociationResult {
        /// Unique ID for an Amplify app.
        pub app_id: pulumi_gestalt_rust::Output<String>,
        /// ARN for the domain association.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The type of SSL/TLS certificate to use for your custom domain. If you don't specify a certificate type, Amplify uses the default certificate that it provisions and manages for you.
        pub certificate_settings: pulumi_gestalt_rust::Output<
            super::super::types::amplify::DomainAssociationCertificateSettings,
        >,
        /// DNS records for certificate verification in a space-delimited format (`<record> CNAME <target>`).
        pub certificate_verification_dns_record: pulumi_gestalt_rust::Output<String>,
        /// Domain name for the domain association.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// Enables the automated creation of subdomains for branches.
        pub enable_auto_sub_domain: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Setting for the subdomain. Documented below.
        pub sub_domains: pulumi_gestalt_rust::Output<
            Vec<super::super::types::amplify::DomainAssociationSubDomain>,
        >,
        /// If enabled, the resource will wait for the domain association status to change to `PENDING_DEPLOYMENT` or `AVAILABLE`. Setting this to `false` will skip the process. Default: `true`.
        pub wait_for_verification: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainAssociationArgs,
    ) -> DomainAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_id_binding = args.app_id.get_output(context);
        let certificate_settings_binding = args.certificate_settings.get_output(context);
        let domain_name_binding = args.domain_name.get_output(context);
        let enable_auto_sub_domain_binding = args
            .enable_auto_sub_domain
            .get_output(context);
        let sub_domains_binding = args.sub_domains.get_output(context);
        let wait_for_verification_binding = args
            .wait_for_verification
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:amplify/domainAssociation:DomainAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appId".into(),
                    value: app_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateSettings".into(),
                    value: certificate_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: domain_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableAutoSubDomain".into(),
                    value: enable_auto_sub_domain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subDomains".into(),
                    value: sub_domains_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "waitForVerification".into(),
                    value: wait_for_verification_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DomainAssociationResult {
            app_id: o.get_field("appId"),
            arn: o.get_field("arn"),
            certificate_settings: o.get_field("certificateSettings"),
            certificate_verification_dns_record: o
                .get_field("certificateVerificationDnsRecord"),
            domain_name: o.get_field("domainName"),
            enable_auto_sub_domain: o.get_field("enableAutoSubDomain"),
            sub_domains: o.get_field("subDomains"),
            wait_for_verification: o.get_field("waitForVerification"),
        }
    }
}
