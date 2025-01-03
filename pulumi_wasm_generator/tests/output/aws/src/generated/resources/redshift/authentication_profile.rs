/// Creates a Redshift authentication profile
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:redshift:AuthenticationProfile
///     properties:
///       authenticationProfileName: example
///       authenticationProfileContent:
///         fn::toJSON:
///           AllowDBUserOverride: '1'
///           Client_ID: ExampleClientID
///           App_ID: example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Authentication by `authentication_profile_name`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/authenticationProfile:AuthenticationProfile test example
/// ```
pub mod authentication_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthenticationProfileArgs {
        /// The content of the authentication profile in JSON format. The maximum length of the JSON string is determined by a quota for your account.
        #[builder(into)]
        pub authentication_profile_content: pulumi_wasm_rust::Output<String>,
        /// The name of the authentication profile.
        #[builder(into)]
        pub authentication_profile_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AuthenticationProfileResult {
        /// The content of the authentication profile in JSON format. The maximum length of the JSON string is determined by a quota for your account.
        pub authentication_profile_content: pulumi_wasm_rust::Output<String>,
        /// The name of the authentication profile.
        pub authentication_profile_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AuthenticationProfileArgs,
    ) -> AuthenticationProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_profile_content_binding = args
            .authentication_profile_content
            .get_inner();
        let authentication_profile_name_binding = args
            .authentication_profile_name
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/authenticationProfile:AuthenticationProfile".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authenticationProfileContent".into(),
                    value: &authentication_profile_content_binding,
                },
                register_interface::ObjectField {
                    name: "authenticationProfileName".into(),
                    value: &authentication_profile_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authenticationProfileContent".into(),
                },
                register_interface::ResultField {
                    name: "authenticationProfileName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AuthenticationProfileResult {
            authentication_profile_content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationProfileContent").unwrap(),
            ),
            authentication_profile_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationProfileName").unwrap(),
            ),
        }
    }
}
