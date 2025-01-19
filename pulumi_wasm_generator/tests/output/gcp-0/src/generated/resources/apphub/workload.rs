/// Workload represents a binary deployment (such as Managed Instance Groups (MIGs), GKE deployments, etc.) that performs the smallest logical subset of business functionality. It registers identified workload to the Application.
///
///
///
/// ## Example Usage
///
/// ## Import
///
/// Workload can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/applications/{{application_id}}/workloads/{{workload_id}}`
///
/// * `{{project}}/{{location}}/{{application_id}}/{{workload_id}}`
///
/// * `{{location}}/{{application_id}}/{{workload_id}}`
///
/// When using the `pulumi import` command, Workload can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apphub/workload:Workload default projects/{{project}}/locations/{{location}}/applications/{{application_id}}/workloads/{{workload_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apphub/workload:Workload default {{project}}/{{location}}/{{application_id}}/{{workload_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apphub/workload:Workload default {{location}}/{{application_id}}/{{workload_id}}
/// ```
///
pub mod workload {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkloadArgs {
        /// Part of `parent`.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// Consumer provided attributes.
        /// Structure is documented below.
        #[builder(into, default)]
        pub attributes: pulumi_wasm_rust::Output<
            Option<super::super::types::apphub::WorkloadAttributes>,
        >,
        /// User-defined description of a Workload.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Immutable. The resource name of the original discovered workload.
        #[builder(into)]
        pub discovered_workload: pulumi_wasm_rust::Output<String>,
        /// User-defined name for the Workload.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Part of `parent`.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The Workload identifier.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub workload_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WorkloadResult {
        /// Part of `parent`.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// Consumer provided attributes.
        /// Structure is documented below.
        pub attributes: pulumi_wasm_rust::Output<
            Option<super::super::types::apphub::WorkloadAttributes>,
        >,
        /// Output only. Create time.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// User-defined description of a Workload.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Immutable. The resource name of the original discovered workload.
        pub discovered_workload: pulumi_wasm_rust::Output<String>,
        /// User-defined name for the Workload.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Part of `parent`.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}
        pub location: pulumi_wasm_rust::Output<String>,
        /// Identifier. The resource name of the Workload. Format:"projects/{host-project-id}/locations/{location}/applications/{application-id}/workloads/{workload-id}"
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Output only. Workload state. Possible values:  STATE_UNSPECIFIED CREATING ACTIVE DELETING DETACHED
        pub state: pulumi_wasm_rust::Output<String>,
        /// Output only. A universally unique identifier (UUID) for the `Workload` in the UUID4 format.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Output only. Update time.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// The Workload identifier.
        ///
        ///
        /// - - -
        pub workload_id: pulumi_wasm_rust::Output<String>,
        /// Properties of an underlying compute resource represented by the Workload.
        /// Structure is documented below.
        pub workload_properties: pulumi_wasm_rust::Output<
            Vec<super::super::types::apphub::WorkloadWorkloadProperty>,
        >,
        /// Reference of an underlying compute resource represented by the Workload.
        /// Structure is documented below.
        pub workload_references: pulumi_wasm_rust::Output<
            Vec<super::super::types::apphub::WorkloadWorkloadReference>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WorkloadArgs) -> WorkloadResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_inner();
        let attributes_binding = args.attributes.get_inner();
        let description_binding = args.description.get_inner();
        let discovered_workload_binding = args.discovered_workload.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let workload_id_binding = args.workload_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apphub/workload:Workload".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "discoveredWorkload".into(),
                    value: &discovered_workload_binding,
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
                    name: "workloadId".into(),
                    value: &workload_id_binding,
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
                    name: "discoveredWorkload".into(),
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
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "workloadId".into(),
                },
                register_interface::ResultField {
                    name: "workloadProperties".into(),
                },
                register_interface::ResultField {
                    name: "workloadReferences".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkloadResult {
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
            discovered_workload: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("discoveredWorkload").unwrap(),
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
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            workload_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workloadId").unwrap(),
            ),
            workload_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workloadProperties").unwrap(),
            ),
            workload_references: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workloadReferences").unwrap(),
            ),
        }
    }
}
