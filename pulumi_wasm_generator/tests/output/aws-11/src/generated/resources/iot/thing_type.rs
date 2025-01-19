/// Creates and manages an AWS IoT Thing Type.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod thing_type {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ThingTypeArgs {
        /// Whether the thing type is deprecated. If true, no new things could be associated with this type.
        #[builder(into, default)]
        pub deprecated: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the thing type.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// , Configuration block that can contain the following properties of the thing type:
        #[builder(into, default)]
        pub properties: pulumi_wasm_rust::Output<
            Option<super::super::types::iot::ThingTypeProperties>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ThingTypeResult {
        /// The ARN of the created AWS IoT Thing Type.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether the thing type is deprecated. If true, no new things could be associated with this type.
        pub deprecated: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the thing type.
        pub name: pulumi_wasm_rust::Output<String>,
        /// , Configuration block that can contain the following properties of the thing type:
        pub properties: pulumi_wasm_rust::Output<
            Option<super::super::types::iot::ThingTypeProperties>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ThingTypeArgs) -> ThingTypeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let deprecated_binding = args.deprecated.get_inner();
        let name_binding = args.name.get_inner();
        let properties_binding = args.properties.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iot/thingType:ThingType".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deprecated".into(),
                    value: &deprecated_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "properties".into(),
                    value: &properties_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "deprecated".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "properties".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ThingTypeResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            deprecated: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deprecated").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("properties").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
