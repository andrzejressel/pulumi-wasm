/// Provides a resource to manage a domain that has been [registered](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/registrar-tld-list.html) and associated with the current AWS account.
///
/// **This is an advanced resource** and has special caveats to be aware of when using it. Please read this document in its entirety before using this resource.
///
/// The `aws.route53domains.RegisteredDomain` resource behaves differently from normal resources in that if a domain has been registered, the provider does not _register_ this domain, but instead "adopts" it into management. A destroy does not delete the domain but does remove the resource from state.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:route53domains:RegisteredDomain
///     properties:
///       domainName: example.com
///       nameServers:
///         - name: ns-195.awsdns-24.com
///         - name: ns-874.awsdns-45.net
///       tags:
///         Environment: test
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import domains using the domain name. For example:
///
/// ```sh
/// $ pulumi import aws:route53domains/registeredDomain:RegisteredDomain example example.com
/// ```
pub mod registered_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegisteredDomainArgs {
        /// Details about the domain administrative contact. See Contact Blocks for more details.
        #[builder(into, default)]
        pub admin_contact: pulumi_wasm_rust::Output<
            Option<super::super::types::route53domains::RegisteredDomainAdminContact>,
        >,
        /// Whether domain administrative contact information is concealed from WHOIS queries. Default: `true`.
        #[builder(into, default)]
        pub admin_privacy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the domain registration is set to renew automatically. Default: `true`.
        #[builder(into, default)]
        pub auto_renew: pulumi_wasm_rust::Output<Option<bool>>,
        /// Details about the domain billing contact. See Contact Blocks for more details.
        #[builder(into, default)]
        pub billing_contact: pulumi_wasm_rust::Output<
            Option<super::super::types::route53domains::RegisteredDomainBillingContact>,
        >,
        /// Whether domain billing contact information is concealed from WHOIS queries. Default: `true`.
        #[builder(into, default)]
        pub billing_privacy: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the registered domain.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// The list of nameservers for the domain. See `name_server` Blocks for more details.
        #[builder(into, default)]
        pub name_servers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::route53domains::RegisteredDomainNameServer>>,
        >,
        /// Details about the domain registrant. See Contact Blocks for more details.
        #[builder(into, default)]
        pub registrant_contact: pulumi_wasm_rust::Output<
            Option<
                super::super::types::route53domains::RegisteredDomainRegistrantContact,
            >,
        >,
        /// Whether domain registrant contact information is concealed from WHOIS queries. Default: `true`.
        #[builder(into, default)]
        pub registrant_privacy: pulumi_wasm_rust::Output<Option<bool>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Details about the domain technical contact. See Contact Blocks for more details.
        #[builder(into, default)]
        pub tech_contact: pulumi_wasm_rust::Output<
            Option<super::super::types::route53domains::RegisteredDomainTechContact>,
        >,
        /// Whether domain technical contact information is concealed from WHOIS queries. Default: `true`.
        #[builder(into, default)]
        pub tech_privacy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the domain is locked for transfer. Default: `true`.
        #[builder(into, default)]
        pub transfer_lock: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct RegisteredDomainResult {
        /// Email address to contact to report incorrect contact information for a domain, to report that the domain is being used to send spam, to report that someone is cybersquatting on a domain name, or report some other type of abuse.
        pub abuse_contact_email: pulumi_wasm_rust::Output<String>,
        /// Phone number for reporting abuse.
        pub abuse_contact_phone: pulumi_wasm_rust::Output<String>,
        /// Details about the domain administrative contact. See Contact Blocks for more details.
        pub admin_contact: pulumi_wasm_rust::Output<
            super::super::types::route53domains::RegisteredDomainAdminContact,
        >,
        /// Whether domain administrative contact information is concealed from WHOIS queries. Default: `true`.
        pub admin_privacy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the domain registration is set to renew automatically. Default: `true`.
        pub auto_renew: pulumi_wasm_rust::Output<Option<bool>>,
        /// Details about the domain billing contact. See Contact Blocks for more details.
        pub billing_contact: pulumi_wasm_rust::Output<
            super::super::types::route53domains::RegisteredDomainBillingContact,
        >,
        /// Whether domain billing contact information is concealed from WHOIS queries. Default: `true`.
        pub billing_privacy: pulumi_wasm_rust::Output<Option<bool>>,
        /// The date when the domain was created as found in the response to a WHOIS query.
        pub creation_date: pulumi_wasm_rust::Output<String>,
        /// The name of the registered domain.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// The date when the registration for the domain is set to expire.
        pub expiration_date: pulumi_wasm_rust::Output<String>,
        /// The list of nameservers for the domain. See `name_server` Blocks for more details.
        pub name_servers: pulumi_wasm_rust::Output<
            Vec<super::super::types::route53domains::RegisteredDomainNameServer>,
        >,
        /// Details about the domain registrant. See Contact Blocks for more details.
        pub registrant_contact: pulumi_wasm_rust::Output<
            super::super::types::route53domains::RegisteredDomainRegistrantContact,
        >,
        /// Whether domain registrant contact information is concealed from WHOIS queries. Default: `true`.
        pub registrant_privacy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the registrar of the domain as identified in the registry.
        pub registrar_name: pulumi_wasm_rust::Output<String>,
        /// Web address of the registrar.
        pub registrar_url: pulumi_wasm_rust::Output<String>,
        /// Reseller of the domain.
        pub reseller: pulumi_wasm_rust::Output<String>,
        /// List of [domain name status codes](https://www.icann.org/resources/pages/epp-status-codes-2014-06-16-en).
        pub status_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Details about the domain technical contact. See Contact Blocks for more details.
        pub tech_contact: pulumi_wasm_rust::Output<
            super::super::types::route53domains::RegisteredDomainTechContact,
        >,
        /// Whether domain technical contact information is concealed from WHOIS queries. Default: `true`.
        pub tech_privacy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the domain is locked for transfer. Default: `true`.
        pub transfer_lock: pulumi_wasm_rust::Output<Option<bool>>,
        /// The last updated date of the domain as found in the response to a WHOIS query.
        pub updated_date: pulumi_wasm_rust::Output<String>,
        /// The fully qualified name of the WHOIS server that can answer the WHOIS query for the domain.
        pub whois_server: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RegisteredDomainArgs) -> RegisteredDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let admin_contact_binding = args.admin_contact.get_inner();
        let admin_privacy_binding = args.admin_privacy.get_inner();
        let auto_renew_binding = args.auto_renew.get_inner();
        let billing_contact_binding = args.billing_contact.get_inner();
        let billing_privacy_binding = args.billing_privacy.get_inner();
        let domain_name_binding = args.domain_name.get_inner();
        let name_servers_binding = args.name_servers.get_inner();
        let registrant_contact_binding = args.registrant_contact.get_inner();
        let registrant_privacy_binding = args.registrant_privacy.get_inner();
        let tags_binding = args.tags.get_inner();
        let tech_contact_binding = args.tech_contact.get_inner();
        let tech_privacy_binding = args.tech_privacy.get_inner();
        let transfer_lock_binding = args.transfer_lock.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53domains/registeredDomain:RegisteredDomain".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "adminContact".into(),
                    value: &admin_contact_binding,
                },
                register_interface::ObjectField {
                    name: "adminPrivacy".into(),
                    value: &admin_privacy_binding,
                },
                register_interface::ObjectField {
                    name: "autoRenew".into(),
                    value: &auto_renew_binding,
                },
                register_interface::ObjectField {
                    name: "billingContact".into(),
                    value: &billing_contact_binding,
                },
                register_interface::ObjectField {
                    name: "billingPrivacy".into(),
                    value: &billing_privacy_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "nameServers".into(),
                    value: &name_servers_binding,
                },
                register_interface::ObjectField {
                    name: "registrantContact".into(),
                    value: &registrant_contact_binding,
                },
                register_interface::ObjectField {
                    name: "registrantPrivacy".into(),
                    value: &registrant_privacy_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "techContact".into(),
                    value: &tech_contact_binding,
                },
                register_interface::ObjectField {
                    name: "techPrivacy".into(),
                    value: &tech_privacy_binding,
                },
                register_interface::ObjectField {
                    name: "transferLock".into(),
                    value: &transfer_lock_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "abuseContactEmail".into(),
                },
                register_interface::ResultField {
                    name: "abuseContactPhone".into(),
                },
                register_interface::ResultField {
                    name: "adminContact".into(),
                },
                register_interface::ResultField {
                    name: "adminPrivacy".into(),
                },
                register_interface::ResultField {
                    name: "autoRenew".into(),
                },
                register_interface::ResultField {
                    name: "billingContact".into(),
                },
                register_interface::ResultField {
                    name: "billingPrivacy".into(),
                },
                register_interface::ResultField {
                    name: "creationDate".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "expirationDate".into(),
                },
                register_interface::ResultField {
                    name: "nameServers".into(),
                },
                register_interface::ResultField {
                    name: "registrantContact".into(),
                },
                register_interface::ResultField {
                    name: "registrantPrivacy".into(),
                },
                register_interface::ResultField {
                    name: "registrarName".into(),
                },
                register_interface::ResultField {
                    name: "registrarUrl".into(),
                },
                register_interface::ResultField {
                    name: "reseller".into(),
                },
                register_interface::ResultField {
                    name: "statusLists".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "techContact".into(),
                },
                register_interface::ResultField {
                    name: "techPrivacy".into(),
                },
                register_interface::ResultField {
                    name: "transferLock".into(),
                },
                register_interface::ResultField {
                    name: "updatedDate".into(),
                },
                register_interface::ResultField {
                    name: "whoisServer".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RegisteredDomainResult {
            abuse_contact_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("abuseContactEmail").unwrap(),
            ),
            abuse_contact_phone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("abuseContactPhone").unwrap(),
            ),
            admin_contact: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adminContact").unwrap(),
            ),
            admin_privacy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adminPrivacy").unwrap(),
            ),
            auto_renew: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoRenew").unwrap(),
            ),
            billing_contact: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("billingContact").unwrap(),
            ),
            billing_privacy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("billingPrivacy").unwrap(),
            ),
            creation_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationDate").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            expiration_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expirationDate").unwrap(),
            ),
            name_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nameServers").unwrap(),
            ),
            registrant_contact: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registrantContact").unwrap(),
            ),
            registrant_privacy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registrantPrivacy").unwrap(),
            ),
            registrar_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registrarName").unwrap(),
            ),
            registrar_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registrarUrl").unwrap(),
            ),
            reseller: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reseller").unwrap(),
            ),
            status_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statusLists").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            tech_contact: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("techContact").unwrap(),
            ),
            tech_privacy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("techPrivacy").unwrap(),
            ),
            transfer_lock: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transferLock").unwrap(),
            ),
            updated_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updatedDate").unwrap(),
            ),
            whois_server: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("whoisServer").unwrap(),
            ),
        }
    }
}
