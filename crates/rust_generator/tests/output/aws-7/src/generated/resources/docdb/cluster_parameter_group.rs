/// Manages a DocumentDB Cluster Parameter Group
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster_parameter_group::create(
///         "example",
///         ClusterParameterGroupArgs::builder()
///             .description("docdb cluster parameter group")
///             .family("docdb3.6")
///             .name("example")
///             .parameters(
///                 vec![
///                     ClusterParameterGroupParameter::builder().name("tls")
///                     .value("enabled").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DocumentDB Cluster Parameter Groups using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:docdb/clusterParameterGroup:ClusterParameterGroup cluster_pg production-pg-1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster_parameter_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterParameterGroupArgs {
        /// The description of the DocumentDB cluster parameter group. Defaults to "Managed by Pulumi".
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The family of the DocumentDB cluster parameter group.
        #[builder(into)]
        pub family: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the DocumentDB parameter.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of DocumentDB parameters to apply. Setting parameters to system default values may show a difference on imported resources.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::docdb::ClusterParameterGroupParameter>>,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterParameterGroupResult {
        /// The ARN of the DocumentDB cluster parameter group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The description of the DocumentDB cluster parameter group. Defaults to "Managed by Pulumi".
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The family of the DocumentDB cluster parameter group.
        pub family: pulumi_gestalt_rust::Output<String>,
        /// The name of the DocumentDB parameter.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// A list of DocumentDB parameters to apply. Setting parameters to system default values may show a difference on imported resources.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::docdb::ClusterParameterGroupParameter>>,
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ClusterParameterGroupArgs,
    ) -> ClusterParameterGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let family_binding = args.family.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let name_prefix_binding = args.name_prefix.get_output(context).get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:docdb/clusterParameterGroup:ClusterParameterGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "family".into(),
                    value: &family_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClusterParameterGroupResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            family: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("family"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            name_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namePrefix"),
            ),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
