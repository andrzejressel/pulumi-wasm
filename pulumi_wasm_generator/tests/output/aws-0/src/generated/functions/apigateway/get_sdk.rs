pub mod get_sdk {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSdkArgs {
        /// Key-value map of query string parameters `sdk_type` properties of the SDK. For SDK Type of `objectivec` or `swift`, a parameter named `classPrefix` is required. For SDK Type of `android`, parameters named `groupId`, `artifactId`, `artifactVersion`, and `invokerPackage` are required. For SDK Type of `java`, parameters named `serviceName` and `javaPackageName` are required.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the associated REST API.
        #[builder(into)]
        pub rest_api_id: pulumi_wasm_rust::Output<String>,
        /// Language for the generated SDK. Currently `java`, `javascript`, `android`, `objectivec` (for iOS), `swift` (for iOS), and `ruby` are supported.
        #[builder(into)]
        pub sdk_type: pulumi_wasm_rust::Output<String>,
        /// Name of the Stage that will be exported.
        #[builder(into)]
        pub stage_name: pulumi_wasm_rust::Output<String>,
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
    pub fn invoke(args: GetSdkArgs) -> GetSdkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parameters_binding = args.parameters.get_inner();
        let rest_api_id_binding = args.rest_api_id.get_inner();
        let sdk_type_binding = args.sdk_type.get_inner();
        let stage_name_binding = args.stage_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:apigateway/getSdk:getSdk".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "body".into(),
                },
                register_interface::ResultField {
                    name: "contentDisposition".into(),
                },
                register_interface::ResultField {
                    name: "contentType".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "restApiId".into(),
                },
                register_interface::ResultField {
                    name: "sdkType".into(),
                },
                register_interface::ResultField {
                    name: "stageName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSdkResult {
            body: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("body").unwrap(),
            ),
            content_disposition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentDisposition").unwrap(),
            ),
            content_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentType").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            rest_api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restApiId").unwrap(),
            ),
            sdk_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sdkType").unwrap(),
            ),
            stage_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stageName").unwrap(),
            ),
        }
    }
}
