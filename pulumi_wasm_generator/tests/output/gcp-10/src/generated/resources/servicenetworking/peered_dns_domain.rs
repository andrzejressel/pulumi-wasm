/// Allows management of a single peered DNS domain for an existing Google Cloud Platform project.
///
/// When using Google Cloud DNS to manage internal DNS, create peered DNS domains to make your DNS available to services like Google Cloud Build.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   name:
///     type: gcp:servicenetworking:PeeredDnsDomain
///     properties:
///       project: 1e+07
///       name: example-com
///       network: default
///       dnsSuffix: example.com.
///       service: peering-service
/// ```
///
/// ## Import
///
/// Project peered DNS domains can be imported using the `service`, `project`, `network` and `name`, where:
///
/// - `service` is the service connection, defaults to `servicenetworking.googleapis.com`.
///
/// - `project` is the producer project name.
///
/// - `network` is the consumer network name.
///
/// - `name` is the name of your peered DNS domain.
///
/// * `services/{service}/projects/{project}/global/networks/{network}/peeredDnsDomains/{name}`
///
/// When using the `pulumi import` command, project peered DNS domains can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:servicenetworking/peeredDnsDomain:PeeredDnsDomain default services/{service}/projects/{project}/global/networks/{network}/peeredDnsDomains/{name}
/// ```
///
pub mod peered_dns_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PeeredDnsDomainArgs {
        /// The DNS domain suffix of the peered DNS domain. Make sure to suffix with a `.` (dot).
        #[builder(into)]
        pub dns_suffix: pulumi_wasm_rust::Output<String>,
        /// Internal name used for the peered DNS domain.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The network in the consumer project.
        #[builder(into)]
        pub network: pulumi_wasm_rust::Output<String>,
        /// The producer project number. If not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Private service connection between service and consumer network, defaults to `servicenetworking.googleapis.com`
        #[builder(into, default)]
        pub service: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PeeredDnsDomainResult {
        /// The DNS domain suffix of the peered DNS domain. Make sure to suffix with a `.` (dot).
        pub dns_suffix: pulumi_wasm_rust::Output<String>,
        /// Internal name used for the peered DNS domain.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The network in the consumer project.
        pub network: pulumi_wasm_rust::Output<String>,
        /// an identifier for the resource with format `services/{{service}}/projects/{{project}}/global/networks/{{network}}`
        pub parent: pulumi_wasm_rust::Output<String>,
        /// The producer project number. If not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Private service connection between service and consumer network, defaults to `servicenetworking.googleapis.com`
        pub service: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PeeredDnsDomainArgs) -> PeeredDnsDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dns_suffix_binding = args.dns_suffix.get_inner();
        let name_binding = args.name.get_inner();
        let network_binding = args.network.get_inner();
        let project_binding = args.project.get_inner();
        let service_binding = args.service.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:servicenetworking/peeredDnsDomain:PeeredDnsDomain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dnsSuffix".into(),
                    value: &dns_suffix_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dnsSuffix".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "service".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PeeredDnsDomainResult {
            dns_suffix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsSuffix").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("service").unwrap(),
            ),
        }
    }
}
