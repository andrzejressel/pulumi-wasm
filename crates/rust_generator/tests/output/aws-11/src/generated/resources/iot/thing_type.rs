/// Creates and manages an AWS IoT Thing Type.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = thing_type::create(
///         "foo",
///         ThingTypeArgs::builder().name("my_iot_thing").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IOT Thing Types using the name. For example:
///
/// ```sh
/// $ pulumi import aws:iot/thingType:ThingType example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod thing_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ThingTypeArgs {
        /// Whether the thing type is deprecated. If true, no new things could be associated with this type.
        #[builder(into, default)]
        pub deprecated: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the thing type.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// , Configuration block that can contain the following properties of the thing type:
        #[builder(into, default)]
        pub properties: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::iot::ThingTypeProperties>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ThingTypeResult {
        /// The ARN of the created AWS IoT Thing Type.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether the thing type is deprecated. If true, no new things could be associated with this type.
        pub deprecated: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the thing type.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// , Configuration block that can contain the following properties of the thing type:
        pub properties: pulumi_gestalt_rust::Output<
            Option<super::super::types::iot::ThingTypeProperties>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: ThingTypeArgs,
    ) -> ThingTypeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let deprecated_binding = args.deprecated.get_output(context);
        let name_binding = args.name.get_output(context);
        let properties_binding = args.properties.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iot/thingType:ThingType".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deprecated".into(),
                    value: deprecated_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "properties".into(),
                    value: properties_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ThingTypeResult {
            arn: o.get_field("arn"),
            deprecated: o.get_field("deprecated"),
            name: o.get_field("name"),
            properties: o.get_field("properties"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
