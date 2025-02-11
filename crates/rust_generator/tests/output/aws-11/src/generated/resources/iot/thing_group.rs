/// Manages an AWS IoT Thing Group.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   parent:
///     type: aws:iot:ThingGroup
///     properties:
///       name: parent
///   example:
///     type: aws:iot:ThingGroup
///     properties:
///       name: example
///       parentGroupName: ${parent.name}
///       properties:
///         attributePayload:
///           attributes:
///             One: '11111'
///             Two: TwoTwo
///         description: This is my thing group
///       tags:
///         managed: 'true'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IoT Things Groups using the name. For example:
///
/// ```sh
/// $ pulumi import aws:iot/thingGroup:ThingGroup example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod thing_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ThingGroupArgs {
        /// The name of the Thing Group.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the parent Thing Group.
        #[builder(into, default)]
        pub parent_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Thing Group properties. Defined below.
        #[builder(into, default)]
        pub properties: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::iot::ThingGroupProperties>,
        >,
        /// Key-value mapping of resource tags
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ThingGroupResult {
        /// The ARN of the Thing Group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub metadatas: pulumi_gestalt_rust::Output<
            Vec<super::super::types::iot::ThingGroupMetadata>,
        >,
        /// The name of the Thing Group.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the parent Thing Group.
        pub parent_group_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Thing Group properties. Defined below.
        pub properties: pulumi_gestalt_rust::Output<
            Option<super::super::types::iot::ThingGroupProperties>,
        >,
        /// Key-value mapping of resource tags
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The current version of the Thing Group record in the registry.
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ThingGroupArgs,
    ) -> ThingGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let parent_group_name_binding = args.parent_group_name.get_output(context);
        let properties_binding = args.properties.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iot/thingGroup:ThingGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentGroupName".into(),
                    value: &parent_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "properties".into(),
                    value: &properties_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ThingGroupResult {
            arn: o.get_field("arn"),
            metadatas: o.get_field("metadatas"),
            name: o.get_field("name"),
            parent_group_name: o.get_field("parentGroupName"),
            properties: o.get_field("properties"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            version: o.get_field("version"),
        }
    }
}
