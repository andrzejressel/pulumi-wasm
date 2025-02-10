#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_web_app_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWebAppConfigArgs {
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// the id of the firebase web app
        ///
        /// - - -
        #[builder(into)]
        pub web_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetWebAppConfigResult {
        /// The API key associated with the web App.
        pub api_key: pulumi_gestalt_rust::Output<String>,
        /// The domain Firebase Auth configures for OAuth redirects, in the format:
        /// projectId.firebaseapp.com
        pub auth_domain: pulumi_gestalt_rust::Output<String>,
        /// The default Firebase Realtime Database URL.
        pub database_url: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project's default GCP resource location. The location is one of the available GCP resource
        /// locations.
        /// This field is omitted if the default GCP resource location has not been finalized yet. To set your project's
        /// default GCP resource location, call defaultLocation.finalize after you add Firebase services to your project.
        pub location_id: pulumi_gestalt_rust::Output<String>,
        /// The unique Google-assigned identifier of the Google Analytics web stream associated with the Firebase Web App.
        /// Firebase SDKs use this ID to interact with Google Analytics APIs.
        /// This field is only present if the App is linked to a web stream in a Google Analytics App + Web property.
        /// Learn more about this ID and Google Analytics web streams in the Analytics documentation.
        /// To generate a measurementId and link the Web App with a Google Analytics web stream,
        /// call projects.addGoogleAnalytics.
        pub measurement_id: pulumi_gestalt_rust::Output<String>,
        /// The sender ID for use with Firebase Cloud Messaging.
        pub messaging_sender_id: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        /// The default Cloud Storage for Firebase storage bucket name.
        pub storage_bucket: pulumi_gestalt_rust::Output<String>,
        pub web_app_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetWebAppConfigArgs,
    ) -> GetWebAppConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let project_binding = args.project.get_output(context);
        let web_app_id_binding = args.web_app_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:firebase/getWebAppConfig:getWebAppConfig".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "webAppId".into(),
                    value: web_app_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetWebAppConfigResult {
            api_key: o.get_field("apiKey"),
            auth_domain: o.get_field("authDomain"),
            database_url: o.get_field("databaseUrl"),
            id: o.get_field("id"),
            location_id: o.get_field("locationId"),
            measurement_id: o.get_field("measurementId"),
            messaging_sender_id: o.get_field("messagingSenderId"),
            project: o.get_field("project"),
            storage_bucket: o.get_field("storageBucket"),
            web_app_id: o.get_field("webAppId"),
        }
    }
}
