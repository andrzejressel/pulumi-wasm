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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod env_references {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvReferencesArgs {
        /// Optional. A human-readable description of this reference.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/environments/{{env_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub env_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Required. The resource id of this reference. Values must match the regular expression [\w\s-.]+.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. The id of the resource to which this reference refers. Must be the id of a resource that exists in the parent environment and is of the given resourceType.
        #[builder(into)]
        pub refers: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of resource referred to by this reference. Valid values are 'KeyStore' or 'TrustStore'.
        #[builder(into)]
        pub resource_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EnvReferencesResult {
        /// Optional. A human-readable description of this reference.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/environments/{{env_name}}`.
        ///
        ///
        /// - - -
        pub env_id: pulumi_gestalt_rust::Output<String>,
        /// Required. The resource id of this reference. Values must match the regular expression [\w\s-.]+.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Required. The id of the resource to which this reference refers. Must be the id of a resource that exists in the parent environment and is of the given resourceType.
        pub refers: pulumi_gestalt_rust::Output<String>,
        /// The type of resource referred to by this reference. Valid values are 'KeyStore' or 'TrustStore'.
        pub resource_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvReferencesArgs,
    ) -> EnvReferencesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let env_id_binding = args.env_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let refers_binding = args.refers.get_output(context);
        let resource_type_binding = args.resource_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigee/envReferences:EnvReferences".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "envId".into(),
                    value: &env_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "refers".into(),
                    value: &refers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceType".into(),
                    value: &resource_type_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnvReferencesResult {
            description: o.get_field("description"),
            env_id: o.get_field("envId"),
            name: o.get_field("name"),
            refers: o.get_field("refers"),
            resource_type: o.get_field("resourceType"),
        }
    }
}
