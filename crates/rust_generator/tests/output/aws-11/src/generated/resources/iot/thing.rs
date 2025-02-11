/// Creates and manages an AWS IoT Thing.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iot:Thing
///     properties:
///       name: example
///       attributes:
///         First: examplevalue
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IOT Things using the name. For example:
///
/// ```sh
/// $ pulumi import aws:iot/thing:Thing example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod thing {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ThingArgs {
        /// Map of attributes of the thing.
        #[builder(into, default)]
        pub attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the thing.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The thing type name.
        #[builder(into, default)]
        pub thing_type_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ThingResult {
        /// The ARN of the thing.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Map of attributes of the thing.
        pub attributes: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The default client ID.
        pub default_client_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the thing.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The thing type name.
        pub thing_type_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The current version of the thing record in the registry.
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ThingArgs,
    ) -> ThingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let attributes_binding = args.attributes.get_output(context);
        let name_binding = args.name.get_output(context);
        let thing_type_name_binding = args.thing_type_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iot/thing:Thing".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attributes".into(),
                    value: &attributes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "thingTypeName".into(),
                    value: &thing_type_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ThingResult {
            arn: o.get_field("arn"),
            attributes: o.get_field("attributes"),
            default_client_id: o.get_field("defaultClientId"),
            name: o.get_field("name"),
            thing_type_name: o.get_field("thingTypeName"),
            version: o.get_field("version"),
        }
    }
}
