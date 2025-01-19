/// ## Example Usage
///
/// ### Artifact Registry Vpcsc Config
///
///
/// ```yaml
/// resources:
///   my-config:
///     type: gcp:artifactregistry:VpcscConfig
///     properties:
///       location: us-central1
///       vpcscPolicy: ALLOW
/// ```
///
/// ## Import
///
/// VPCSCConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/vpcscConfig/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, VPCSCConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:artifactregistry/vpcscConfig:VpcscConfig default projects/{{project}}/locations/{{location}}/vpcscConfig/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:artifactregistry/vpcscConfig:VpcscConfig default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:artifactregistry/vpcscConfig:VpcscConfig default {{location}}/{{name}}
/// ```
///
pub mod vpcsc_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcscConfigArgs {
        /// The name of the location this config is located in.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The VPC SC policy for project and location.
        /// Possible values are: `DENY`, `ALLOW`.
        #[builder(into, default)]
        pub vpcsc_policy: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VpcscConfigResult {
        /// The name of the location this config is located in.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the project's VPC SC Config.
        /// Always of the form: projects/{project}/location/{location}/vpcscConfig
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The VPC SC policy for project and location.
        /// Possible values are: `DENY`, `ALLOW`.
        pub vpcsc_policy: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VpcscConfigArgs) -> VpcscConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let vpcsc_policy_binding = args.vpcsc_policy.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:artifactregistry/vpcscConfig:VpcscConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "vpcscPolicy".into(),
                    value: &vpcsc_policy_binding,
                },
            ]),
            results: Vec::from([
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
                    name: "vpcscPolicy".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcscConfigResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            vpcsc_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcscPolicy").unwrap(),
            ),
        }
    }
}
