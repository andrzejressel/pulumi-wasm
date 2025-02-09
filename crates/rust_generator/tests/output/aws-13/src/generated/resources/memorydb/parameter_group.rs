/// Provides a MemoryDB Parameter Group.
///
/// More information about parameter groups can be found in the [MemoryDB User Guide](https://docs.aws.amazon.com/memorydb/latest/devguide/parametergroups.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = parameter_group::create(
///         "example",
///         ParameterGroupArgs::builder()
///             .family("memorydb_redis6")
///             .name("my-parameter-group")
///             .parameters(
///                 vec![
///                     ParameterGroupParameter::builder().name("activedefrag").value("yes")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a parameter group using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:memorydb/parameterGroup:ParameterGroup example my-parameter-group
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod parameter_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ParameterGroupArgs {
        /// Description for the parameter group. Defaults to `"Managed by Pulumi"`.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The engine version that the parameter group can be used with.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub family: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the parameter group. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of MemoryDB parameters to apply. Any parameters not specified will fall back to their family defaults. Detailed below.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::memorydb::ParameterGroupParameter>>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ParameterGroupResult {
        /// The ARN of the parameter group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description for the parameter group. Defaults to `"Managed by Pulumi"`.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The engine version that the parameter group can be used with.
        ///
        /// The following arguments are optional:
        pub family: pulumi_gestalt_rust::Output<String>,
        /// Name of the parameter group. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// Set of MemoryDB parameters to apply. Any parameters not specified will fall back to their family defaults. Detailed below.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::memorydb::ParameterGroupParameter>>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: ParameterGroupArgs,
    ) -> ParameterGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let family_binding_1 = args.family.get_output(context);
        let family_binding = family_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let name_prefix_binding_1 = args.name_prefix.get_output(context);
        let name_prefix_binding = name_prefix_binding_1.get_inner();
        let parameters_binding_1 = args.parameters.get_output(context);
        let parameters_binding = parameters_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:memorydb/parameterGroup:ParameterGroup".into(),
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
        ParameterGroupResult {
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
