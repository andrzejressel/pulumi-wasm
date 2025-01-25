/// An `Environment KeyStore` in Apigee.
///
///
/// To get more information about EnvKeystore, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments.keystores/create)
/// * How-to Guides
///     * [Creating an environment](https://cloud.google.com/apigee/docs/api-platform/get-started/create-environment)
///
/// ## Import
///
/// EnvKeystore can be imported using any of these accepted formats:
///
/// * `{{env_id}}/keystores/{{name}}`
///
/// * `{{env_id}}/{{name}}`
///
/// When using the `pulumi import` command, EnvKeystore can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/envKeystore:EnvKeystore default {{env_id}}/keystores/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/envKeystore:EnvKeystore default {{env_id}}/{{name}}
/// ```
///
pub mod env_keystore {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvKeystoreArgs {
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/environments/{{env_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub env_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the newly created keystore.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EnvKeystoreResult {
        /// Aliases in this keystore.
        pub aliases: pulumi_wasm_rust::Output<Vec<String>>,
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/environments/{{env_name}}`.
        ///
        ///
        /// - - -
        pub env_id: pulumi_wasm_rust::Output<String>,
        /// The name of the newly created keystore.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EnvKeystoreArgs,
    ) -> EnvKeystoreResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let env_id_binding = args.env_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigee/envKeystore:EnvKeystore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "envId".into(),
                    value: &env_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "aliases".into(),
                },
                register_interface::ResultField {
                    name: "envId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EnvKeystoreResult {
            aliases: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aliases").unwrap(),
            ),
            env_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("envId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
