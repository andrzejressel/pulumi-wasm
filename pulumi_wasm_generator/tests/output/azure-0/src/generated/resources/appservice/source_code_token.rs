/// Manages an App Service source control token.
///
/// !> **NOTE:** This resource has been deprecated in version 5.0 of the provider and will be removed in version 6.0. Please use `azure.appservice.ServicePlan` resource instead.
///
/// > **NOTE:** Source Control Tokens are configured at the subscription level, not on each App Service - as such this can only be configured Subscription-wide
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = source_code_token::create(
///         "example",
///         SourceCodeTokenArgs::builder()
///             .token("7e57735e77e577e57")
///             .type_("GitHub")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// App Service Source Control Token's can be imported using the `type`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/sourceCodeToken:SourceCodeToken example {type}
/// ```
///
pub mod source_code_token {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SourceCodeTokenArgs {
        /// The OAuth access token.
        #[builder(into)]
        pub token: pulumi_wasm_rust::Output<String>,
        /// The OAuth access token secret.
        #[builder(into, default)]
        pub token_secret: pulumi_wasm_rust::Output<Option<String>>,
        /// The source control type. Possible values are `BitBucket`, `Dropbox`, `GitHub` and `OneDrive`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SourceCodeTokenResult {
        /// The OAuth access token.
        pub token: pulumi_wasm_rust::Output<String>,
        /// The OAuth access token secret.
        pub token_secret: pulumi_wasm_rust::Output<Option<String>>,
        /// The source control type. Possible values are `BitBucket`, `Dropbox`, `GitHub` and `OneDrive`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SourceCodeTokenArgs) -> SourceCodeTokenResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let token_binding = args.token.get_inner();
        let token_secret_binding = args.token_secret.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/sourceCodeToken:SourceCodeToken".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "token".into(),
                    value: &token_binding,
                },
                register_interface::ObjectField {
                    name: "tokenSecret".into(),
                    value: &token_secret_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "token".into(),
                },
                register_interface::ResultField {
                    name: "tokenSecret".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SourceCodeTokenResult {
            token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("token").unwrap(),
            ),
            token_secret: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tokenSecret").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
