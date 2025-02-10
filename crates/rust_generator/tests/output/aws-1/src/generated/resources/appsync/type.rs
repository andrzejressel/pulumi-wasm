/// Provides an AppSync Type.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:appsync:GraphQLApi
///     properties:
///       authenticationType: API_KEY
///       name: example
///   exampleType:
///     type: aws:appsync:Type
///     name: example
///     properties:
///       apiId: ${example.id}
///       format: SDL
///       definition: |
///         type Mutation
///
///         {
///         putPost(id: ID!,title: String! ): Post
///
///         }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Appsync Types using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:appsync/type:Type example api-id:format:name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod type_ {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TypeArgs {
        /// GraphQL API ID.
        #[builder(into)]
        pub api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type definition.
        #[builder(into)]
        pub definition: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type format: `SDL` or `JSON`.
        #[builder(into)]
        pub format: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TypeResult {
        /// GraphQL API ID.
        pub api_id: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the type.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The type definition.
        pub definition: pulumi_gestalt_rust::Output<String>,
        /// The type description.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The type format: `SDL` or `JSON`.
        pub format: pulumi_gestalt_rust::Output<String>,
        /// The type name.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TypeArgs,
    ) -> TypeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_id_binding = args.api_id.get_output(context);
        let definition_binding = args.definition.get_output(context);
        let format_binding = args.format.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appsync/type:Type".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiId".into(),
                    value: api_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "definition".into(),
                    value: definition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "format".into(),
                    value: format_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TypeResult {
            api_id: o.get_field("apiId"),
            arn: o.get_field("arn"),
            definition: o.get_field("definition"),
            description: o.get_field("description"),
            format: o.get_field("format"),
            name: o.get_field("name"),
        }
    }
}
