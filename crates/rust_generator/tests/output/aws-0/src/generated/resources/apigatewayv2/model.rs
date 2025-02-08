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
#[allow(clippy::doc_lazy_continuation)]
pub mod model {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ModelArgs {
        /// API identifier.
        #[builder(into)]
        pub api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The content-type for the model, for example, `application/json`. Must be between 1 and 256 characters in length.
        #[builder(into)]
        pub content_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the model. Must be between 1 and 128 characters in length.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the model. Must be alphanumeric. Must be between 1 and 128 characters in length.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Schema for the model. This should be a [JSON schema draft 4](https://tools.ietf.org/html/draft-zyp-json-schema-04) model. Must be less than or equal to 32768 characters in length.
        #[builder(into)]
        pub schema: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ModelResult {
        /// API identifier.
        pub api_id: pulumi_gestalt_rust::Output<String>,
        /// The content-type for the model, for example, `application/json`. Must be between 1 and 256 characters in length.
        pub content_type: pulumi_gestalt_rust::Output<String>,
        /// Description of the model. Must be between 1 and 128 characters in length.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the model. Must be alphanumeric. Must be between 1 and 128 characters in length.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Schema for the model. This should be a [JSON schema draft 4](https://tools.ietf.org/html/draft-zyp-json-schema-04) model. Must be less than or equal to 32768 characters in length.
        pub schema: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ModelArgs,
    ) -> ModelResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_output(context).get_inner();
        let content_type_binding = args.content_type.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let schema_binding = args.schema.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigatewayv2/model:Model".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ModelResult {
            api_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiId"),
            ),
            content_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentType"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            schema: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schema"),
            ),
        }
    }
}
