pub mod get_sdk {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSdkArgs {
        /// Key-value map of query string parameters `sdk_type` properties of the SDK. For SDK Type of `objectivec` or `swift`, a parameter named `classPrefix` is required. For SDK Type of `android`, parameters named `groupId`, `artifactId`, `artifactVersion`, and `invokerPackage` are required. For SDK Type of `java`, parameters named `serviceName` and `javaPackageName` are required.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the associated REST API.
        #[builder(into)]
        pub rest_api_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Language for the generated SDK. Currently `java`, `javascript`, `android`, `objectivec` (for iOS), `swift` (for iOS), and `ruby` are supported.
        #[builder(into)]
        pub sdk_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the Stage that will be exported.
        #[builder(into)]
        pub stage_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSdkResult {
        /// SDK as a string.
        pub body: pulumi_wasm_rust::Output<String>,
        /// Content-disposition header value in the HTTP response.
        pub content_disposition: pulumi_wasm_rust::Output<String>,
        /// Content-type header value in the HTTP response.
        pub content_type: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub rest_api_id: pulumi_wasm_rust::Output<String>,
        pub sdk_type: pulumi_wasm_rust::Output<String>,
        pub stage_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSdkArgs,
    ) -> GetSdkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let rest_api_id_binding = args.rest_api_id.get_output(context).get_inner();
        let sdk_type_binding = args.sdk_type.get_output(context).get_inner();
        let stage_name_binding = args.stage_name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:apigateway/getSdk:getSdk".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "restApiId".into(),
                    value: &rest_api_id_binding,
                },
                register_interface::ObjectField {
                    name: "sdkType".into(),
                    value: &sdk_type_binding,
                },
                register_interface::ObjectField {
                    name: "stageName".into(),
                    value: &stage_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSdkResult {
            body: pulumi_wasm_rust::__private::into_domain(o.extract_field("body")),
            content_disposition: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contentDisposition"),
            ),
            content_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contentType"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            parameters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            rest_api_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("restApiId"),
            ),
            sdk_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sdkType"),
            ),
            stage_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("stageName"),
            ),
        }
    }
}
