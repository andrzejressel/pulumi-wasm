/// Represents a ServiceAttachment resource.
///
///
/// To get more information about ServiceAttachment, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/beta/serviceAttachments)
/// * How-to Guides
///     * [Configuring Private Service Connect to access services](https://cloud.google.com/vpc/docs/configure-private-service-connect-services)
///
/// ## Example Usage
///
/// ### Service Attachment Basic
///
///
/// ```yaml
/// resources:
///   pscIlbServiceAttachment:
///     type: gcp:compute:ServiceAttachment
///     name: psc_ilb_service_attachment
///     properties:
///       name: my-psc-ilb
///       region: us-west2
///       description: A service attachment configured with Terraform
///       domainNames:
///         - gcp.tfacc.hashicorptest.com.
///       enableProxyProtocol: true
///       connectionPreference: ACCEPT_AUTOMATIC
///       natSubnets:
///         - ${pscIlbNat.id}
///       targetService: ${pscIlbTargetService.id}
///   pscIlbConsumerAddress:
///     type: gcp:compute:Address
///     name: psc_ilb_consumer_address
///     properties:
///       name: psc-ilb-consumer-address
///       region: us-west2
///       subnetwork: default
///       addressType: INTERNAL
///   pscIlbConsumer:
///     type: gcp:compute:ForwardingRule
///     name: psc_ilb_consumer
///     properties:
///       name: psc-ilb-consumer-forwarding-rule
///       region: us-west2
///       target: ${pscIlbServiceAttachment.id}
///       loadBalancingScheme: ""
///       network: default
///       ipAddress: ${pscIlbConsumerAddress.id}
///   pscIlbTargetService:
///     type: gcp:compute:ForwardingRule
///     name: psc_ilb_target_service
///     properties:
///       name: producer-forwarding-rule
///       region: us-west2
///       loadBalancingScheme: INTERNAL
///       backendService: ${producerServiceBackend.id}
///       allPorts: true
///       network: ${pscIlbNetwork.name}
///       subnetwork: ${pscIlbProducerSubnetwork.name}
///   producerServiceBackend:
///     type: gcp:compute:RegionBackendService
///     name: producer_service_backend
///     properties:
///       name: producer-service
///       region: us-west2
///       healthChecks: ${producerServiceHealthCheck.id}
///   producerServiceHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: producer_service_health_check
///     properties:
///       name: producer-service-health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
///   pscIlbNetwork:
///     type: gcp:compute:Network
///     name: psc_ilb_network
///     properties:
///       name: psc-ilb-network
///       autoCreateSubnetworks: false
///   pscIlbProducerSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: psc_ilb_producer_subnetwork
///     properties:
///       name: psc-ilb-producer-subnetwork
///       region: us-west2
///       network: ${pscIlbNetwork.id}
///       ipCidrRange: 10.0.0.0/16
///   pscIlbNat:
///     type: gcp:compute:Subnetwork
///     name: psc_ilb_nat
///     properties:
///       name: psc-ilb-nat
///       region: us-west2
///       network: ${pscIlbNetwork.id}
///       purpose: PRIVATE_SERVICE_CONNECT
///       ipCidrRange: 10.1.0.0/16
/// ```
/// ### Service Attachment Explicit Projects
///
///
/// ```yaml
/// resources:
///   pscIlbServiceAttachment:
///     type: gcp:compute:ServiceAttachment
///     name: psc_ilb_service_attachment
///     properties:
///       name: my-psc-ilb
///       region: us-west2
///       description: A service attachment configured with Terraform
///       domainNames:
///         - gcp.tfacc.hashicorptest.com.
///       enableProxyProtocol: true
///       connectionPreference: ACCEPT_MANUAL
///       natSubnets:
///         - ${pscIlbNat.id}
///       targetService: ${pscIlbTargetService.id}
///       consumerRejectLists:
///         - '673497134629'
///         - '482878270665'
///       consumerAcceptLists:
///         - projectIdOrNum: '658859330310'
///           connectionLimit: 4
///   pscIlbConsumerAddress:
///     type: gcp:compute:Address
///     name: psc_ilb_consumer_address
///     properties:
///       name: psc-ilb-consumer-address
///       region: us-west2
///       subnetwork: default
///       addressType: INTERNAL
///   pscIlbConsumer:
///     type: gcp:compute:ForwardingRule
///     name: psc_ilb_consumer
///     properties:
///       name: psc-ilb-consumer-forwarding-rule
///       region: us-west2
///       target: ${pscIlbServiceAttachment.id}
///       loadBalancingScheme: ""
///       network: default
///       ipAddress: ${pscIlbConsumerAddress.id}
///   pscIlbTargetService:
///     type: gcp:compute:ForwardingRule
///     name: psc_ilb_target_service
///     properties:
///       name: producer-forwarding-rule
///       region: us-west2
///       loadBalancingScheme: INTERNAL
///       backendService: ${producerServiceBackend.id}
///       allPorts: true
///       network: ${pscIlbNetwork.name}
///       subnetwork: ${pscIlbProducerSubnetwork.name}
///   producerServiceBackend:
///     type: gcp:compute:RegionBackendService
///     name: producer_service_backend
///     properties:
///       name: producer-service
///       region: us-west2
///       healthChecks: ${producerServiceHealthCheck.id}
///   producerServiceHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: producer_service_health_check
///     properties:
///       name: producer-service-health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
///   pscIlbNetwork:
///     type: gcp:compute:Network
///     name: psc_ilb_network
///     properties:
///       name: psc-ilb-network
///       autoCreateSubnetworks: false
///   pscIlbProducerSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: psc_ilb_producer_subnetwork
///     properties:
///       name: psc-ilb-producer-subnetwork
///       region: us-west2
///       network: ${pscIlbNetwork.id}
///       ipCidrRange: 10.0.0.0/16
///   pscIlbNat:
///     type: gcp:compute:Subnetwork
///     name: psc_ilb_nat
///     properties:
///       name: psc-ilb-nat
///       region: us-west2
///       network: ${pscIlbNetwork.id}
///       purpose: PRIVATE_SERVICE_CONNECT
///       ipCidrRange: 10.1.0.0/16
/// ```
/// ### Service Attachment Explicit Networks
///
///
/// ```yaml
/// resources:
///   pscIlbServiceAttachment:
///     type: gcp:compute:ServiceAttachment
///     name: psc_ilb_service_attachment
///     properties:
///       name: my-psc-ilb
///       region: us-west2
///       description: A service attachment configured with Terraform
///       enableProxyProtocol: false
///       connectionPreference: ACCEPT_MANUAL
///       natSubnets:
///         - ${pscIlbNat.id}
///       targetService: ${pscIlbTargetService.id}
///       consumerAcceptLists:
///         - networkUrl: ${pscIlbConsumerNetwork.selfLink}
///           connectionLimit: 1
///   pscIlbConsumerNetwork:
///     type: gcp:compute:Network
///     name: psc_ilb_consumer_network
///     properties:
///       name: psc-ilb-consumer-network
///       autoCreateSubnetworks: false
///   pscIlbConsumerSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: psc_ilb_consumer_subnetwork
///     properties:
///       name: psc-ilb-consumer-network
///       ipCidrRange: 10.0.0.0/16
///       region: us-west2
///       network: ${pscIlbConsumerNetwork.id}
///   pscIlbConsumerAddress:
///     type: gcp:compute:Address
///     name: psc_ilb_consumer_address
///     properties:
///       name: psc-ilb-consumer-address
///       region: us-west2
///       subnetwork: ${pscIlbConsumerSubnetwork.id}
///       addressType: INTERNAL
///   pscIlbConsumer:
///     type: gcp:compute:ForwardingRule
///     name: psc_ilb_consumer
///     properties:
///       name: psc-ilb-consumer-forwarding-rule
///       region: us-west2
///       target: ${pscIlbServiceAttachment.id}
///       loadBalancingScheme: ""
///       network: ${pscIlbConsumerNetwork.id}
///       subnetwork: ${pscIlbConsumerSubnetwork.id}
///       ipAddress: ${pscIlbConsumerAddress.id}
///   pscIlbTargetService:
///     type: gcp:compute:ForwardingRule
///     name: psc_ilb_target_service
///     properties:
///       name: producer-forwarding-rule
///       region: us-west2
///       loadBalancingScheme: INTERNAL
///       backendService: ${producerServiceBackend.id}
///       allPorts: true
///       network: ${pscIlbNetwork.name}
///       subnetwork: ${pscIlbProducerSubnetwork.name}
///   producerServiceBackend:
///     type: gcp:compute:RegionBackendService
///     name: producer_service_backend
///     properties:
///       name: producer-service
///       region: us-west2
///       healthChecks: ${producerServiceHealthCheck.id}
///   producerServiceHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: producer_service_health_check
///     properties:
///       name: producer-service-health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
///   pscIlbNetwork:
///     type: gcp:compute:Network
///     name: psc_ilb_network
///     properties:
///       name: psc-ilb-network
///       autoCreateSubnetworks: false
///   pscIlbProducerSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: psc_ilb_producer_subnetwork
///     properties:
///       name: psc-ilb-producer-subnetwork
///       region: us-west2
///       network: ${pscIlbNetwork.id}
///       ipCidrRange: 10.0.0.0/16
///   pscIlbNat:
///     type: gcp:compute:Subnetwork
///     name: psc_ilb_nat
///     properties:
///       name: psc-ilb-nat
///       region: us-west2
///       network: ${pscIlbNetwork.id}
///       purpose: PRIVATE_SERVICE_CONNECT
///       ipCidrRange: 10.1.0.0/16
/// ```
/// ### Service Attachment Reconcile Connections
///
///
/// ```yaml
/// resources:
///   pscIlbServiceAttachment:
///     type: gcp:compute:ServiceAttachment
///     name: psc_ilb_service_attachment
///     properties:
///       name: my-psc-ilb
///       region: us-west2
///       description: A service attachment configured with Terraform
///       domainNames:
///         - gcp.tfacc.hashicorptest.com.
///       enableProxyProtocol: true
///       connectionPreference: ACCEPT_MANUAL
///       natSubnets:
///         - ${pscIlbNat.id}
///       targetService: ${pscIlbTargetService.id}
///       consumerRejectLists:
///         - '673497134629'
///         - '482878270665'
///       consumerAcceptLists:
///         - projectIdOrNum: '658859330310'
///           connectionLimit: 4
///       reconcileConnections: false
///   pscIlbTargetService:
///     type: gcp:compute:ForwardingRule
///     name: psc_ilb_target_service
///     properties:
///       name: producer-forwarding-rule
///       region: us-west2
///       loadBalancingScheme: INTERNAL
///       backendService: ${producerServiceBackend.id}
///       allPorts: true
///       network: ${pscIlbNetwork.name}
///       subnetwork: ${pscIlbProducerSubnetwork.name}
///   producerServiceBackend:
///     type: gcp:compute:RegionBackendService
///     name: producer_service_backend
///     properties:
///       name: producer-service
///       region: us-west2
///       healthChecks: ${producerServiceHealthCheck.id}
///   producerServiceHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: producer_service_health_check
///     properties:
///       name: producer-service-health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
///   pscIlbNetwork:
///     type: gcp:compute:Network
///     name: psc_ilb_network
///     properties:
///       name: psc-ilb-network
///       autoCreateSubnetworks: false
///   pscIlbProducerSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: psc_ilb_producer_subnetwork
///     properties:
///       name: psc-ilb-producer-subnetwork
///       region: us-west2
///       network: ${pscIlbNetwork.id}
///       ipCidrRange: 10.0.0.0/16
///   pscIlbNat:
///     type: gcp:compute:Subnetwork
///     name: psc_ilb_nat
///     properties:
///       name: psc-ilb-nat
///       region: us-west2
///       network: ${pscIlbNetwork.id}
///       purpose: PRIVATE_SERVICE_CONNECT
///       ipCidrRange: 10.1.0.0/16
/// ```
///
/// ## Import
///
/// ServiceAttachment can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/serviceAttachments/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, ServiceAttachment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/serviceAttachment:ServiceAttachment default projects/{{project}}/regions/{{region}}/serviceAttachments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/serviceAttachment:ServiceAttachment default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/serviceAttachment:ServiceAttachment default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/serviceAttachment:ServiceAttachment default {{name}}
/// ```
///
pub mod service_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceAttachmentArgs {
        /// The connection preference to use for this service attachment. Valid
        /// values include "ACCEPT_AUTOMATIC", "ACCEPT_MANUAL".
        #[builder(into)]
        pub connection_preference: pulumi_wasm_rust::Output<String>,
        /// An array of projects that are allowed to connect to this service
        /// attachment.
        /// Structure is documented below.
        #[builder(into, default)]
        pub consumer_accept_lists: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::compute::ServiceAttachmentConsumerAcceptList>,
            >,
        >,
        /// An array of projects that are not allowed to connect to this service
        /// attachment.
        #[builder(into, default)]
        pub consumer_reject_lists: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// If specified, the domain name will be used during the integration between
        /// the PSC connected endpoints and the Cloud DNS. For example, this is a
        /// valid domain name: "p.mycompany.com.". Current max number of domain names
        /// supported is 1.
        #[builder(into, default)]
        pub domain_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// If true, enable the proxy protocol which is for supplying client TCP/IP
        /// address data in TCP connections that traverse proxies on their way to
        /// destination servers.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub enable_proxy_protocol: pulumi_wasm_rust::Output<bool>,
        /// Name of the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?`
        /// which means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// An array of subnets that is provided for NAT in this service attachment.
        #[builder(into)]
        pub nat_subnets: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of consumer spokes that connected Private Service Connect endpoints can be propagated to through Network Connectivity Center.
        /// This limit lets the service producer limit how many propagated Private Service Connect connections can be established to this service attachment from a single consumer.
        /// If the connection preference of the service attachment is ACCEPT_MANUAL, the limit applies to each project or network that is listed in the consumer accept list.
        /// If the connection preference of the service attachment is ACCEPT_AUTOMATIC, the limit applies to each project that contains a connected endpoint.
        /// If unspecified, the default propagated connection limit is 250.
        #[builder(into, default)]
        pub propagated_connection_limit: pulumi_wasm_rust::Output<Option<i32>>,
        /// This flag determines whether a consumer accept/reject list change can reconcile the statuses of existing ACCEPTED or REJECTED PSC endpoints.
        /// If false, connection policy update will only affect existing PENDING PSC endpoints. Existing ACCEPTED/REJECTED endpoints will remain untouched regardless how the connection policy is modified .
        /// If true, update will affect both PENDING and ACCEPTED/REJECTED PSC endpoints. For example, an ACCEPTED PSC endpoint will be moved to REJECTED if its project is added to the reject list.
        #[builder(into, default)]
        pub reconcile_connections: pulumi_wasm_rust::Output<Option<bool>>,
        /// URL of the region where the resource resides.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// The URL of a service serving the endpoint identified by this service attachment.
        #[builder(into)]
        pub target_service: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceAttachmentResult {
        /// An array of the consumer forwarding rules connected to this service
        /// attachment.
        /// Structure is documented below.
        pub connected_endpoints: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::ServiceAttachmentConnectedEndpoint>,
        >,
        /// The connection preference to use for this service attachment. Valid
        /// values include "ACCEPT_AUTOMATIC", "ACCEPT_MANUAL".
        pub connection_preference: pulumi_wasm_rust::Output<String>,
        /// An array of projects that are allowed to connect to this service
        /// attachment.
        /// Structure is documented below.
        pub consumer_accept_lists: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::compute::ServiceAttachmentConsumerAcceptList>,
            >,
        >,
        /// An array of projects that are not allowed to connect to this service
        /// attachment.
        pub consumer_reject_lists: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// An optional description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// If specified, the domain name will be used during the integration between
        /// the PSC connected endpoints and the Cloud DNS. For example, this is a
        /// valid domain name: "p.mycompany.com.". Current max number of domain names
        /// supported is 1.
        pub domain_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// If true, enable the proxy protocol which is for supplying client TCP/IP
        /// address data in TCP connections that traverse proxies on their way to
        /// destination servers.
        ///
        ///
        /// - - -
        pub enable_proxy_protocol: pulumi_wasm_rust::Output<bool>,
        /// Fingerprint of this resource. This field is used internally during
        /// updates of this resource.
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        /// Name of the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?`
        /// which means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        pub name: pulumi_wasm_rust::Output<String>,
        /// An array of subnets that is provided for NAT in this service attachment.
        pub nat_subnets: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The number of consumer spokes that connected Private Service Connect endpoints can be propagated to through Network Connectivity Center.
        /// This limit lets the service producer limit how many propagated Private Service Connect connections can be established to this service attachment from a single consumer.
        /// If the connection preference of the service attachment is ACCEPT_MANUAL, the limit applies to each project or network that is listed in the consumer accept list.
        /// If the connection preference of the service attachment is ACCEPT_AUTOMATIC, the limit applies to each project that contains a connected endpoint.
        /// If unspecified, the default propagated connection limit is 250.
        pub propagated_connection_limit: pulumi_wasm_rust::Output<i32>,
        /// This flag determines whether a consumer accept/reject list change can reconcile the statuses of existing ACCEPTED or REJECTED PSC endpoints.
        /// If false, connection policy update will only affect existing PENDING PSC endpoints. Existing ACCEPTED/REJECTED endpoints will remain untouched regardless how the connection policy is modified .
        /// If true, update will affect both PENDING and ACCEPTED/REJECTED PSC endpoints. For example, an ACCEPTED PSC endpoint will be moved to REJECTED if its project is added to the reject list.
        pub reconcile_connections: pulumi_wasm_rust::Output<bool>,
        /// URL of the region where the resource resides.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// The URL of a service serving the endpoint identified by this service attachment.
        pub target_service: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceAttachmentArgs) -> ServiceAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let connection_preference_binding = args.connection_preference.get_inner();
        let consumer_accept_lists_binding = args.consumer_accept_lists.get_inner();
        let consumer_reject_lists_binding = args.consumer_reject_lists.get_inner();
        let description_binding = args.description.get_inner();
        let domain_names_binding = args.domain_names.get_inner();
        let enable_proxy_protocol_binding = args.enable_proxy_protocol.get_inner();
        let name_binding = args.name.get_inner();
        let nat_subnets_binding = args.nat_subnets.get_inner();
        let project_binding = args.project.get_inner();
        let propagated_connection_limit_binding = args
            .propagated_connection_limit
            .get_inner();
        let reconcile_connections_binding = args.reconcile_connections.get_inner();
        let region_binding = args.region.get_inner();
        let target_service_binding = args.target_service.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/serviceAttachment:ServiceAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "connectionPreference".into(),
                    value: &connection_preference_binding,
                },
                register_interface::ObjectField {
                    name: "consumerAcceptLists".into(),
                    value: &consumer_accept_lists_binding,
                },
                register_interface::ObjectField {
                    name: "consumerRejectLists".into(),
                    value: &consumer_reject_lists_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "domainNames".into(),
                    value: &domain_names_binding,
                },
                register_interface::ObjectField {
                    name: "enableProxyProtocol".into(),
                    value: &enable_proxy_protocol_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "natSubnets".into(),
                    value: &nat_subnets_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "propagatedConnectionLimit".into(),
                    value: &propagated_connection_limit_binding,
                },
                register_interface::ObjectField {
                    name: "reconcileConnections".into(),
                    value: &reconcile_connections_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "targetService".into(),
                    value: &target_service_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "connectedEndpoints".into(),
                },
                register_interface::ResultField {
                    name: "connectionPreference".into(),
                },
                register_interface::ResultField {
                    name: "consumerAcceptLists".into(),
                },
                register_interface::ResultField {
                    name: "consumerRejectLists".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "domainNames".into(),
                },
                register_interface::ResultField {
                    name: "enableProxyProtocol".into(),
                },
                register_interface::ResultField {
                    name: "fingerprint".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "natSubnets".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "propagatedConnectionLimit".into(),
                },
                register_interface::ResultField {
                    name: "reconcileConnections".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "targetService".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServiceAttachmentResult {
            connected_endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectedEndpoints").unwrap(),
            ),
            connection_preference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionPreference").unwrap(),
            ),
            consumer_accept_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("consumerAcceptLists").unwrap(),
            ),
            consumer_reject_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("consumerRejectLists").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            domain_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainNames").unwrap(),
            ),
            enable_proxy_protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableProxyProtocol").unwrap(),
            ),
            fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fingerprint").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            nat_subnets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("natSubnets").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            propagated_connection_limit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("propagatedConnectionLimit").unwrap(),
            ),
            reconcile_connections: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reconcileConnections").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            target_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetService").unwrap(),
            ),
        }
    }
}
