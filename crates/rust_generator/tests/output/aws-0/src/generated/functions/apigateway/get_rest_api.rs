pub mod get_rest_api {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRestApiArgs {
        /// Name of the REST API to look up. If no REST API is found with this name, an error will be returned. If multiple REST APIs are found with this name, an error will be returned.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetRestApiResult {
        /// Source of the API key for requests.
        pub api_key_source: pulumi_gestalt_rust::Output<String>,
        /// ARN of the REST API.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// List of binary media types supported by the REST API.
        pub binary_media_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Description of the REST API.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The endpoint configuration of this RestApi showing the endpoint types of the API.
        pub endpoint_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::apigateway::GetRestApiEndpointConfiguration>,
        >,
        /// Execution ARN part to be used in `lambda_permission`'s `source_arn` when allowing API Gateway to invoke a Lambda function, e.g., `arn:aws:execute-api:eu-west-2:123456789012:z4675bid1j`, which can be concatenated with allowed stage, method and resource path.
        pub execution_arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Minimum response size to compress for the REST API.
        pub minimum_compression_size: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// JSON formatted policy document that controls access to the API Gateway.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// Set to the ID of the API Gateway Resource on the found REST API where the route matches '/'.
        pub root_resource_id: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetRestApiArgs,
    ) -> GetRestApiResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:apigateway/getRestApi:getRestApi".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRestApiResult {
            api_key_source: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiKeySource"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            binary_media_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("binaryMediaTypes"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            endpoint_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointConfigurations"),
            ),
            execution_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("executionArn"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            minimum_compression_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minimumCompressionSize"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policy"),
            ),
            root_resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rootResourceId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
