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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod env_keystore {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvKeystoreArgs {
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/environments/{{env_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub env_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the newly created keystore.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EnvKeystoreResult {
        /// Aliases in this keystore.
        pub aliases: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/environments/{{env_name}}`.
        ///
        ///
        /// - - -
        pub env_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the newly created keystore.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EnvKeystoreArgs,
    ) -> EnvKeystoreResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        EnvKeystoreResult {
            aliases: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("aliases"),
            ),
            env_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("envId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
