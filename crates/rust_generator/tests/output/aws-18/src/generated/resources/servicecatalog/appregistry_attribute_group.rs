/// Resource for managing an AWS Service Catalog AppRegistry Attribute Group.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:servicecatalog:AppregistryAttributeGroup
///     properties:
///       name: example
///       description: example description
///       attributes:
///         fn::toJSON:
///           app: exampleapp
///           group: examplegroup
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Service Catalog AppRegistry Attribute Group using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:servicecatalog/appregistryAttributeGroup:AppregistryAttributeGroup example 1234567890abcfedhijk09876s
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod appregistry_attribute_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppregistryAttributeGroupArgs {
        /// A JSON string of nested key-value pairs that represents the attributes of the group.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub attributes: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the Attribute Group.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the Attribute Group.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags assigned to the Attribute Group. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AppregistryAttributeGroupResult {
        /// ARN of the Attribute Group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A JSON string of nested key-value pairs that represents the attributes of the group.
        ///
        /// The following arguments are optional:
        pub attributes: pulumi_gestalt_rust::Output<String>,
        /// Description of the Attribute Group.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the Attribute Group.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags assigned to the Attribute Group. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: AppregistryAttributeGroupArgs,
    ) -> AppregistryAttributeGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let attributes_binding = args.attributes.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:servicecatalog/appregistryAttributeGroup:AppregistryAttributeGroup"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attributes".into(),
                    value: attributes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AppregistryAttributeGroupResult {
            arn: o.get_field("arn"),
            attributes: o.get_field("attributes"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
