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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SourceCodeTokenArgs {
        /// The OAuth access token.
        #[builder(into)]
        pub token: pulumi_wasm_rust::InputOrOutput<String>,
        /// The OAuth access token secret.
        #[builder(into, default)]
        pub token_secret: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The source control type. Possible values are `BitBucket`, `Dropbox`, `GitHub` and `OneDrive`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SourceCodeTokenArgs,
    ) -> SourceCodeTokenResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let token_binding = args.token.get_output(context).get_inner();
        let token_secret_binding = args.token_secret.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        SourceCodeTokenResult {
            token: pulumi_wasm_rust::__private::into_domain(o.extract_field("token")),
            token_secret: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tokenSecret"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
