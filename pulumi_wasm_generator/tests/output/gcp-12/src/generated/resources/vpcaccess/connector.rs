/// Serverless VPC Access connector resource.
///
///
/// To get more information about Connector, see:
///
/// * [API documentation](https://cloud.google.com/vpc/docs/reference/vpcaccess/rest/v1/projects.locations.connectors)
/// * How-to Guides
///     * [Configuring Serverless VPC Access](https://cloud.google.com/vpc/docs/configure-serverless-vpc-access)
///
/// ## Example Usage
///
/// ### Vpc Access Connector
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let connector = connector::create(
///         "connector",
///         ConnectorArgs::builder()
///             .ip_cidr_range("10.8.0.0/28")
///             .max_instances(3)
///             .min_instances(2)
///             .name("vpc-con")
///             .network("default")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Vpc Access Connector Shared Vpc
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let connector = connector::create(
///         "connector",
///         ConnectorArgs::builder()
///             .machine_type("e2-standard-4")
///             .max_instances(3)
///             .min_instances(2)
///             .name("vpc-con")
///             .subnet(ConnectorSubnet::builder().name("${customTest.name}").build_struct())
///             .build_struct(),
///     );
///     let customTest = subnetwork::create(
///         "customTest",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.2.0.0/28")
///             .name("vpc-con")
///             .network("default")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Connector can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/connectors/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Connector can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vpcaccess/connector:Connector default projects/{{project}}/locations/{{region}}/connectors/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vpcaccess/connector:Connector default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vpcaccess/connector:Connector default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vpcaccess/connector:Connector default {{name}}
/// ```
///
pub mod connector {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectorArgs {
        /// The range of internal addresses that follows RFC 4632 notation. Example: `10.132.0.0/28`.
        #[builder(into, default)]
        pub ip_cidr_range: pulumi_wasm_rust::Output<Option<String>>,
        /// Machine type of VM Instance underlying connector. Default is e2-micro
        #[builder(into, default)]
        pub machine_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Maximum value of instances in autoscaling group underlying the connector. Value must be between 3 and 10, inclusive. Must be
        /// higher than the value specified by min_instances.
        #[builder(into, default)]
        pub max_instances: pulumi_wasm_rust::Output<Option<i32>>,
        /// Maximum throughput of the connector in Mbps, must be greater than `min_throughput`. Default is 300. Refers to the expected throughput
        /// when using an e2-micro machine type. Value must be a multiple of 100 from 300 through 1000. Must be higher than the value specified by
        /// min_throughput. Only one of `max_throughput` and `max_instances` can be specified. The use of max_throughput is discouraged in favor of max_instances.
        #[builder(into, default)]
        pub max_throughput: pulumi_wasm_rust::Output<Option<i32>>,
        /// Minimum value of instances in autoscaling group underlying the connector. Value must be between 2 and 9, inclusive. Must be
        /// lower than the value specified by max_instances.
        #[builder(into, default)]
        pub min_instances: pulumi_wasm_rust::Output<Option<i32>>,
        /// Minimum throughput of the connector in Mbps. Default and min is 200. Refers to the expected throughput when using an e2-micro machine type.
        /// Value must be a multiple of 100 from 200 through 900. Must be lower than the value specified by max_throughput.
        /// Only one of `min_throughput` and `min_instances` can be specified. The use of min_throughput is discouraged in favor of min_instances.
        #[builder(into, default)]
        pub min_throughput: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the resource (Max 25 characters).
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Name or self_link of the VPC network. Required if `ip_cidr_range` is set.
        #[builder(into, default)]
        pub network: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Region where the VPC Access connector resides. If it is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// The subnet in which to house the connector
        /// Structure is documented below.
        #[builder(into, default)]
        pub subnet: pulumi_wasm_rust::Output<
            Option<super::super::types::vpcaccess::ConnectorSubnet>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConnectorResult {
        /// List of projects using the connector.
        pub connected_projects: pulumi_wasm_rust::Output<Vec<String>>,
        /// The range of internal addresses that follows RFC 4632 notation. Example: `10.132.0.0/28`.
        pub ip_cidr_range: pulumi_wasm_rust::Output<Option<String>>,
        /// Machine type of VM Instance underlying connector. Default is e2-micro
        pub machine_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Maximum value of instances in autoscaling group underlying the connector. Value must be between 3 and 10, inclusive. Must be
        /// higher than the value specified by min_instances.
        pub max_instances: pulumi_wasm_rust::Output<i32>,
        /// Maximum throughput of the connector in Mbps, must be greater than `min_throughput`. Default is 300. Refers to the expected throughput
        /// when using an e2-micro machine type. Value must be a multiple of 100 from 300 through 1000. Must be higher than the value specified by
        /// min_throughput. Only one of `max_throughput` and `max_instances` can be specified. The use of max_throughput is discouraged in favor of max_instances.
        pub max_throughput: pulumi_wasm_rust::Output<i32>,
        /// Minimum value of instances in autoscaling group underlying the connector. Value must be between 2 and 9, inclusive. Must be
        /// lower than the value specified by max_instances.
        pub min_instances: pulumi_wasm_rust::Output<i32>,
        /// Minimum throughput of the connector in Mbps. Default and min is 200. Refers to the expected throughput when using an e2-micro machine type.
        /// Value must be a multiple of 100 from 200 through 900. Must be lower than the value specified by max_throughput.
        /// Only one of `min_throughput` and `min_instances` can be specified. The use of min_throughput is discouraged in favor of min_instances.
        pub min_throughput: pulumi_wasm_rust::Output<i32>,
        /// The name of the resource (Max 25 characters).
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// Name or self_link of the VPC network. Required if `ip_cidr_range` is set.
        pub network: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Region where the VPC Access connector resides. If it is not provided, the provider region is used.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The fully qualified name of this VPC connector
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// State of the VPC access connector.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The subnet in which to house the connector
        /// Structure is documented below.
        pub subnet: pulumi_wasm_rust::Output<
            Option<super::super::types::vpcaccess::ConnectorSubnet>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConnectorArgs) -> ConnectorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let ip_cidr_range_binding = args.ip_cidr_range.get_inner();
        let machine_type_binding = args.machine_type.get_inner();
        let max_instances_binding = args.max_instances.get_inner();
        let max_throughput_binding = args.max_throughput.get_inner();
        let min_instances_binding = args.min_instances.get_inner();
        let min_throughput_binding = args.min_throughput.get_inner();
        let name_binding = args.name.get_inner();
        let network_binding = args.network.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let subnet_binding = args.subnet.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vpcaccess/connector:Connector".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "ipCidrRange".into(),
                    value: &ip_cidr_range_binding,
                },
                register_interface::ObjectField {
                    name: "machineType".into(),
                    value: &machine_type_binding,
                },
                register_interface::ObjectField {
                    name: "maxInstances".into(),
                    value: &max_instances_binding,
                },
                register_interface::ObjectField {
                    name: "maxThroughput".into(),
                    value: &max_throughput_binding,
                },
                register_interface::ObjectField {
                    name: "minInstances".into(),
                    value: &min_instances_binding,
                },
                register_interface::ObjectField {
                    name: "minThroughput".into(),
                    value: &min_throughput_binding,
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
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "subnet".into(),
                    value: &subnet_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "connectedProjects".into(),
                },
                register_interface::ResultField {
                    name: "ipCidrRange".into(),
                },
                register_interface::ResultField {
                    name: "machineType".into(),
                },
                register_interface::ResultField {
                    name: "maxInstances".into(),
                },
                register_interface::ResultField {
                    name: "maxThroughput".into(),
                },
                register_interface::ResultField {
                    name: "minInstances".into(),
                },
                register_interface::ResultField {
                    name: "minThroughput".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "subnet".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConnectorResult {
            connected_projects: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectedProjects").unwrap(),
            ),
            ip_cidr_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipCidrRange").unwrap(),
            ),
            machine_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("machineType").unwrap(),
            ),
            max_instances: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxInstances").unwrap(),
            ),
            max_throughput: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxThroughput").unwrap(),
            ),
            min_instances: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minInstances").unwrap(),
            ),
            min_throughput: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minThroughput").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            subnet: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnet").unwrap(),
            ),
        }
    }
}
