#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_apple_app {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAppleAppArgs {
        /// The app_id of name of the Firebase iosApp.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAppleAppResult {
        pub api_key_id: pulumi_gestalt_rust::Output<String>,
        /// Immutable. The globally unique, Firebase-assigned identifier of the App.
        /// This identifier should be treated as an opaque token, as the data format is not specified.
        pub app_id: pulumi_gestalt_rust::Output<String>,
        /// The automatically generated Apple ID assigned to the Apple app by Apple in the Apple App Store.
        pub app_store_id: pulumi_gestalt_rust::Output<String>,
        /// The canonical bundle ID of the Apple app as it would appear in the Apple AppStore.
        pub bundle_id: pulumi_gestalt_rust::Output<String>,
        pub deletion_policy: pulumi_gestalt_rust::Output<String>,
        /// The user-assigned display name of the App.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The fully qualified resource name of the App, for example:
        /// projects/projectId/iosApps/appId
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Apple Developer Team ID associated with the App in the App Store.
        pub team_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAppleAppArgs,
    ) -> GetAppleAppResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_id_binding = args.app_id.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:firebase/getAppleApp:getAppleApp".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appId".into(),
                    value: &app_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAppleAppResult {
            api_key_id: o.get_field("apiKeyId"),
            app_id: o.get_field("appId"),
            app_store_id: o.get_field("appStoreId"),
            bundle_id: o.get_field("bundleId"),
            deletion_policy: o.get_field("deletionPolicy"),
            display_name: o.get_field("displayName"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            team_id: o.get_field("teamId"),
        }
    }
}
