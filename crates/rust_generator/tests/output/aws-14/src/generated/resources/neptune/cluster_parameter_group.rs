/// Manages a Neptune Cluster Parameter Group
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:neptune:ClusterParameterGroup
///     properties:
///       family: neptune1
///       name: example
///       description: neptune cluster parameter group
///       parameters:
///         - name: neptune_enable_audit_log
///           value: 1
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Neptune Cluster Parameter Groups using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:neptune/clusterParameterGroup:ClusterParameterGroup cluster_pg production-pg-1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster_parameter_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterParameterGroupArgs {
        /// The description of the neptune cluster parameter group. Defaults to "Managed by Pulumi".
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The family of the neptune cluster parameter group.
        #[builder(into)]
        pub family: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the neptune parameter.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of neptune parameters to apply.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::neptune::ClusterParameterGroupParameter>>,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterParameterGroupResult {
        /// The ARN of the neptune cluster parameter group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The description of the neptune cluster parameter group. Defaults to "Managed by Pulumi".
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The family of the neptune cluster parameter group.
        pub family: pulumi_gestalt_rust::Output<String>,
        /// The name of the neptune parameter.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// A list of neptune parameters to apply.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::neptune::ClusterParameterGroupParameter>>,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterParameterGroupArgs,
    ) -> ClusterParameterGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let family_binding = args.family.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:neptune/clusterParameterGroup:ClusterParameterGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "family".into(),
                    value: &family_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterParameterGroupResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            family: o.get_field("family"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            parameters: o.get_field("parameters"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
