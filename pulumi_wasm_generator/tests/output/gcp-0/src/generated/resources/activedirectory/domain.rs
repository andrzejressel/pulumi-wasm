/// Creates a Microsoft AD domain
///
///
/// To get more information about Domain, see:
///
/// * [API documentation](https://cloud.google.com/managed-microsoft-ad/reference/rest/v1/projects.locations.global.domains)
/// * How-to Guides
///     * [Managed Microsoft Active Directory Quickstart](https://cloud.google.com/managed-microsoft-ad/docs/quickstarts)
///
/// ## Example Usage
///
/// ### Active Directory Domain Basic
///
///
/// ```yaml
/// resources:
///   ad-domain:
///     type: gcp:activedirectory:Domain
///     properties:
///       domainName: tfgen.org.com
///       locations:
///         - us-central1
///       reservedIpRange: 192.168.255.0/24
///       deletionProtection: false
/// ```
///
/// ## Import
///
/// Domain can be imported using any of these accepted formats:
///
/// * `{{project}}/{{name}}`
///
/// * `{{project}} {{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Domain can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:activedirectory/domain:Domain default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:activedirectory/domain:Domain default "{{project}} {{name}}"
/// ```
///
/// ```sh
/// $ pulumi import gcp:activedirectory/domain:Domain default {{name}}
/// ```
///
pub mod domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainArgs {
        /// The name of delegated administrator account used to perform Active Directory operations.
        /// If not specified, setupadmin will be used.
        #[builder(into, default)]
        pub admin: pulumi_wasm_rust::Output<Option<String>>,
        /// The full names of the Google Compute Engine networks the domain instance is connected to. The domain is only available on networks listed in authorizedNetworks.
        /// If CIDR subnets overlap between networks, domain creation will fail.
        #[builder(into, default)]
        pub authorized_networks: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        #[builder(into, default)]
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// The fully qualified domain name. e.g. mydomain.myorganization.com, with the restrictions
        /// of https://cloud.google.com/managed-microsoft-ad/reference/rest/v1/projects.locations.global.domains.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Resource labels that can contain user-provided metadata
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Locations where domain needs to be provisioned. [regions][compute/docs/regions-zones/]
        /// e.g. us-west1 or us-east4 Service supports up to 4 locations at once. Each location will use a /26 block.
        #[builder(into)]
        pub locations: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger.
        /// Ranges must be unique and non-overlapping with existing subnets in authorizedNetworks
        #[builder(into)]
        pub reserved_ip_range: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DomainResult {
        /// The name of delegated administrator account used to perform Active Directory operations.
        /// If not specified, setupadmin will be used.
        pub admin: pulumi_wasm_rust::Output<Option<String>>,
        /// The full names of the Google Compute Engine networks the domain instance is connected to. The domain is only available on networks listed in authorizedNetworks.
        /// If CIDR subnets overlap between networks, domain creation will fail.
        pub authorized_networks: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// The fully qualified domain name. e.g. mydomain.myorganization.com, with the restrictions
        /// of https://cloud.google.com/managed-microsoft-ad/reference/rest/v1/projects.locations.global.domains.
        ///
        ///
        /// - - -
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The fully-qualified domain name of the exposed domain used by clients to connect to the service.
        /// Similar to what would be chosen for an Active Directory set up on an internal network.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// Resource labels that can contain user-provided metadata
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Locations where domain needs to be provisioned. [regions][compute/docs/regions-zones/]
        /// e.g. us-west1 or us-east4 Service supports up to 4 locations at once. Each location will use a /26 block.
        pub locations: pulumi_wasm_rust::Output<Vec<String>>,
        /// The unique name of the domain using the format: `projects/{project}/locations/global/domains/{domainName}`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger.
        /// Ranges must be unique and non-overlapping with existing subnets in authorizedNetworks
        pub reserved_ip_range: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DomainArgs) -> DomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let admin_binding = args.admin.get_inner();
        let authorized_networks_binding = args.authorized_networks.get_inner();
        let deletion_protection_binding = args.deletion_protection.get_inner();
        let domain_name_binding = args.domain_name.get_inner();
        let labels_binding = args.labels.get_inner();
        let locations_binding = args.locations.get_inner();
        let project_binding = args.project.get_inner();
        let reserved_ip_range_binding = args.reserved_ip_range.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:activedirectory/domain:Domain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "admin".into(),
                    value: &admin_binding,
                },
                register_interface::ObjectField {
                    name: "authorizedNetworks".into(),
                    value: &authorized_networks_binding,
                },
                register_interface::ObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "locations".into(),
                    value: &locations_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "reservedIpRange".into(),
                    value: &reserved_ip_range_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "admin".into(),
                },
                register_interface::ResultField {
                    name: "authorizedNetworks".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "fqdn".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "locations".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "reservedIpRange".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DomainResult {
            admin: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("admin").unwrap(),
            ),
            authorized_networks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizedNetworks").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdn").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            locations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locations").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            reserved_ip_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reservedIpRange").unwrap(),
            ),
        }
    }
}
