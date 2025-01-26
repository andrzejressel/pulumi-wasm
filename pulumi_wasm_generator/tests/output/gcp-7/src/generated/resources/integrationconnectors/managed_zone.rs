/// An Integration connectors Managed Zone.
///
///
/// To get more information about ManagedZone, see:
///
/// * [API documentation](https://cloud.google.com/integration-connectors/docs/reference/rest/v1/projects.locations.global.managedZones)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/integration-connectors/docs)
///
/// ## Example Usage
///
/// ### Integration Connectors Managed Zone
///
///
/// ```yaml
/// resources:
///   targetProject:
///     type: gcp:organizations:Project
///     name: target_project
///     properties:
///       projectId: tf-test_40785
///       name: tf-test_79169
///       orgId: '123456789'
///       billingAccount: 000000-0000000-0000000-000000
///       deletionPolicy: DELETE
///   dnsPeerBinding:
///     type: gcp:projects:IAMMember
///     name: dns_peer_binding
///     properties:
///       project: ${targetProject.projectId}
///       role: roles/dns.peer
///       member: serviceAccount:service-${testProject.number}@gcp-sa-connectors.iam.gserviceaccount.com
///   dns:
///     type: gcp:projects:Service
///     properties:
///       project: ${targetProject.projectId}
///       service: dns.googleapis.com
///   compute:
///     type: gcp:projects:Service
///     properties:
///       project: ${targetProject.projectId}
///       service: compute.googleapis.com
///   network:
///     type: gcp:compute:Network
///     properties:
///       project: ${targetProject.projectId}
///       name: test
///       autoCreateSubnetworks: false
///     options:
///       dependsOn:
///         - ${compute}
///   zone:
///     type: gcp:dns:ManagedZone
///     properties:
///       name: tf-test-dns_56529
///       dnsName: private_75413.example.com.
///       visibility: private
///       privateVisibilityConfig:
///         networks:
///           - networkUrl: ${network.id}
///     options:
///       dependsOn:
///         - ${dns}
///   testmanagedzone:
///     type: gcp:integrationconnectors:ManagedZone
///     properties:
///       name: test
///       description: tf created description
///       labels:
///         intent: example
///       targetProject: ${targetProject.projectId}
///       targetVpc: test
///       dns: ${zone.dnsName}
///     options:
///       dependsOn:
///         - ${dnsPeerBinding}
///         - ${zone}
/// variables:
///   testProject:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// ManagedZone can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/managedZones/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, ManagedZone can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:integrationconnectors/managedZone:ManagedZone default projects/{{project}}/locations/global/managedZones/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:integrationconnectors/managedZone:ManagedZone default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:integrationconnectors/managedZone:ManagedZone default {{name}}
/// ```
///
pub mod managed_zone {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedZoneArgs {
        /// Description of the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// DNS Name of the resource.
        #[builder(into)]
        pub dns: pulumi_wasm_rust::InputOrOutput<String>,
        /// Resource labels to represent user provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of Managed Zone needs to be created.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Target Project.
        #[builder(into)]
        pub target_project: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Target Project VPC Network.
        #[builder(into)]
        pub target_vpc: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedZoneResult {
        /// Time the Namespace was created in UTC.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Description of the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// DNS Name of the resource.
        pub dns: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Resource labels to represent user provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of Managed Zone needs to be created.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the Target Project.
        pub target_project: pulumi_wasm_rust::Output<String>,
        /// The name of the Target Project VPC Network.
        pub target_vpc: pulumi_wasm_rust::Output<String>,
        /// Time the Namespace was updated in UTC.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ManagedZoneArgs,
    ) -> ManagedZoneResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let dns_binding = args.dns.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let target_project_binding = args.target_project.get_output(context).get_inner();
        let target_vpc_binding = args.target_vpc.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:integrationconnectors/managedZone:ManagedZone".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "dns".into(),
                    value: &dns_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "targetProject".into(),
                    value: &target_project_binding,
                },
                register_interface::ObjectField {
                    name: "targetVpc".into(),
                    value: &target_vpc_binding,
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
                    name: "dns".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
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
                    name: "targetProject".into(),
                },
                register_interface::ResultField {
                    name: "targetVpc".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ManagedZoneResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            dns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dns").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
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
            target_project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetProject").unwrap(),
            ),
            target_vpc: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetVpc").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
