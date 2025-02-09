/// Allows creation and management of an App Engine application.
///
/// > App Engine applications cannot be deleted once they're created; you have to delete the
///    entire project to delete the application. This provider will report the application has been
///    successfully deleted; this is a limitation of the provider, and will go away in the future.
///    This provider is not able to delete App Engine applications.
///
///
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let app = application::create(
///         "app",
///         ApplicationArgs::builder()
///             .location_id("us-central")
///             .project("${myProject.projectId}")
///             .build_struct(),
///     );
///     let myProject = project::create(
///         "myProject",
///         ProjectArgs::builder()
///             .name("My Project")
///             .org_id("1234567")
///             .project_id("your-project-id")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Applications can be imported using the ID of the project the application belongs to, e.g.
///
/// * `{{project-id}}`
///
/// When using the `pulumi import` command, Applications can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:appengine/application:Application default {{project-id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// The domain to authenticate users with when using App Engine's User API.
        #[builder(into, default)]
        pub auth_domain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of the Cloud Firestore or Cloud Datastore database associated with this application.
        /// Can be `CLOUD_FIRESTORE` or `CLOUD_DATASTORE_COMPATIBILITY` for new
        /// instances.  To support old instances, the value `CLOUD_DATASTORE` is accepted by the provider, but will be rejected by the API.
        /// To create a Cloud Firestore database without creating an App Engine application, use the
        /// `gcp.firestore.Database`
        /// resource instead.
        #[builder(into, default)]
        pub database_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A block of optional settings to configure specific App Engine features:
        #[builder(into, default)]
        pub feature_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appengine::ApplicationFeatureSettings>,
        >,
        /// Settings for enabling Cloud Identity Aware Proxy
        #[builder(into, default)]
        pub iap: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appengine::ApplicationIap>,
        >,
        /// The [location](https://cloud.google.com/appengine/docs/locations)
        /// to serve the app from.
        #[builder(into)]
        pub location_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project ID to create the application under.
        /// ~>**NOTE:** GCP only accepts project ID, not project number. If you are using number,
        /// you may get a "Permission denied" error.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The serving status of the app.
        #[builder(into, default)]
        pub serving_status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// Identifier of the app, usually `{PROJECT_ID}`
        pub app_id: pulumi_gestalt_rust::Output<String>,
        /// The domain to authenticate users with when using App Engine's User API.
        pub auth_domain: pulumi_gestalt_rust::Output<String>,
        /// The GCS bucket code is being stored in for this app.
        pub code_bucket: pulumi_gestalt_rust::Output<String>,
        /// The type of the Cloud Firestore or Cloud Datastore database associated with this application.
        /// Can be `CLOUD_FIRESTORE` or `CLOUD_DATASTORE_COMPATIBILITY` for new
        /// instances.  To support old instances, the value `CLOUD_DATASTORE` is accepted by the provider, but will be rejected by the API.
        /// To create a Cloud Firestore database without creating an App Engine application, use the
        /// `gcp.firestore.Database`
        /// resource instead.
        pub database_type: pulumi_gestalt_rust::Output<String>,
        /// The GCS bucket content is being stored in for this app.
        pub default_bucket: pulumi_gestalt_rust::Output<String>,
        /// The default hostname for this app.
        pub default_hostname: pulumi_gestalt_rust::Output<String>,
        /// A block of optional settings to configure specific App Engine features:
        pub feature_settings: pulumi_gestalt_rust::Output<
            super::super::types::appengine::ApplicationFeatureSettings,
        >,
        /// The GCR domain used for storing managed Docker images for this app.
        pub gcr_domain: pulumi_gestalt_rust::Output<String>,
        /// Settings for enabling Cloud Identity Aware Proxy
        pub iap: pulumi_gestalt_rust::Output<
            super::super::types::appengine::ApplicationIap,
        >,
        /// The [location](https://cloud.google.com/appengine/docs/locations)
        /// to serve the app from.
        pub location_id: pulumi_gestalt_rust::Output<String>,
        /// Unique name of the app, usually `apps/{PROJECT_ID}`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The project ID to create the application under.
        /// ~>**NOTE:** GCP only accepts project ID, not project number. If you are using number,
        /// you may get a "Permission denied" error.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The serving status of the app.
        pub serving_status: pulumi_gestalt_rust::Output<String>,
        /// A list of dispatch rule blocks. Each block has a `domain`, `path`, and `service` field.
        pub url_dispatch_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appengine::ApplicationUrlDispatchRule>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationArgs,
    ) -> ApplicationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auth_domain_binding = args.auth_domain.get_output(context);
        let database_type_binding = args.database_type.get_output(context);
        let feature_settings_binding = args.feature_settings.get_output(context);
        let iap_binding = args.iap.get_output(context);
        let location_id_binding = args.location_id.get_output(context);
        let project_binding = args.project.get_output(context);
        let serving_status_binding = args.serving_status.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:appengine/application:Application".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authDomain".into(),
                    value: auth_domain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseType".into(),
                    value: database_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "featureSettings".into(),
                    value: feature_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iap".into(),
                    value: iap_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "locationId".into(),
                    value: location_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "servingStatus".into(),
                    value: serving_status_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationResult {
            app_id: o.get_field("appId"),
            auth_domain: o.get_field("authDomain"),
            code_bucket: o.get_field("codeBucket"),
            database_type: o.get_field("databaseType"),
            default_bucket: o.get_field("defaultBucket"),
            default_hostname: o.get_field("defaultHostname"),
            feature_settings: o.get_field("featureSettings"),
            gcr_domain: o.get_field("gcrDomain"),
            iap: o.get_field("iap"),
            location_id: o.get_field("locationId"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            serving_status: o.get_field("servingStatus"),
            url_dispatch_rules: o.get_field("urlDispatchRules"),
        }
    }
}
