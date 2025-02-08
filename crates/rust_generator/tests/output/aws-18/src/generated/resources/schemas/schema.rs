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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod schema {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SchemaArgs {
        /// The schema specification. Must be a valid Open API 3.0 spec.
        #[builder(into)]
        pub content: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the schema. Maximum of 256 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the schema. Maximum of 385 characters consisting of lower case letters, upper case letters, ., -, _, @.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the registry in which this schema belongs.
        #[builder(into)]
        pub registry_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the schema. Valid values: `OpenApi3` or `JSONSchemaDraft4`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SchemaResult {
        /// The Amazon Resource Name (ARN) of the discoverer.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The schema specification. Must be a valid Open API 3.0 spec.
        pub content: pulumi_gestalt_rust::Output<String>,
        /// The description of the schema. Maximum of 256 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The last modified date of the schema.
        pub last_modified: pulumi_gestalt_rust::Output<String>,
        /// The name of the schema. Maximum of 385 characters consisting of lower case letters, upper case letters, ., -, _, @.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the registry in which this schema belongs.
        pub registry_name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of the schema. Valid values: `OpenApi3` or `JSONSchemaDraft4`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The version of the schema.
        pub version: pulumi_gestalt_rust::Output<String>,
        /// The created date of the version of the schema.
        pub version_created_date: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SchemaArgs,
    ) -> SchemaResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        SchemaResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("content"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            last_modified: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastModified"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            registry_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("registryName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            version_created_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionCreatedDate"),
            ),
        }
    }
}
