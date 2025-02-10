#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_function_url {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFunctionUrlArgs {
        /// The name (or ARN) of the Lambda function.
        #[builder(into)]
        pub function_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Alias name or `"$LATEST"`.
        #[builder(into, default)]
        pub qualifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetFunctionUrlResult {
        /// Type of authentication that the function URL uses.
        pub authorization_type: pulumi_gestalt_rust::Output<String>,
        /// The [cross-origin resource sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) settings for the function URL. See the `aws.lambda.FunctionUrl` resource documentation for more details.
        pub cors: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::lambda::GetFunctionUrlCor>,
        >,
        /// When the function URL was created, in [ISO-8601 format](https://www.w3.org/TR/NOTE-datetime).
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        /// ARN of the function.
        pub function_arn: pulumi_gestalt_rust::Output<String>,
        pub function_name: pulumi_gestalt_rust::Output<String>,
        /// HTTP URL endpoint for the function in the format `https://<url_id>.lambda-url.<region>.on.aws/`.
        pub function_url: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Whether the Lambda function responds in `BUFFERED` or `RESPONSE_STREAM` mode.
        pub invoke_mode: pulumi_gestalt_rust::Output<String>,
        /// When the function URL configuration was last updated, in [ISO-8601 format](https://www.w3.org/TR/NOTE-datetime).
        pub last_modified_time: pulumi_gestalt_rust::Output<String>,
        pub qualifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// Generated ID for the endpoint.
        pub url_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetFunctionUrlArgs,
    ) -> GetFunctionUrlResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let function_name_binding = args.function_name.get_output(context);
        let qualifier_binding = args.qualifier.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:lambda/getFunctionUrl:getFunctionUrl".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "functionName".into(),
                    value: function_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "qualifier".into(),
                    value: qualifier_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetFunctionUrlResult {
            authorization_type: o.get_field("authorizationType"),
            cors: o.get_field("cors"),
            creation_time: o.get_field("creationTime"),
            function_arn: o.get_field("functionArn"),
            function_name: o.get_field("functionName"),
            function_url: o.get_field("functionUrl"),
            id: o.get_field("id"),
            invoke_mode: o.get_field("invokeMode"),
            last_modified_time: o.get_field("lastModifiedTime"),
            qualifier: o.get_field("qualifier"),
            url_id: o.get_field("urlId"),
        }
    }
}
