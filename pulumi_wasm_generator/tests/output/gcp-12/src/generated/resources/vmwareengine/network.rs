/// Provides connectivity for VMware Engine private clouds.
///
///
/// To get more information about Network, see:
///
/// * [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.vmwareEngineNetworks)
///
/// ## Example Usage
///
/// ### Vmware Engine Network Standard
///
///
/// ```yaml
/// resources:
///   vmw-engine-network:
///     type: gcp:vmwareengine:Network
///     properties:
///       name: standard-nw
///       location: global
///       type: STANDARD
///       description: VMwareEngine standard network sample
/// ```
/// ### Vmware Engine Network Legacy
///
///
/// ```yaml
/// resources:
///   vmw-engine-network:
///     type: gcp:vmwareengine:Network
///     properties:
///       project: ${acceptance.project}
///       name: us-west1-default
///       location: us-west1
///       type: LEGACY
///       description: VMwareEngine legacy network sample
///   acceptance:
///     type: gcp:projects:Service
///     properties:
///       project: ${acceptanceProject.projectId}
///       service: vmwareengine.googleapis.com
///     options:
///       dependsOn:
///         - ${wait60Seconds}
///   # there can be only 1 Legacy network per region for a given project,
///   # so creating new project for isolation in CI.
///   acceptanceProject:
///     type: gcp:organizations:Project
///     name: acceptance
///     properties:
///       name: vmw-proj
///       projectId: vmw-proj
///       orgId: '123456789'
///       billingAccount: 000000-0000000-0000000-000000
///       deletionPolicy: DELETE
///   wait60Seconds:
///     type: time:sleep
///     name: wait_60_seconds
///     properties:
///       createDuration: 60s
///     options:
///       dependsOn:
///         - ${acceptanceProject}
/// ```
///
/// ## Import
///
/// Network can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/vmwareEngineNetworks/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Network can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/network:Network default projects/{{project}}/locations/{{location}}/vmwareEngineNetworks/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/network:Network default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/network:Network default {{location}}/{{name}}
/// ```
///
pub mod network {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkArgs {
        /// User-provided description for this VMware Engine network.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The location where the VMwareEngineNetwork should reside.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the VMwareEngineNetwork.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// VMware Engine network type.
        /// Possible values are: `LEGACY`, `STANDARD`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkResult {
        /// User-provided description for this VMware Engine network.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The location where the VMwareEngineNetwork should reside.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the VMwareEngineNetwork.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// State of the VMware Engine network.
        pub state: pulumi_wasm_rust::Output<String>,
        /// VMware Engine network type.
        /// Possible values are: `LEGACY`, `STANDARD`.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// System-generated unique identifier for the resource.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// VMware Engine service VPC networks that provide connectivity from a private cloud to customer projects,
        /// the internet, and other Google Cloud services.
        /// Structure is documented below.
        pub vpc_networks: pulumi_wasm_rust::Output<
            Vec<super::super::types::vmwareengine::NetworkVpcNetwork>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NetworkArgs) -> NetworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vmwareengine/network:Network".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
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
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "vpcNetworks".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
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
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            vpc_networks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcNetworks").unwrap(),
            ),
        }
    }
}
