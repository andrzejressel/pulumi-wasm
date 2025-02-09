/// Provides a GameLift Alias resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:gamelift:Alias
///     properties:
///       name: example-alias
///       description: Example Description
///       routingStrategy:
///         message: Example Message
///         type: TERMINAL
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import GameLift Aliases using the ID. For example:
///
/// ```sh
/// $ pulumi import aws:gamelift/alias:Alias example <alias-id>
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod alias {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AliasArgs {
        /// Description of the alias.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the alias.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the fleet and/or routing type to use for the alias.
        #[builder(into)]
        pub routing_strategy: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::gamelift::AliasRoutingStrategy,
        >,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AliasResult {
        /// Alias ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the alias.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the alias.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the fleet and/or routing type to use for the alias.
        pub routing_strategy: pulumi_gestalt_rust::Output<
            super::super::types::gamelift::AliasRoutingStrategy,
        >,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: AliasArgs,
    ) -> AliasResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let routing_strategy_binding = args.routing_strategy.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:gamelift/alias:Alias".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routingStrategy".into(),
                    value: routing_strategy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AliasResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            routing_strategy: o.get_field("routingStrategy"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
