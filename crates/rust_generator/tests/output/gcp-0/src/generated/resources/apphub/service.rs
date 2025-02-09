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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// Part of `parent`.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Consumer provided attributes.
        /// Structure is documented below.
        #[builder(into, default)]
        pub attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apphub::ServiceAttributes>,
        >,
        /// User-defined description of a Service.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Immutable. The resource name of the original discovered service.
        #[builder(into)]
        pub discovered_service: pulumi_gestalt_rust::InputOrOutput<String>,
        /// User-defined name for the Service.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Part of `parent`.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Service identifier.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub service_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// Part of `parent`.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// Consumer provided attributes.
        /// Structure is documented below.
        pub attributes: pulumi_gestalt_rust::Output<
            Option<super::super::types::apphub::ServiceAttributes>,
        >,
        /// Output only. Create time.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// User-defined description of a Service.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Immutable. The resource name of the original discovered service.
        pub discovered_service: pulumi_gestalt_rust::Output<String>,
        /// User-defined name for the Service.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Part of `parent`.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Identifier. The resource name of a Service. Format:
        /// "projects/{host-project-id}/locations/{location}/applications/{application-id}/services/{service-id}"
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The Service identifier.
        ///
        ///
        /// - - -
        pub service_id: pulumi_gestalt_rust::Output<String>,
        /// Properties of an underlying cloud resource that can comprise a Service.
        /// Structure is documented below.
        pub service_properties: pulumi_gestalt_rust::Output<
            Vec<super::super::types::apphub::ServiceServiceProperty>,
        >,
        /// Reference to an underlying networking resource that can comprise a Service.
        /// Structure is documented below.
        pub service_references: pulumi_gestalt_rust::Output<
            Vec<super::super::types::apphub::ServiceServiceReference>,
        >,
        /// Output only. Service state. Possible values: STATE_UNSPECIFIED CREATING ACTIVE DELETING DETACHED
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Output only. A universally unique identifier (UUID) for the `Service` in the UUID4
        /// format.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. Update time.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceArgs,
    ) -> ServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_id_binding = args.application_id.get_output(context);
        let attributes_binding = args.attributes.get_output(context);
        let description_binding = args.description.get_output(context);
        let discovered_service_binding = args.discovered_service.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let service_id_binding = args.service_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apphub/service:Service".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationId".into(),
                    value: application_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attributes".into(),
                    value: attributes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "discoveredService".into(),
                    value: discovered_service_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceId".into(),
                    value: service_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceResult {
            application_id: o.get_field("applicationId"),
            attributes: o.get_field("attributes"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            discovered_service: o.get_field("discoveredService"),
            display_name: o.get_field("displayName"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            service_id: o.get_field("serviceId"),
            service_properties: o.get_field("serviceProperties"),
            service_references: o.get_field("serviceReferences"),
            state: o.get_field("state"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
