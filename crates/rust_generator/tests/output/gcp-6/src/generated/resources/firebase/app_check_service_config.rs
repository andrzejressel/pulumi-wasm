/// The enforcement configuration for a service supported by App Check.
///
///
/// To get more information about ServiceConfig, see:
///
/// * [API documentation](https://firebase.google.com/docs/reference/appcheck/rest/v1/projects.services)
/// * How-to Guides
///     * [Official Documentation](https://firebase.google.com/docs/app-check)
///
/// ## Example Usage
///
/// ### Firebase App Check Service Config Off
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let appcheck = service::create(
///         "appcheck",
///         ServiceArgs::builder()
///             .disable_on_destroy(false)
///             .project("my-project-name")
///             .service("firebaseappcheck.googleapis.com")
///             .build_struct(),
///     );
///     let default = app_check_service_config::create(
///         "default",
///         AppCheckServiceConfigArgs::builder()
///             .project("my-project-name")
///             .service_id("firestore.googleapis.com")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firebase App Check Service Config Enforced
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let appcheck = service::create(
///         "appcheck",
///         ServiceArgs::builder()
///             .disable_on_destroy(false)
///             .project("my-project-name")
///             .service("firebaseappcheck.googleapis.com")
///             .build_struct(),
///     );
///     let default = app_check_service_config::create(
///         "default",
///         AppCheckServiceConfigArgs::builder()
///             .enforcement_mode("ENFORCED")
///             .project("my-project-name")
///             .service_id("firebasestorage.googleapis.com")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firebase App Check Service Config Unenforced
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let appcheck = service::create(
///         "appcheck",
///         ServiceArgs::builder()
///             .disable_on_destroy(false)
///             .project("my-project-name")
///             .service("firebaseappcheck.googleapis.com")
///             .build_struct(),
///     );
///     let default = app_check_service_config::create(
///         "default",
///         AppCheckServiceConfigArgs::builder()
///             .enforcement_mode("UNENFORCED")
///             .project("my-project-name")
///             .service_id("identitytoolkit.googleapis.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ServiceConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/services/{{service_id}}`
///
/// * `{{project}}/{{service_id}}`
///
/// * `{{service_id}}`
///
/// When using the `pulumi import` command, ServiceConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckServiceConfig:AppCheckServiceConfig default projects/{{project}}/services/{{service_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckServiceConfig:AppCheckServiceConfig default {{project}}/{{service_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckServiceConfig:AppCheckServiceConfig default {{service_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod app_check_service_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppCheckServiceConfigArgs {
        #[builder(into, default)]
        pub enforcement_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The identifier of the service to configure enforcement. Currently, the following service IDs are supported:
        /// firebasestorage.googleapis.com (Cloud Storage for Firebase)
        /// firebasedatabase.googleapis.com (Firebase Realtime Database)
        /// firestore.googleapis.com (Cloud Firestore)
        /// identitytoolkit.googleapis.com (Authentication)
        ///
        ///
        /// - - -
        #[builder(into)]
        pub service_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AppCheckServiceConfigResult {
        pub enforcement_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The fully-qualified resource name of the service enforcement configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The identifier of the service to configure enforcement. Currently, the following service IDs are supported:
        /// firebasestorage.googleapis.com (Cloud Storage for Firebase)
        /// firebasedatabase.googleapis.com (Firebase Realtime Database)
        /// firestore.googleapis.com (Cloud Firestore)
        /// identitytoolkit.googleapis.com (Authentication)
        ///
        ///
        /// - - -
        pub service_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AppCheckServiceConfigArgs,
    ) -> AppCheckServiceConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let enforcement_mode_binding = args.enforcement_mode.get_output(context);
        let project_binding = args.project.get_output(context);
        let service_id_binding = args.service_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:firebase/appCheckServiceConfig:AppCheckServiceConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enforcementMode".into(),
                    value: enforcement_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceId".into(),
                    value: service_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AppCheckServiceConfigResult {
            enforcement_mode: o.get_field("enforcementMode"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            service_id: o.get_field("serviceId"),
        }
    }
}
