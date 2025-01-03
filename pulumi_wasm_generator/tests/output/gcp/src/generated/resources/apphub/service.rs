/// Service is a network/api interface that exposes some functionality to clients for consumption over the network. Service typically has one or more Workloads behind it. It registers identified service to the Application.
///
///
///
/// ## Example Usage
///
/// ### Apphub Service Basic
///
///
/// ```yaml
/// resources:
///   application:
///     type: gcp:apphub:Application
///     properties:
///       location: us-central1
///       applicationId: example-application-1
///       scope:
///         type: REGIONAL
///   serviceProject:
///     type: gcp:organizations:Project
///     name: service_project
///     properties:
///       projectId: project-1
///       name: Service Project
///       orgId: '123456789'
///       billingAccount: 000000-0000000-0000000-000000
///       deletionPolicy: DELETE
///   # Enable Compute API
///   computeServiceProject:
///     type: gcp:projects:Service
///     name: compute_service_project
///     properties:
///       project: ${serviceProject.projectId}
///       service: compute.googleapis.com
///   wait120s:
///     type: time:sleep
///     name: wait_120s
///     properties:
///       createDuration: 120s
///     options:
///       dependsOn:
///         - ${computeServiceProject}
///   serviceProjectAttachment:
///     type: gcp:apphub:ServiceProjectAttachment
///     name: service_project_attachment
///     properties:
///       serviceProjectAttachmentId: ${serviceProject.projectId}
///     options:
///       dependsOn:
///         - ${wait120s}
///   wait120sForResourceIngestion:
///     type: time:sleep
///     name: wait_120s_for_resource_ingestion
///     properties:
///       createDuration: 120s
///     options:
///       dependsOn:
///         - ${forwardingRule}
///   example:
///     type: gcp:apphub:Service
///     properties:
///       location: us-central1
///       applicationId: ${application.applicationId}
///       serviceId: ${forwardingRule.name}
///       discoveredService: ${["catalog-service"].name}
///   # VPC network
///   ilbNetwork:
///     type: gcp:compute:Network
///     name: ilb_network
///     properties:
///       name: l7-ilb-network
///       project: ${serviceProject.projectId}
///       autoCreateSubnetworks: false
///     options:
///       dependsOn:
///         - ${wait120s}
///   # backend subnet
///   ilbSubnet:
///     type: gcp:compute:Subnetwork
///     name: ilb_subnet
///     properties:
///       name: l7-ilb-subnet
///       project: ${serviceProject.projectId}
///       ipCidrRange: 10.0.1.0/24
///       region: us-central1
///       network: ${ilbNetwork.id}
///   # forwarding rule
///   forwardingRule:
///     type: gcp:compute:ForwardingRule
///     name: forwarding_rule
///     properties:
///       name: l7-ilb-forwarding-rule
///       project: ${serviceProject.projectId}
///       region: us-central1
///       ipVersion: IPV4
///       loadBalancingScheme: INTERNAL
///       allPorts: true
///       backendService: ${backend.id}
///       network: ${ilbNetwork.id}
///       subnetwork: ${ilbSubnet.id}
///   # backend service
///   backend:
///     type: gcp:compute:RegionBackendService
///     properties:
///       name: l7-ilb-backend-subnet
///       project: ${serviceProject.projectId}
///       region: us-central1
///       healthChecks: ${default.id}
///   # health check
///   default:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: l7-ilb-hc
///       project: ${serviceProject.projectId}
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
///     options:
///       dependsOn:
///         - ${wait120s}
/// variables:
///   # discovered service block
///   catalog-service:
///     fn::invoke:
///       function: gcp:apphub:getDiscoveredService
///       arguments:
///         location: us-central1
///         serviceUri: //compute.googleapis.com/${forwardingRule.id}
/// ```
/// ### Apphub Service Full
///
///
/// ```yaml
/// resources:
///   application:
///     type: gcp:apphub:Application
///     properties:
///       location: us-central1
///       applicationId: example-application-1
///       scope:
///         type: REGIONAL
///   serviceProject:
///     type: gcp:organizations:Project
///     name: service_project
///     properties:
///       projectId: project-1
///       name: Service Project
///       orgId: '123456789'
///       billingAccount: 000000-0000000-0000000-000000
///       deletionPolicy: DELETE
///   # Enable Compute API
///   computeServiceProject:
///     type: gcp:projects:Service
///     name: compute_service_project
///     properties:
///       project: ${serviceProject.projectId}
///       service: compute.googleapis.com
///   wait120s:
///     type: time:sleep
///     name: wait_120s
///     properties:
///       createDuration: 120s
///     options:
///       dependsOn:
///         - ${computeServiceProject}
///   serviceProjectAttachment:
///     type: gcp:apphub:ServiceProjectAttachment
///     name: service_project_attachment
///     properties:
///       serviceProjectAttachmentId: ${serviceProject.projectId}
///     options:
///       dependsOn:
///         - ${wait120s}
///   wait120sForResourceIngestion:
///     type: time:sleep
///     name: wait_120s_for_resource_ingestion
///     properties:
///       createDuration: 120s
///     options:
///       dependsOn:
///         - ${forwardingRule}
///   example:
///     type: gcp:apphub:Service
///     properties:
///       location: us-central1
///       applicationId: ${application.applicationId}
///       serviceId: ${forwardingRule.name}
///       discoveredService: ${["catalog-service"].name}
///       displayName: Example Service Full
///       description: Register service for testing
///       attributes:
///         environment:
///           type: STAGING
///         criticality:
///           type: MISSION_CRITICAL
///         businessOwners:
///           - displayName: Alice
///             email: alice@google.com
///         developerOwners:
///           - displayName: Bob
///             email: bob@google.com
///         operatorOwners:
///           - displayName: Charlie
///             email: charlie@google.com
///   # VPC network
///   ilbNetwork:
///     type: gcp:compute:Network
///     name: ilb_network
///     properties:
///       name: l7-ilb-network
///       project: ${serviceProject.projectId}
///       autoCreateSubnetworks: false
///     options:
///       dependsOn:
///         - ${wait120s}
///   # backend subnet
///   ilbSubnet:
///     type: gcp:compute:Subnetwork
///     name: ilb_subnet
///     properties:
///       name: l7-ilb-subnet
///       project: ${serviceProject.projectId}
///       ipCidrRange: 10.0.1.0/24
///       region: us-central1
///       network: ${ilbNetwork.id}
///   # forwarding rule
///   forwardingRule:
///     type: gcp:compute:ForwardingRule
///     name: forwarding_rule
///     properties:
///       name: l7-ilb-forwarding-rule
///       project: ${serviceProject.projectId}
///       region: us-central1
///       ipVersion: IPV4
///       loadBalancingScheme: INTERNAL
///       allPorts: true
///       backendService: ${backend.id}
///       network: ${ilbNetwork.id}
///       subnetwork: ${ilbSubnet.id}
///   # backend service
///   backend:
///     type: gcp:compute:RegionBackendService
///     properties:
///       name: l7-ilb-backend-subnet
///       project: ${serviceProject.projectId}
///       region: us-central1
///       healthChecks: ${default.id}
///   # health check
///   default:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: l7-ilb-hc
///       project: ${serviceProject.projectId}
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
///     options:
///       dependsOn:
///         - ${wait120s}
/// variables:
///   # discovered service block
///   catalog-service:
///     fn::invoke:
///       function: gcp:apphub:getDiscoveredService
///       arguments:
///         location: us-central1
///         serviceUri: //compute.googleapis.com/${forwardingRule.id}
/// ```
///
/// ## Import
///
/// Service can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/applications/{{application_id}}/services/{{service_id}}`
///
/// * `{{project}}/{{location}}/{{application_id}}/{{service_id}}`
///
/// * `{{location}}/{{application_id}}/{{service_id}}`
///
/// When using the `pulumi import` command, Service can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apphub/service:Service default projects/{{project}}/locations/{{location}}/applications/{{application_id}}/services/{{service_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apphub/service:Service default {{project}}/{{location}}/{{application_id}}/{{service_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apphub/service:Service default {{location}}/{{application_id}}/{{service_id}}
/// ```
///
pub mod service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// Part of `parent`.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// Consumer provided attributes.
        /// Structure is documented below.
        #[builder(into, default)]
        pub attributes: pulumi_wasm_rust::Output<
            Option<super::super::types::apphub::ServiceAttributes>,
        >,
        /// User-defined description of a Service.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Immutable. The resource name of the original discovered service.
        #[builder(into)]
        pub discovered_service: pulumi_wasm_rust::Output<String>,
        /// User-defined name for the Service.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Part of `parent`.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The Service identifier.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub service_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// Part of `parent`.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// Consumer provided attributes.
        /// Structure is documented below.
        pub attributes: pulumi_wasm_rust::Output<
            Option<super::super::types::apphub::ServiceAttributes>,
        >,
        /// Output only. Create time.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// User-defined description of a Service.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Immutable. The resource name of the original discovered service.
        pub discovered_service: pulumi_wasm_rust::Output<String>,
        /// User-defined name for the Service.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Part of `parent`.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}
        pub location: pulumi_wasm_rust::Output<String>,
        /// Identifier. The resource name of a Service. Format:
        /// "projects/{host-project-id}/locations/{location}/applications/{application-id}/services/{service-id}"
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The Service identifier.
        ///
        ///
        /// - - -
        pub service_id: pulumi_wasm_rust::Output<String>,
        /// Properties of an underlying cloud resource that can comprise a Service.
        /// Structure is documented below.
        pub service_properties: pulumi_wasm_rust::Output<
            Vec<super::super::types::apphub::ServiceServiceProperty>,
        >,
        /// Reference to an underlying networking resource that can comprise a Service.
        /// Structure is documented below.
        pub service_references: pulumi_wasm_rust::Output<
            Vec<super::super::types::apphub::ServiceServiceReference>,
        >,
        /// Output only. Service state. Possible values: STATE_UNSPECIFIED CREATING ACTIVE DELETING DETACHED
        pub state: pulumi_wasm_rust::Output<String>,
        /// Output only. A universally unique identifier (UUID) for the `Service` in the UUID4
        /// format.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Output only. Update time.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceArgs) -> ServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_inner();
        let attributes_binding = args.attributes.get_inner();
        let description_binding = args.description.get_inner();
        let discovered_service_binding = args.discovered_service.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let service_id_binding = args.service_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apphub/service:Service".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "attributes".into(),
                    value: &attributes_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "discoveredService".into(),
                    value: &discovered_service_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "serviceId".into(),
                    value: &service_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationId".into(),
                },
                register_interface::ResultField {
                    name: "attributes".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "discoveredService".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "serviceId".into(),
                },
                register_interface::ResultField {
                    name: "serviceProperties".into(),
                },
                register_interface::ResultField {
                    name: "serviceReferences".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
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
        ServiceResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationId").unwrap(),
            ),
            attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attributes").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            discovered_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("discoveredService").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceId").unwrap(),
            ),
            service_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceProperties").unwrap(),
            ),
            service_references: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceReferences").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
