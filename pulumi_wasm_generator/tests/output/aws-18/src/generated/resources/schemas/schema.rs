/// Provides an EventBridge Schema resource.
///
/// > **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:schemas:Registry
///     properties:
///       name: my_own_registry
///   testSchema:
///     type: aws:schemas:Schema
///     name: test
///     properties:
///       name: my_schema
///       registryName: ${test.name}
///       type: OpenApi3
///       description: The schema definition for my event
///       content:
///         fn::toJSON:
///           openapi: 3.0.0
///           info:
///             version: 1.0.0
///             title: Event
///           paths: {}
///           components:
///             schemas:
///               Event:
///                 type: object
///                 properties:
///                   name:
///                     type: string
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EventBridge schema using the `name` and `registry_name`. For example:
///
/// ```sh
/// $ pulumi import aws:schemas/schema:Schema test name/registry
/// ```
pub mod schema {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SchemaArgs {
        /// The schema specification. Must be a valid Open API 3.0 spec.
        #[builder(into)]
        pub content: pulumi_wasm_rust::InputOrOutput<String>,
        /// The description of the schema. Maximum of 256 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the schema. Maximum of 385 characters consisting of lower case letters, upper case letters, ., -, _, @.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the registry in which this schema belongs.
        #[builder(into)]
        pub registry_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the schema. Valid values: `OpenApi3` or `JSONSchemaDraft4`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SchemaResult {
        /// The Amazon Resource Name (ARN) of the discoverer.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The schema specification. Must be a valid Open API 3.0 spec.
        pub content: pulumi_wasm_rust::Output<String>,
        /// The description of the schema. Maximum of 256 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The last modified date of the schema.
        pub last_modified: pulumi_wasm_rust::Output<String>,
        /// The name of the schema. Maximum of 385 characters consisting of lower case letters, upper case letters, ., -, _, @.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the registry in which this schema belongs.
        pub registry_name: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of the schema. Valid values: `OpenApi3` or `JSONSchemaDraft4`.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The version of the schema.
        pub version: pulumi_wasm_rust::Output<String>,
        /// The created date of the version of the schema.
        pub version_created_date: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SchemaArgs,
    ) -> SchemaResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let content_binding = args.content.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let registry_name_binding = args.registry_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:schemas/schema:Schema".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "content".into(),
                    value: &content_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "registryName".into(),
                    value: &registry_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "content".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "lastModified".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "registryName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "versionCreatedDate".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SchemaResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("content").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            last_modified: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModified").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            registry_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            version_created_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionCreatedDate").unwrap(),
            ),
        }
    }
}
