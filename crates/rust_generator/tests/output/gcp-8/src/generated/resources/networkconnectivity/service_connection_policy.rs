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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_connection_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceConnectionPolicyArgs {
        /// Free-text description of the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User-defined labels.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the ServiceConnectionPolicy.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of a ServiceConnectionPolicy. Format: projects/{project}/locations/{location}/serviceConnectionPolicies/{service_connection_policy} See: https://google.aip.dev/122#fields-representing-resource-names
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource path of the consumer network. Example: - projects/{projectNumOrId}/global/networks/{resourceId}.
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration used for Private Service Connect connections. Used when Infrastructure is PSC.
        /// Structure is documented below.
        #[builder(into, default)]
        pub psc_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::networkconnectivity::ServiceConnectionPolicyPscConfig,
            >,
        >,
        /// The service class identifier for which this ServiceConnectionPolicy is for. The service class identifier is a unique, symbolic representation of a ServiceClass.
        /// It is provided by the Service Producer. Google services have a prefix of gcp. For example, gcp-cloud-sql. 3rd party services do not. For example, test-service-a3dfcx.
        #[builder(into)]
        pub service_class: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceConnectionPolicyResult {
        /// The timestamp when the resource was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Free-text description of the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The etag is computed by the server, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The type of underlying resources used to create the connection.
        pub infrastructure: pulumi_gestalt_rust::Output<String>,
        /// User-defined labels.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the ServiceConnectionPolicy.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of a ServiceConnectionPolicy. Format: projects/{project}/locations/{location}/serviceConnectionPolicies/{service_connection_policy} See: https://google.aip.dev/122#fields-representing-resource-names
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The resource path of the consumer network. Example: - projects/{projectNumOrId}/global/networks/{resourceId}.
        pub network: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Configuration used for Private Service Connect connections. Used when Infrastructure is PSC.
        /// Structure is documented below.
        pub psc_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::networkconnectivity::ServiceConnectionPolicyPscConfig,
            >,
        >,
        /// Information about each Private Service Connect connection.
        /// Structure is documented below.
        pub psc_connections: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::networkconnectivity::ServiceConnectionPolicyPscConnection,
            >,
        >,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The service class identifier for which this ServiceConnectionPolicy is for. The service class identifier is a unique, symbolic representation of a ServiceClass.
        /// It is provided by the Service Producer. Google services have a prefix of gcp. For example, gcp-cloud-sql. 3rd party services do not. For example, test-service-a3dfcx.
        pub service_class: pulumi_gestalt_rust::Output<String>,
        /// The timestamp when the resource was updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceConnectionPolicyArgs,
    ) -> ServiceConnectionPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let project_binding = args.project.get_output(context);
        let psc_config_binding = args.psc_config.get_output(context);
        let service_class_binding = args.service_class.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networkconnectivity/serviceConnectionPolicy:ServiceConnectionPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: network_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pscConfig".into(),
                    value: psc_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceClass".into(),
                    value: service_class_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceConnectionPolicyResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            etag: o.get_field("etag"),
            infrastructure: o.get_field("infrastructure"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            project: o.get_field("project"),
            psc_config: o.get_field("pscConfig"),
            psc_connections: o.get_field("pscConnections"),
            pulumi_labels: o.get_field("pulumiLabels"),
            service_class: o.get_field("serviceClass"),
            update_time: o.get_field("updateTime"),
        }
    }
}
