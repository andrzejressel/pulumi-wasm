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
pub mod service_trust {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceTrustArgs {
        /// The ID of the Active Directory Domain Service. Changing this forces a new Active Directory Domain Service Trust to be created.
        #[builder(into)]
        pub domain_service_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Active Directory Domain Service Trust. Changing this forces a new Active Directory Domain Service Trust to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The password of the inbound trust set in the on-premise Active Directory Domain Service.
        #[builder(into)]
        pub password: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of DNS IPs that are used to resolve the on-premise Active Directory Domain Service.
        #[builder(into)]
        pub trusted_domain_dns_ips: pulumi_wasm_rust::Output<Vec<String>>,
        /// The FQDN of the on-premise Active Directory Domain Service.
        #[builder(into)]
        pub trusted_domain_fqdn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceTrustResult {
        /// The ID of the Active Directory Domain Service. Changing this forces a new Active Directory Domain Service Trust to be created.
        pub domain_service_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Active Directory Domain Service Trust. Changing this forces a new Active Directory Domain Service Trust to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The password of the inbound trust set in the on-premise Active Directory Domain Service.
        pub password: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of DNS IPs that are used to resolve the on-premise Active Directory Domain Service.
        pub trusted_domain_dns_ips: pulumi_wasm_rust::Output<Vec<String>>,
        /// The FQDN of the on-premise Active Directory Domain Service.
        pub trusted_domain_fqdn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceTrustArgs) -> ServiceTrustResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_service_id_binding = args.domain_service_id.get_inner();
        let name_binding = args.name.get_inner();
        let password_binding = args.password.get_inner();
        let trusted_domain_dns_ips_binding = args.trusted_domain_dns_ips.get_inner();
        let trusted_domain_fqdn_binding = args.trusted_domain_fqdn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:domainservices/serviceTrust:ServiceTrust".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "domainServiceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "password".into(),
                },
                register_interface::ResultField {
                    name: "trustedDomainDnsIps".into(),
                },
                register_interface::ResultField {
                    name: "trustedDomainFqdn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServiceTrustResult {
            domain_service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainServiceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password").unwrap(),
            ),
            trusted_domain_dns_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustedDomainDnsIps").unwrap(),
            ),
            trusted_domain_fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustedDomainFqdn").unwrap(),
            ),
        }
    }
}