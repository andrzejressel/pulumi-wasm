#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_sdk {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSdkArgs {
        /// Key-value map of query string parameters `sdk_type` properties of the SDK. For SDK Type of `objectivec` or `swift`, a parameter named `classPrefix` is required. For SDK Type of `android`, parameters named `groupId`, `artifactId`, `artifactVersion`, and `invokerPackage` are required. For SDK Type of `java`, parameters named `serviceName` and `javaPackageName` are required.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the associated REST API.
        #[builder(into)]
        pub rest_api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Language for the generated SDK. Currently `java`, `javascript`, `android`, `objectivec` (for iOS), `swift` (for iOS), and `ruby` are supported.
        #[builder(into)]
        pub sdk_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the Stage that will be exported.
        #[builder(into)]
        pub stage_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSdkResult {
        /// SDK as a string.
        pub body: pulumi_gestalt_rust::Output<String>,
        /// Content-disposition header value in the HTTP response.
        pub content_disposition: pulumi_gestalt_rust::Output<String>,
        /// Content-type header value in the HTTP response.
        pub content_type: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub rest_api_id: pulumi_gestalt_rust::Output<String>,
        pub sdk_type: pulumi_gestalt_rust::Output<String>,
        pub stage_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSdkArgs,
    ) -> GetSdkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let parameters_binding = args.parameters.get_output(context);
        let rest_api_id_binding = args.rest_api_id.get_output(context);
        let sdk_type_binding = args.sdk_type.get_output(context);
        let stage_name_binding = args.stage_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:apigateway/getSdk:getSdk".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restApiId".into(),
                    value: rest_api_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sdkType".into(),
                    value: sdk_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stageName".into(),
                    value: stage_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSdkResult {
            body: o.get_field("body"),
            content_disposition: o.get_field("contentDisposition"),
            content_type: o.get_field("contentType"),
            id: o.get_field("id"),
            parameters: o.get_field("parameters"),
            rest_api_id: o.get_field("restApiId"),
            sdk_type: o.get_field("sdkType"),
            stage_name: o.get_field("stageName"),
        }
    }
}
