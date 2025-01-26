/// Adds a trust between Active Directory domains
///
///
/// To get more information about DomainTrust, see:
///
/// * [API documentation](https://cloud.google.com/managed-microsoft-ad/reference/rest/v1/projects.locations.global.domains/attachTrust)
/// * How-to Guides
///     * [Active Directory Trust](https://cloud.google.com/managed-microsoft-ad/docs/create-one-way-trust)
///
///
///
/// ## Example Usage
///
/// ### Active Directory Domain Trust Basic
///
///
/// ```yaml
/// resources:
///   ad-domain-trust:
///     type: gcp:activedirectory:DomainTrust
///     properties:
///       domain: test-managed-ad.com
///       targetDomainName: example-gcp.com
///       targetDnsIpAddresses:
///         - 10.1.0.100
///       trustDirection: OUTBOUND
///       trustType: FOREST
///       trustHandshakeSecret: Testing1!
///       deletionProtection: false
/// ```
///
/// ## Import
///
/// DomainTrust can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/domains/{{domain}}/{{target_domain_name}}`
///
/// * `{{project}}/{{domain}}/{{target_domain_name}}`
///
/// * `{{domain}}/{{target_domain_name}}`
///
/// When using the `pulumi import` command, DomainTrust can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:activedirectory/domainTrust:DomainTrust default projects/{{project}}/locations/global/domains/{{domain}}/{{target_domain_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:activedirectory/domainTrust:DomainTrust default {{project}}/{{domain}}/{{target_domain_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:activedirectory/domainTrust:DomainTrust default {{domain}}/{{target_domain_name}}
/// ```
///
pub mod domain_trust {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainTrustArgs {
        /// The fully qualified domain name. e.g. mydomain.myorganization.com, with the restrictions
        /// of https://cloud.google.com/managed-microsoft-ad/reference/rest/v1/projects.locations.global.domains.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub domain: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether the trusted side has forest/domain wide access or selective access to an approved set of resources.
        #[builder(into, default)]
        pub selective_authentication: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The target DNS server IP addresses which can resolve the remote domain involved in the trust.
        #[builder(into)]
        pub target_dns_ip_addresses: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The fully qualified target domain name which will be in trust with the current domain.
        #[builder(into)]
        pub target_domain_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The trust direction, which decides if the current domain is trusted, trusting, or both.
        /// Possible values are: `INBOUND`, `OUTBOUND`, `BIDIRECTIONAL`.
        #[builder(into)]
        pub trust_direction: pulumi_wasm_rust::InputOrOutput<String>,
        /// The trust secret used for the handshake with the target domain. This will not be stored.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        #[builder(into)]
        pub trust_handshake_secret: pulumi_wasm_rust::InputOrOutput<String>,
        /// The type of trust represented by the trust resource.
        /// Possible values are: `FOREST`, `EXTERNAL`.
        #[builder(into)]
        pub trust_type: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DomainTrustResult {
        /// The fully qualified domain name. e.g. mydomain.myorganization.com, with the restrictions
        /// of https://cloud.google.com/managed-microsoft-ad/reference/rest/v1/projects.locations.global.domains.
        ///
        ///
        /// - - -
        pub domain: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Whether the trusted side has forest/domain wide access or selective access to an approved set of resources.
        pub selective_authentication: pulumi_wasm_rust::Output<Option<bool>>,
        /// The target DNS server IP addresses which can resolve the remote domain involved in the trust.
        pub target_dns_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The fully qualified target domain name which will be in trust with the current domain.
        pub target_domain_name: pulumi_wasm_rust::Output<String>,
        /// The trust direction, which decides if the current domain is trusted, trusting, or both.
        /// Possible values are: `INBOUND`, `OUTBOUND`, `BIDIRECTIONAL`.
        pub trust_direction: pulumi_wasm_rust::Output<String>,
        /// The trust secret used for the handshake with the target domain. This will not be stored.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        pub trust_handshake_secret: pulumi_wasm_rust::Output<String>,
        /// The type of trust represented by the trust resource.
        /// Possible values are: `FOREST`, `EXTERNAL`.
        pub trust_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DomainTrustArgs,
    ) -> DomainTrustResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_binding = args.domain.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let selective_authentication_binding = args
            .selective_authentication
            .get_output(context)
            .get_inner();
        let target_dns_ip_addresses_binding = args
            .target_dns_ip_addresses
            .get_output(context)
            .get_inner();
        let target_domain_name_binding = args
            .target_domain_name
            .get_output(context)
            .get_inner();
        let trust_direction_binding = args
            .trust_direction
            .get_output(context)
            .get_inner();
        let trust_handshake_secret_binding = args
            .trust_handshake_secret
            .get_output(context)
            .get_inner();
        let trust_type_binding = args.trust_type.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:activedirectory/domainTrust:DomainTrust".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "selectiveAuthentication".into(),
                    value: &selective_authentication_binding,
                },
                register_interface::ObjectField {
                    name: "targetDnsIpAddresses".into(),
                    value: &target_dns_ip_addresses_binding,
                },
                register_interface::ObjectField {
                    name: "targetDomainName".into(),
                    value: &target_domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "trustDirection".into(),
                    value: &trust_direction_binding,
                },
                register_interface::ObjectField {
                    name: "trustHandshakeSecret".into(),
                    value: &trust_handshake_secret_binding,
                },
                register_interface::ObjectField {
                    name: "trustType".into(),
                    value: &trust_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "domain".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "selectiveAuthentication".into(),
                },
                register_interface::ResultField {
                    name: "targetDnsIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "targetDomainName".into(),
                },
                register_interface::ResultField {
                    name: "trustDirection".into(),
                },
                register_interface::ResultField {
                    name: "trustHandshakeSecret".into(),
                },
                register_interface::ResultField {
                    name: "trustType".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DomainTrustResult {
            domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domain").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            selective_authentication: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selectiveAuthentication").unwrap(),
            ),
            target_dns_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetDnsIpAddresses").unwrap(),
            ),
            target_domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetDomainName").unwrap(),
            ),
            trust_direction: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustDirection").unwrap(),
            ),
            trust_handshake_secret: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustHandshakeSecret").unwrap(),
            ),
            trust_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustType").unwrap(),
            ),
        }
    }
}
