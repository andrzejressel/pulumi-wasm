/// A Google Cloud Firebase web application instance
///
/// To get more information about WebApp, see:
///
/// * [API documentation](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps)
/// * How-to Guides
///     * [Official Documentation](https://firebase.google.com/)
///
/// ## Example Usage
///
/// ### Firebase Web App Custom Api Key
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = web_app::create(
///         "default",
///         WebAppArgs::builder()
///             .api_key_id("${web.uid}")
///             .deletion_policy("DELETE")
///             .display_name("Display Name")
///             .project("my-project-name")
///             .build_struct(),
///     );
///     let web = api_key::create(
///         "web",
///         ApiKeyArgs::builder()
///             .display_name("Display Name")
///             .name("api-key")
///             .project("my-project-name")
///             .restrictions(
///                 ApiKeyRestrictions::builder()
///                     .browserKeyRestrictions(
///                         ApiKeyRestrictionsBrowserKeyRestrictions::builder()
///                             .allowedReferrers(vec!["*",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// WebApp can be imported using any of these accepted formats:
///
/// * `{{project}} projects/{{project}}/webApps/{{app_id}}`
///
/// * `projects/{{project}}/webApps/{{app_id}}`
///
/// * `{{project}}/{{project}}/{{app_id}}`
///
/// * `webApps/{{app_id}}`
///
/// * `{{app_id}}`
///
/// When using the `pulumi import` command, WebApp can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/webApp:WebApp default "{{project}} projects/{{project}}/webApps/{{app_id}}"
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/webApp:WebApp default projects/{{project}}/webApps/{{app_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/webApp:WebApp default {{project}}/{{project}}/{{app_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/webApp:WebApp default webApps/{{app_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/webApp:WebApp default {{app_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod web_app {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebAppArgs {
        /// The globally unique, Google-assigned identifier (UID) for the Firebase API key associated with the WebApp.
        /// If apiKeyId is not set during creation, then Firebase automatically associates an apiKeyId with the WebApp.
        /// This auto-associated key may be an existing valid key or, if no valid key exists, a new one will be provisioned.
        #[builder(into, default)]
        pub api_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub deletion_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The user-assigned display name of the App.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct WebAppResult {
        /// The globally unique, Google-assigned identifier (UID) for the Firebase API key associated with the WebApp.
        /// If apiKeyId is not set during creation, then Firebase automatically associates an apiKeyId with the WebApp.
        /// This auto-associated key may be an existing valid key or, if no valid key exists, a new one will be provisioned.
        pub api_key_id: pulumi_gestalt_rust::Output<String>,
        /// The globally unique, Firebase-assigned identifier of the App.
        /// This identifier should be treated as an opaque token, as the data format is not specified.
        pub app_id: pulumi_gestalt_rust::Output<String>,
        /// The URLs where the `WebApp` is hosted.
        pub app_urls: pulumi_gestalt_rust::Output<Vec<String>>,
        pub deletion_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The user-assigned display name of the App.
        ///
        ///
        /// - - -
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The fully qualified resource name of the App, for example:
        /// projects/projectId/webApps/appId
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WebAppArgs,
    ) -> WebAppResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_key_id_binding = args.api_key_id.get_output(context);
        let deletion_policy_binding = args.deletion_policy.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:firebase/webApp:WebApp".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiKeyId".into(),
                    value: &api_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionPolicy".into(),
                    value: &deletion_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        WebAppResult {
            api_key_id: o.get_field("apiKeyId"),
            app_id: o.get_field("appId"),
            app_urls: o.get_field("appUrls"),
            deletion_policy: o.get_field("deletionPolicy"),
            display_name: o.get_field("displayName"),
            name: o.get_field("name"),
            project: o.get_field("project"),
        }
    }
}
