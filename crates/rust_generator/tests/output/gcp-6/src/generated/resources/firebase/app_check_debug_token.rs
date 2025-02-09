/// A debug token is a secret used during the development or integration testing of
/// an app. It essentially allows the development or integration testing to bypass
/// app attestation while still allowing App Check to enforce protection on supported
/// production Firebase services.
///
///
/// To get more information about DebugToken, see:
///
/// * [API documentation](https://firebase.google.com/docs/reference/appcheck/rest/v1/projects.apps.debugTokens)
/// * How-to Guides
///     * [Official Documentation](https://firebase.google.com/docs/app-check)
///
///
///
/// ## Example Usage
///
/// ### Firebase App Check Debug Token Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:firebase:WebApp
///     properties:
///       project: my-project-name
///       displayName: Web App for debug token
///   # It takes a while for App Check to recognize the new app
///   # If your app already exists, you don't have to wait 30 seconds.
///   wait30s:
///     type: time:sleep
///     name: wait_30s
///     properties:
///       createDuration: 30s
///     options:
///       dependsOn:
///         - ${default}
///   defaultAppCheckDebugToken:
///     type: gcp:firebase:AppCheckDebugToken
///     name: default
///     properties:
///       project: my-project-name
///       appId: ${default.appId}
///       displayName: Debug Token
///       token: 00000000-AAAA-BBBB-CCCC-000000000000
///     options:
///       dependsOn:
///         - ${wait30s}
/// ```
///
/// ## Import
///
/// DebugToken can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/apps/{{app_id}}/debugTokens/{{debug_token_id}}`
///
/// * `{{project}}/{{app_id}}/{{debug_token_id}}`
///
/// * `{{app_id}}/{{debug_token_id}}`
///
/// When using the `pulumi import` command, DebugToken can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckDebugToken:AppCheckDebugToken default projects/{{project}}/apps/{{app_id}}/debugTokens/{{debug_token_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckDebugToken:AppCheckDebugToken default {{project}}/{{app_id}}/{{debug_token_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckDebugToken:AppCheckDebugToken default {{app_id}}/{{debug_token_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod app_check_debug_token {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppCheckDebugTokenArgs {
        /// The ID of a
        /// [Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id),
        /// [Apple App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.iosApps#IosApp.FIELDS.app_id),
        /// or [Android App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.androidApps#AndroidApp.FIELDS.app_id)
        ///
        ///
        /// - - -
        #[builder(into)]
        pub app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A human readable display name used to identify this debug token.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The secret token itself. Must be provided during creation, and must be a UUID4,
        /// case insensitive. You may use a method of your choice such as random/random_uuid
        /// to generate the token.
        /// This field is immutable once set, and cannot be updated. You can, however, delete
        /// this debug token to revoke it.
        /// For security reasons, this field will never be populated in any response.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        #[builder(into)]
        pub token: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AppCheckDebugTokenResult {
        /// The ID of a
        /// [Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id),
        /// [Apple App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.iosApps#IosApp.FIELDS.app_id),
        /// or [Android App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.androidApps#AndroidApp.FIELDS.app_id)
        ///
        ///
        /// - - -
        pub app_id: pulumi_gestalt_rust::Output<String>,
        /// The last segment of the resource name of the debug token.
        pub debug_token_id: pulumi_gestalt_rust::Output<String>,
        /// A human readable display name used to identify this debug token.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The secret token itself. Must be provided during creation, and must be a UUID4,
        /// case insensitive. You may use a method of your choice such as random/random_uuid
        /// to generate the token.
        /// This field is immutable once set, and cannot be updated. You can, however, delete
        /// this debug token to revoke it.
        /// For security reasons, this field will never be populated in any response.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        pub token: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AppCheckDebugTokenArgs,
    ) -> AppCheckDebugTokenResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_id_binding = args.app_id.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let project_binding = args.project.get_output(context);
        let token_binding = args.token.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:firebase/appCheckDebugToken:AppCheckDebugToken".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appId".into(),
                    value: app_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "token".into(),
                    value: token_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AppCheckDebugTokenResult {
            app_id: o.get_field("appId"),
            debug_token_id: o.get_field("debugTokenId"),
            display_name: o.get_field("displayName"),
            project: o.get_field("project"),
            token: o.get_field("token"),
        }
    }
}
