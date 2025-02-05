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
pub mod thing {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ThingArgs {
        /// Map of attributes of the thing.
        #[builder(into, default)]
        pub attributes: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the thing.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The thing type name.
        #[builder(into, default)]
        pub thing_type_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ThingResult {
        /// The ARN of the thing.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Map of attributes of the thing.
        pub attributes: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The default client ID.
        pub default_client_id: pulumi_wasm_rust::Output<String>,
        /// The name of the thing.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The thing type name.
        pub thing_type_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The current version of the thing record in the registry.
        pub version: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ThingArgs,
    ) -> ThingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attributes_binding = args.attributes.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let thing_type_name_binding = args
            .thing_type_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iot/thing:Thing".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attributes".into(),
                    value: &attributes_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "thingTypeName".into(),
                    value: &thing_type_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ThingResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            attributes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("attributes"),
            ),
            default_client_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultClientId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            thing_type_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("thingTypeName"),
            ),
            version: pulumi_wasm_rust::__private::into_domain(o.extract_field("version")),
        }
    }
}
