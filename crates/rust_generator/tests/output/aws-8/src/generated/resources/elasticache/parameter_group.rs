/// Provides an ElastiCache parameter group resource.
///
/// > **NOTE:** Attempting to remove the `reserved-memory` parameter when `family` is set to `redis2.6` or `redis2.8` may show a perpetual difference in this provider due to an ElastiCache API limitation. Leave that parameter configured with any value to workaround the issue.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = parameter_group::create(
///         "default",
///         ParameterGroupArgs::builder()
///             .family("redis2.8")
///             .name("cache-params")
///             .parameters(
///                 vec![
///                     ParameterGroupParameter::builder().name("activerehashing")
///                     .value("yes").build_struct(), ParameterGroupParameter::builder()
///                     .name("min-slaves-to-write").value("2").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ElastiCache Parameter Groups using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:elasticache/parameterGroup:ParameterGroup default redis-params
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod parameter_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ParameterGroupArgs {
        /// The description of the ElastiCache parameter group. Defaults to "Managed by Pulumi".
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The family of the ElastiCache parameter group.
        #[builder(into)]
        pub family: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the ElastiCache parameter.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of ElastiCache parameters to apply.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::elasticache::ParameterGroupParameter>>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ParameterGroupResult {
        /// The AWS ARN associated with the parameter group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The description of the ElastiCache parameter group. Defaults to "Managed by Pulumi".
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The family of the ElastiCache parameter group.
        pub family: pulumi_gestalt_rust::Output<String>,
        /// The name of the ElastiCache parameter.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of ElastiCache parameters to apply.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::elasticache::ParameterGroupParameter>>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
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
        args: ParameterGroupArgs,
    ) -> ParameterGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let family_binding = args.family.get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:elasticache/parameterGroup:ParameterGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "family".into(),
                    value: family_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ParameterGroupResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            family: o.get_field("family"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
