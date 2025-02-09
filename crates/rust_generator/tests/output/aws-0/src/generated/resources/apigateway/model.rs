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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod model {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ModelArgs {
        /// Content type of the model
        #[builder(into)]
        pub content_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the model
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the model
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the associated REST API
        #[builder(into)]
        pub rest_api: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Schema of the model in a JSON form
        #[builder(into, default)]
        pub schema: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ModelResult {
        /// Content type of the model
        pub content_type: pulumi_gestalt_rust::Output<String>,
        /// Description of the model
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the model
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ID of the associated REST API
        pub rest_api: pulumi_gestalt_rust::Output<String>,
        /// Schema of the model in a JSON form
        pub schema: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ModelArgs,
    ) -> ModelResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let content_type_binding = args.content_type.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let rest_api_binding = args.rest_api.get_output(context);
        let schema_binding = args.schema.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/model:Model".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentType".into(),
                    value: content_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restApi".into(),
                    value: rest_api_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schema".into(),
                    value: schema_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ModelResult {
            content_type: o.get_field("contentType"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            rest_api: o.get_field("restApi"),
            schema: o.get_field("schema"),
        }
    }
}
