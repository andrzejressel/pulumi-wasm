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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod authentication_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthenticationProfileArgs {
        /// The content of the authentication profile in JSON format. The maximum length of the JSON string is determined by a quota for your account.
        #[builder(into)]
        pub authentication_profile_content: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the authentication profile.
        #[builder(into)]
        pub authentication_profile_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AuthenticationProfileResult {
        /// The content of the authentication profile in JSON format. The maximum length of the JSON string is determined by a quota for your account.
        pub authentication_profile_content: pulumi_gestalt_rust::Output<String>,
        /// The name of the authentication profile.
        pub authentication_profile_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AuthenticationProfileArgs,
    ) -> AuthenticationProfileResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authentication_profile_content_binding = args
            .authentication_profile_content
            .get_output(context);
        let authentication_profile_name_binding = args
            .authentication_profile_name
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:redshift/authenticationProfile:AuthenticationProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationProfileContent".into(),
                    value: authentication_profile_content_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationProfileName".into(),
                    value: authentication_profile_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AuthenticationProfileResult {
            authentication_profile_content: o.get_field("authenticationProfileContent"),
            authentication_profile_name: o.get_field("authenticationProfileName"),
        }
    }
}
