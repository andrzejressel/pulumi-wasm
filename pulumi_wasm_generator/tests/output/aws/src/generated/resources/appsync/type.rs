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
pub mod type_ {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TypeArgs {
        /// GraphQL API ID.
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// The type definition.
        #[builder(into)]
        pub definition: pulumi_wasm_rust::Output<String>,
        /// The type format: `SDL` or `JSON`.
        #[builder(into)]
        pub format: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TypeResult {
        /// GraphQL API ID.
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// The ARN of the type.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The type definition.
        pub definition: pulumi_wasm_rust::Output<String>,
        /// The type description.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The type format: `SDL` or `JSON`.
        pub format: pulumi_wasm_rust::Output<String>,
        /// The type name.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TypeArgs) -> TypeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_inner();
        let definition_binding = args.definition.get_inner();
        let format_binding = args.format.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appsync/type:Type".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "definition".into(),
                    value: &definition_binding,
                },
                register_interface::ObjectField {
                    name: "format".into(),
                    value: &format_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "definition".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "format".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TypeResult {
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            definition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("definition").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            format: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("format").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
