/// Provides a Model for a REST API Gateway.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   myDemoAPI:
///     type: aws:apigateway:RestApi
///     name: MyDemoAPI
///     properties:
///       name: MyDemoAPI
///       description: This is my API for demonstration purposes
///   myDemoModel:
///     type: aws:apigateway:Model
///     name: MyDemoModel
///     properties:
///       restApi: ${myDemoAPI.id}
///       name: user
///       description: a JSON schema
///       contentType: application/json
///       schema:
///         fn::toJSON:
///           type: object
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_api_gateway_model` using `REST-API-ID/NAME`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/model:Model example 12345abcde/example
/// ```
pub mod model {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ModelArgs {
        /// Content type of the model
        #[builder(into)]
        pub content_type: pulumi_wasm_rust::Output<String>,
        /// Description of the model
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the model
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the associated REST API
        #[builder(into)]
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// Schema of the model in a JSON form
        #[builder(into, default)]
        pub schema: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ModelResult {
        /// Content type of the model
        pub content_type: pulumi_wasm_rust::Output<String>,
        /// Description of the model
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the model
        pub name: pulumi_wasm_rust::Output<String>,
        /// ID of the associated REST API
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// Schema of the model in a JSON form
        pub schema: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ModelArgs) -> ModelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let content_type_binding = args.content_type.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let rest_api_binding = args.rest_api.get_inner();
        let schema_binding = args.schema.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/model:Model".into(),
            name: name.to_string(),
            object: Vec::from([
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
                    name: "restApi".into(),
                    value: &rest_api_binding,
                },
                register_interface::ObjectField {
                    name: "schema".into(),
                    value: &schema_binding,
                },
            ]),
            results: Vec::from([
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
                    name: "restApi".into(),
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
            content_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentType").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            rest_api: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restApi").unwrap(),
            ),
            schema: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schema").unwrap(),
            ),
        }
    }
}
