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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ModelArgs {
        /// Content type of the model
        #[builder(into)]
        pub content_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// Description of the model
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the model
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ID of the associated REST API
        #[builder(into)]
        pub rest_api: pulumi_wasm_rust::InputOrOutput<String>,
        /// Schema of the model in a JSON form
        #[builder(into, default)]
        pub schema: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ModelArgs,
    ) -> ModelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let content_type_binding = args.content_type.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let rest_api_binding = args.rest_api.get_output(context).get_inner();
        let schema_binding = args.schema.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/model:Model".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ModelResult {
            content_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contentType"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            rest_api: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("restApi"),
            ),
            schema: pulumi_wasm_rust::__private::into_domain(o.extract_field("schema")),
        }
    }
}
