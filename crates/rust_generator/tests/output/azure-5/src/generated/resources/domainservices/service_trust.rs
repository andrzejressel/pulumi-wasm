/// Manages a Active Directory Domain Service Trust.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleServiceTrust:
///     type: azure:domainservices:ServiceTrust
///     name: example
///     properties:
///       name: example-trust
///       domainServiceId: ${example.id}
///       trustedDomainFqdn: example.com
///       trustedDomainDnsIps:
///         - 10.1.0.3
///         - 10.1.0.4
///       password: Password123
/// variables:
///   example:
///     fn::invoke:
///       function: azure:domainservices:getService
///       arguments:
///         name: example-ds
///         resourceGroupName: example-rg
/// ```
///
/// ## Import
///
/// Active Directory Domain Service Trusts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:domainservices/serviceTrust:ServiceTrust example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.AAD/domainServices/DomainService1/trusts/trust1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_trust {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceTrustArgs {
        /// The ID of the Active Directory Domain Service. Changing this forces a new Active Directory Domain Service Trust to be created.
        #[builder(into)]
        pub domain_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Active Directory Domain Service Trust. Changing this forces a new Active Directory Domain Service Trust to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The password of the inbound trust set in the on-premise Active Directory Domain Service.
        #[builder(into)]
        pub password: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies a list of DNS IPs that are used to resolve the on-premise Active Directory Domain Service.
        #[builder(into)]
        pub trusted_domain_dns_ips: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The FQDN of the on-premise Active Directory Domain Service.
        #[builder(into)]
        pub trusted_domain_fqdn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceTrustResult {
        /// The ID of the Active Directory Domain Service. Changing this forces a new Active Directory Domain Service Trust to be created.
        pub domain_service_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Active Directory Domain Service Trust. Changing this forces a new Active Directory Domain Service Trust to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The password of the inbound trust set in the on-premise Active Directory Domain Service.
        pub password: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of DNS IPs that are used to resolve the on-premise Active Directory Domain Service.
        pub trusted_domain_dns_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The FQDN of the on-premise Active Directory Domain Service.
        pub trusted_domain_fqdn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ServiceTrustArgs,
    ) -> ServiceTrustResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let domain_service_id_binding_1 = args.domain_service_id.get_output(context);
        let domain_service_id_binding = domain_service_id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let password_binding_1 = args.password.get_output(context);
        let password_binding = password_binding_1.get_inner();
        let trusted_domain_dns_ips_binding_1 = args
            .trusted_domain_dns_ips
            .get_output(context);
        let trusted_domain_dns_ips_binding = trusted_domain_dns_ips_binding_1
            .get_inner();
        let trusted_domain_fqdn_binding_1 = args.trusted_domain_fqdn.get_output(context);
        let trusted_domain_fqdn_binding = trusted_domain_fqdn_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:domainservices/serviceTrust:ServiceTrust".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainServiceId".into(),
                    value: &domain_service_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "trustedDomainDnsIps".into(),
                    value: &trusted_domain_dns_ips_binding,
                },
                register_interface::ObjectField {
                    name: "trustedDomainFqdn".into(),
                    value: &trusted_domain_fqdn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServiceTrustResult {
            domain_service_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainServiceId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            trusted_domain_dns_ips: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("trustedDomainDnsIps"),
            ),
            trusted_domain_fqdn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("trustedDomainFqdn"),
            ),
        }
    }
}
