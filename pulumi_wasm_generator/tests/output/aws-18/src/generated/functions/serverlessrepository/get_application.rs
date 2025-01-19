pub mod get_application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApplicationArgs {
        /// ARN of the application.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// Requested version of the application. By default, retrieves the latest version.
        #[builder(into, default)]
        pub semantic_version: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetApplicationResult {
        /// ARN of the application.
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Name of the application.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of capabilities describing the permissions needed to deploy the application.
        pub required_capabilities: pulumi_wasm_rust::Output<Vec<String>>,
        pub semantic_version: pulumi_wasm_rust::Output<String>,
        /// URL pointing to the source code of the application version.
        pub source_code_url: pulumi_wasm_rust::Output<String>,
        /// URL pointing to the Cloud Formation template for the application version.
        pub template_url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetApplicationArgs) -> GetApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_inner();
        let semantic_version_binding = args.semantic_version.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:serverlessrepository/getApplication:getApplication".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "semanticVersion".into(),
                    value: &semantic_version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "requiredCapabilities".into(),
                },
                register_interface::ResultField {
                    name: "semanticVersion".into(),
                },
                register_interface::ResultField {
                    name: "sourceCodeUrl".into(),
                },
                register_interface::ResultField {
                    name: "templateUrl".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetApplicationResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            required_capabilities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requiredCapabilities").unwrap(),
            ),
            semantic_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("semanticVersion").unwrap(),
            ),
            source_code_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceCodeUrl").unwrap(),
            ),
            template_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templateUrl").unwrap(),
            ),
        }
    }
}
