/// Manage Service Connection Policies.
///
///
/// To get more information about ServiceConnectionPolicy, see:
///
/// * [API documentation](https://cloud.google.com/secure-web-proxy/docs/reference/networkconnectivity/rest/v1/projects.locations.networkConnectionPolicies)
/// * How-to Guides
///     * [About Service Connection Policies](https://cloud.google.com/vpc/docs/about-service-connection-policies#service-policies)
///
/// ## Example Usage
///
/// ### Network Connectivity Policy Basic
///
///
/// ```yaml
/// resources:
///   producerNet:
///     type: gcp:compute:Network
///     name: producer_net
///     properties:
///       name: producer-net
///       autoCreateSubnetworks: false
///   producerSubnet:
///     type: gcp:compute:Subnetwork
///     name: producer_subnet
///     properties:
///       name: producer-subnet
///       ipCidrRange: 10.0.0.0/16
///       region: us-central1
///       network: ${producerNet.id}
///   default:
///     type: gcp:networkconnectivity:ServiceConnectionPolicy
///     properties:
///       name: my-network-connectivity-policy
///       location: us-central1
///       serviceClass: my-basic-service-class
///       description: my basic service connection policy
///       network: ${producerNet.id}
///       pscConfig:
///         subnetworks:
///           - ${producerSubnet.id}
///         limit: 2
/// ```
///
/// ## Import
///
/// ServiceConnectionPolicy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/serviceConnectionPolicies/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, ServiceConnectionPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkconnectivity/serviceConnectionPolicy:ServiceConnectionPolicy default projects/{{project}}/locations/{{location}}/serviceConnectionPolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkconnectivity/serviceConnectionPolicy:ServiceConnectionPolicy default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkconnectivity/serviceConnectionPolicy:ServiceConnectionPolicy default {{location}}/{{name}}
/// ```
///
pub mod service_connection_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceConnectionPolicyArgs {
        /// Free-text description of the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// User-defined labels.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the ServiceConnectionPolicy.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of a ServiceConnectionPolicy. Format: projects/{project}/locations/{location}/serviceConnectionPolicies/{service_connection_policy} See: https://google.aip.dev/122#fields-representing-resource-names
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource path of the consumer network. Example: - projects/{projectNumOrId}/global/networks/{resourceId}.
        #[builder(into)]
        pub network: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration used for Private Service Connect connections. Used when Infrastructure is PSC.
        /// Structure is documented below.
        #[builder(into, default)]
        pub psc_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkconnectivity::ServiceConnectionPolicyPscConfig,
            >,
        >,
        /// The service class identifier for which this ServiceConnectionPolicy is for. The service class identifier is a unique, symbolic representation of a ServiceClass.
        /// It is provided by the Service Producer. Google services have a prefix of gcp. For example, gcp-cloud-sql. 3rd party services do not. For example, test-service-a3dfcx.
        #[builder(into)]
        pub service_class: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceConnectionPolicyResult {
        /// The timestamp when the resource was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Free-text description of the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The etag is computed by the server, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The type of underlying resources used to create the connection.
        pub infrastructure: pulumi_wasm_rust::Output<String>,
        /// User-defined labels.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the ServiceConnectionPolicy.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of a ServiceConnectionPolicy. Format: projects/{project}/locations/{location}/serviceConnectionPolicies/{service_connection_policy} See: https://google.aip.dev/122#fields-representing-resource-names
        pub name: pulumi_wasm_rust::Output<String>,
        /// The resource path of the consumer network. Example: - projects/{projectNumOrId}/global/networks/{resourceId}.
        pub network: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Configuration used for Private Service Connect connections. Used when Infrastructure is PSC.
        /// Structure is documented below.
        pub psc_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkconnectivity::ServiceConnectionPolicyPscConfig,
            >,
        >,
        /// Information about each Private Service Connect connection.
        /// Structure is documented below.
        pub psc_connections: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::networkconnectivity::ServiceConnectionPolicyPscConnection,
            >,
        >,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The service class identifier for which this ServiceConnectionPolicy is for. The service class identifier is a unique, symbolic representation of a ServiceClass.
        /// It is provided by the Service Producer. Google services have a prefix of gcp. For example, gcp-cloud-sql. 3rd party services do not. For example, test-service-a3dfcx.
        pub service_class: pulumi_wasm_rust::Output<String>,
        /// The timestamp when the resource was updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ServiceConnectionPolicyArgs,
    ) -> ServiceConnectionPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let network_binding = args.network.get_inner();
        let project_binding = args.project.get_inner();
        let psc_config_binding = args.psc_config.get_inner();
        let service_class_binding = args.service_class.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkconnectivity/serviceConnectionPolicy:ServiceConnectionPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "pscConfig".into(),
                    value: &psc_config_binding,
                },
                register_interface::ObjectField {
                    name: "serviceClass".into(),
                    value: &service_class_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "infrastructure".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
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
                    name: "pscConfig".into(),
                },
                register_interface::ResultField {
                    name: "pscConnections".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "serviceClass".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServiceConnectionPolicyResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            infrastructure: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("infrastructure").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
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
            psc_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pscConfig").unwrap(),
            ),
            psc_connections: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pscConnections").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            service_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceClass").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
