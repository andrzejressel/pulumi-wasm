/// Provides an Amplify Domain Association resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod domain_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainAssociationArgs {
        /// Unique ID for an Amplify app.
        #[builder(into)]
        pub app_id: pulumi_wasm_rust::Output<String>,
        /// The type of SSL/TLS certificate to use for your custom domain. If you don't specify a certificate type, Amplify uses the default certificate that it provisions and manages for you.
        #[builder(into, default)]
        pub certificate_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::amplify::DomainAssociationCertificateSettings>,
        >,
        /// Domain name for the domain association.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Enables the automated creation of subdomains for branches.
        #[builder(into, default)]
        pub enable_auto_sub_domain: pulumi_wasm_rust::Output<Option<bool>>,
        /// Setting for the subdomain. Documented below.
        #[builder(into)]
        pub sub_domains: pulumi_wasm_rust::Output<
            Vec<super::super::types::amplify::DomainAssociationSubDomain>,
        >,
        /// If enabled, the resource will wait for the domain association status to change to `PENDING_DEPLOYMENT` or `AVAILABLE`. Setting this to `false` will skip the process. Default: `true`.
        #[builder(into, default)]
        pub wait_for_verification: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct DomainAssociationResult {
        /// Unique ID for an Amplify app.
        pub app_id: pulumi_wasm_rust::Output<String>,
        /// ARN for the domain association.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The type of SSL/TLS certificate to use for your custom domain. If you don't specify a certificate type, Amplify uses the default certificate that it provisions and manages for you.
        pub certificate_settings: pulumi_wasm_rust::Output<
            super::super::types::amplify::DomainAssociationCertificateSettings,
        >,
        /// DNS records for certificate verification in a space-delimited format (`<record> CNAME <target>`).
        pub certificate_verification_dns_record: pulumi_wasm_rust::Output<String>,
        /// Domain name for the domain association.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Enables the automated creation of subdomains for branches.
        pub enable_auto_sub_domain: pulumi_wasm_rust::Output<Option<bool>>,
        /// Setting for the subdomain. Documented below.
        pub sub_domains: pulumi_wasm_rust::Output<
            Vec<super::super::types::amplify::DomainAssociationSubDomain>,
        >,
        /// If enabled, the resource will wait for the domain association status to change to `PENDING_DEPLOYMENT` or `AVAILABLE`. Setting this to `false` will skip the process. Default: `true`.
        pub wait_for_verification: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DomainAssociationArgs) -> DomainAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_id_binding = args.app_id.get_inner();
        let certificate_settings_binding = args.certificate_settings.get_inner();
        let domain_name_binding = args.domain_name.get_inner();
        let enable_auto_sub_domain_binding = args.enable_auto_sub_domain.get_inner();
        let sub_domains_binding = args.sub_domains.get_inner();
        let wait_for_verification_binding = args.wait_for_verification.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:amplify/domainAssociation:DomainAssociation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appId".into(),
                    value: &app_id_binding,
                },
                register_interface::ObjectField {
                    name: "certificateSettings".into(),
                    value: &certificate_settings_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "enableAutoSubDomain".into(),
                    value: &enable_auto_sub_domain_binding,
                },
                register_interface::ObjectField {
                    name: "subDomains".into(),
                    value: &sub_domains_binding,
                },
                register_interface::ObjectField {
                    name: "waitForVerification".into(),
                    value: &wait_for_verification_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "certificateSettings".into(),
                },
                register_interface::ResultField {
                    name: "certificateVerificationDnsRecord".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "enableAutoSubDomain".into(),
                },
                register_interface::ResultField {
                    name: "subDomains".into(),
                },
                register_interface::ResultField {
                    name: "waitForVerification".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DomainAssociationResult {
            app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            certificate_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateSettings").unwrap(),
            ),
            certificate_verification_dns_record: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateVerificationDnsRecord").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            enable_auto_sub_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableAutoSubDomain").unwrap(),
            ),
            sub_domains: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subDomains").unwrap(),
            ),
            wait_for_verification: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("waitForVerification").unwrap(),
            ),
        }
    }
}
