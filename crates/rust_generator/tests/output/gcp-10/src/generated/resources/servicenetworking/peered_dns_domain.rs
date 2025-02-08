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
#[allow(clippy::doc_lazy_continuation)]
pub mod peered_dns_domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PeeredDnsDomainArgs {
        /// The DNS domain suffix of the peered DNS domain. Make sure to suffix with a `.` (dot).
        #[builder(into)]
        pub dns_suffix: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Internal name used for the peered DNS domain.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The network in the consumer project.
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The producer project number. If not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Private service connection between service and consumer network, defaults to `servicenetworking.googleapis.com`
        #[builder(into, default)]
        pub service: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PeeredDnsDomainResult {
        /// The DNS domain suffix of the peered DNS domain. Make sure to suffix with a `.` (dot).
        pub dns_suffix: pulumi_gestalt_rust::Output<String>,
        /// Internal name used for the peered DNS domain.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The network in the consumer project.
        pub network: pulumi_gestalt_rust::Output<String>,
        /// an identifier for the resource with format `services/{{service}}/projects/{{project}}/global/networks/{{network}}`
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// The producer project number. If not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Private service connection between service and consumer network, defaults to `servicenetworking.googleapis.com`
        pub service: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PeeredDnsDomainArgs,
    ) -> PeeredDnsDomainResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let dns_suffix_binding = args.dns_suffix.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let service_binding = args.service.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        PeeredDnsDomainResult {
            dns_suffix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsSuffix"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            parent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parent"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("service"),
            ),
        }
    }
}
