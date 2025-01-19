/// An `Environment Reference` in Apigee.
///
///
/// To get more information about EnvReferences, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments.references/create)
/// * How-to Guides
///     * [Creating an environment](https://cloud.google.com/apigee/docs/api-platform/get-started/create-environment)
///
/// ## Import
///
/// EnvReferences can be imported using any of these accepted formats:
///
/// * `{{env_id}}/references/{{name}}`
///
/// * `{{env_id}}/{{name}}`
///
/// When using the `pulumi import` command, EnvReferences can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/envReferences:EnvReferences default {{env_id}}/references/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/envReferences:EnvReferences default {{env_id}}/{{name}}
/// ```
///
pub mod env_references {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvReferencesArgs {
        /// Optional. A human-readable description of this reference.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/environments/{{env_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub env_id: pulumi_wasm_rust::Output<String>,
        /// Required. The resource id of this reference. Values must match the regular expression [\w\s-.]+.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Required. The id of the resource to which this reference refers. Must be the id of a resource that exists in the parent environment and is of the given resourceType.
        #[builder(into)]
        pub refers: pulumi_wasm_rust::Output<String>,
        /// The type of resource referred to by this reference. Valid values are 'KeyStore' or 'TrustStore'.
        #[builder(into)]
        pub resource_type: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct EnvReferencesResult {
        /// Optional. A human-readable description of this reference.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/environments/{{env_name}}`.
        ///
        ///
        /// - - -
        pub env_id: pulumi_wasm_rust::Output<String>,
        /// Required. The resource id of this reference. Values must match the regular expression [\w\s-.]+.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Required. The id of the resource to which this reference refers. Must be the id of a resource that exists in the parent environment and is of the given resourceType.
        pub refers: pulumi_wasm_rust::Output<String>,
        /// The type of resource referred to by this reference. Valid values are 'KeyStore' or 'TrustStore'.
        pub resource_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EnvReferencesArgs) -> EnvReferencesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let env_id_binding = args.env_id.get_inner();
        let name_binding = args.name.get_inner();
        let refers_binding = args.refers.get_inner();
        let resource_type_binding = args.resource_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigee/envReferences:EnvReferences".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "envId".into(),
                    value: &env_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "refers".into(),
                    value: &refers_binding,
                },
                register_interface::ObjectField {
                    name: "resourceType".into(),
                    value: &resource_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "envId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "refers".into(),
                },
                register_interface::ResultField {
                    name: "resourceType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EnvReferencesResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            env_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("envId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            refers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("refers").unwrap(),
            ),
            resource_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceType").unwrap(),
            ),
        }
    }
}
