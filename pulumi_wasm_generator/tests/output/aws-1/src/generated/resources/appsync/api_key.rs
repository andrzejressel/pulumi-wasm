/// Provides an AppSync API Key.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = graph_ql_api::create(
///         "example",
///         GraphQlApiArgs::builder()
///             .authentication_type("API_KEY")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleApiKey = api_key::create(
///         "exampleApiKey",
///         ApiKeyArgs::builder()
///             .api_id("${example.id}")
///             .expires("2018-05-03T04:00:00Z")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_appsync_api_key` using the AppSync API ID and key separated by `:`. For example:
///
/// ```sh
/// $ pulumi import aws:appsync/apiKey:ApiKey example xxxxx:yyyyy
/// ```
pub mod api_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiKeyArgs {
        /// ID of the associated AppSync API
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// API key description. Defaults to "Managed by Pulumi".
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// RFC3339 string representation of the expiry date. Rounded down to nearest hour. By default, it is 7 days from the date of creation.
        #[builder(into, default)]
        pub expires: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiKeyResult {
        /// ID of the associated AppSync API
        pub api_id: pulumi_wasm_rust::Output<String>,
        pub api_key_id: pulumi_wasm_rust::Output<String>,
        /// API key description. Defaults to "Managed by Pulumi".
        pub description: pulumi_wasm_rust::Output<String>,
        /// RFC3339 string representation of the expiry date. Rounded down to nearest hour. By default, it is 7 days from the date of creation.
        pub expires: pulumi_wasm_rust::Output<Option<String>>,
        /// API key
        pub key: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ApiKeyArgs,
    ) -> ApiKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let expires_binding = args.expires.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appsync/apiKey:ApiKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "expires".into(),
                    value: &expires_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiId".into(),
                },
                register_interface::ResultField {
                    name: "apiKeyId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "expires".into(),
                },
                register_interface::ResultField {
                    name: "key".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApiKeyResult {
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            api_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiKeyId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            expires: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expires").unwrap(),
            ),
            key: pulumi_wasm_rust::__private::into_domain(hashmap.remove("key").unwrap()),
        }
    }
}
