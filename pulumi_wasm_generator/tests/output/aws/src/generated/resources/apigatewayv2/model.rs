/// Manages an Amazon API Gateway Version 2 [model](https://docs.aws.amazon.com/apigateway/latest/developerguide/models-mappings.html#models-mappings-models).
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   example:
///     type: aws:apigatewayv2:Model
///     properties:
///       apiId: ${exampleAwsApigatewayv2Api.id}
///       contentType: application/json
///       name: example
///       schema:
///         fn::toJSON:
///           $schema: http://json-schema.org/draft-04/schema#
///           title: ExampleModel
///           type: object
///           properties:
///             id:
///               type: string
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_apigatewayv2_model` using the API identifier and model identifier. For example:
///
/// ```sh
/// $ pulumi import aws:apigatewayv2/model:Model example aabbccddee/1122334
/// ```
pub mod model {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ModelArgs {
        /// API identifier.
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// The content-type for the model, for example, `application/json`. Must be between 1 and 256 characters in length.
        #[builder(into)]
        pub content_type: pulumi_wasm_rust::Output<String>,
        /// Description of the model. Must be between 1 and 128 characters in length.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the model. Must be alphanumeric. Must be between 1 and 128 characters in length.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Schema for the model. This should be a [JSON schema draft 4](https://tools.ietf.org/html/draft-zyp-json-schema-04) model. Must be less than or equal to 32768 characters in length.
        #[builder(into)]
        pub schema: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ModelResult {
        /// API identifier.
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// The content-type for the model, for example, `application/json`. Must be between 1 and 256 characters in length.
        pub content_type: pulumi_wasm_rust::Output<String>,
        /// Description of the model. Must be between 1 and 128 characters in length.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the model. Must be alphanumeric. Must be between 1 and 128 characters in length.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Schema for the model. This should be a [JSON schema draft 4](https://tools.ietf.org/html/draft-zyp-json-schema-04) model. Must be less than or equal to 32768 characters in length.
        pub schema: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ModelArgs) -> ModelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_inner();
        let content_type_binding = args.content_type.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let schema_binding = args.schema.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigatewayv2/model:Model".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "contentType".into(),
                    value: &content_type_binding,
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
                    name: "schema".into(),
                    value: &schema_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiId".into(),
                },
                register_interface::ResultField {
                    name: "contentType".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "schema".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ModelResult {
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            content_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentType").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            schema: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schema").unwrap(),
            ),
        }
    }
}
