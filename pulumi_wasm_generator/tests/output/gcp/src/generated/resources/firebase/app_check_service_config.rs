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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod app_check_service_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppCheckServiceConfigArgs {
        #[builder(into, default)]
        pub enforcement_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The identifier of the service to configure enforcement. Currently, the following service IDs are supported:
        /// firebasestorage.googleapis.com (Cloud Storage for Firebase)
        /// firebasedatabase.googleapis.com (Firebase Realtime Database)
        /// firestore.googleapis.com (Cloud Firestore)
        /// identitytoolkit.googleapis.com (Authentication)
        ///
        ///
        /// - - -
        #[builder(into)]
        pub service_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AppCheckServiceConfigResult {
        pub enforcement_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The fully-qualified resource name of the service enforcement configuration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The identifier of the service to configure enforcement. Currently, the following service IDs are supported:
        /// firebasestorage.googleapis.com (Cloud Storage for Firebase)
        /// firebasedatabase.googleapis.com (Firebase Realtime Database)
        /// firestore.googleapis.com (Cloud Firestore)
        /// identitytoolkit.googleapis.com (Authentication)
        ///
        ///
        /// - - -
        pub service_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AppCheckServiceConfigArgs,
    ) -> AppCheckServiceConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enforcement_mode_binding = args.enforcement_mode.get_inner();
        let project_binding = args.project.get_inner();
        let service_id_binding = args.service_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firebase/appCheckServiceConfig:AppCheckServiceConfig".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enforcementMode".into(),
                    value: &enforcement_mode_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "serviceId".into(),
                    value: &service_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "enforcementMode".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "serviceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AppCheckServiceConfigResult {
            enforcement_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enforcementMode").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceId").unwrap(),
            ),
        }
    }
}
