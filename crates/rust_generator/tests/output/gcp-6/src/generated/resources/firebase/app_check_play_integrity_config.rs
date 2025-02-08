/// An app's Play Integrity configuration object. Note that your registered SHA-256 certificate fingerprints are used to validate tokens issued by the Play Integrity API.
/// Make sure your `gcp.firebase.AndroidApp` has at least one `sha256_hashes` present.
///
///
/// To get more information about PlayIntegrityConfig, see:
///
/// * [API documentation](https://firebase.google.com/docs/reference/appcheck/rest/v1/projects.apps.playIntegrityConfig)
/// * How-to Guides
///     * [Official Documentation](https://firebase.google.com/docs/app-check)
///
/// ## Example Usage
///
/// ### Firebase App Check Play Integrity Config Minimal
///
///
/// ```yaml
/// resources:
///   # Enables the Play Integrity API
///   playIntegrity:
///     type: gcp:projects:Service
///     name: play_integrity
///     properties:
///       project: my-project-name
///       service: playintegrity.googleapis.com
///       disableOnDestroy: false
///   default:
///     type: gcp:firebase:AndroidApp
///     properties:
///       project: my-project-name
///       displayName: Play Integrity app
///       packageName: package.name.playintegrity
///       sha1Hashes:
///         - 2145bdf698b8715039bd0e83f2069bed435ac21c
///       sha256Hashes:
///         - 2145bdf698b8715039bd0e83f2069bed435ac21ca1b2c3d4e5f6123456789abc
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
///   defaultAppCheckPlayIntegrityConfig:
///     type: gcp:firebase:AppCheckPlayIntegrityConfig
///     name: default
///     properties:
///       project: my-project-name
///       appId: ${default.appId}
///     options:
///       dependsOn:
///         - ${wait30s}
/// ```
/// ### Firebase App Check Play Integrity Config Full
///
///
/// ```yaml
/// resources:
///   # Enables the Play Integrity API
///   playIntegrity:
///     type: gcp:projects:Service
///     name: play_integrity
///     properties:
///       project: my-project-name
///       service: playintegrity.googleapis.com
///       disableOnDestroy: false
///   default:
///     type: gcp:firebase:AndroidApp
///     properties:
///       project: my-project-name
///       displayName: Play Integrity app
///       packageName: package.name.playintegrity
///       sha1Hashes:
///         - 2145bdf698b8715039bd0e83f2069bed435ac21c
///       sha256Hashes:
///         - 2145bdf698b8715039bd0e83f2069bed435ac21ca1b2c3d4e5f6123456789abc
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
///   defaultAppCheckPlayIntegrityConfig:
///     type: gcp:firebase:AppCheckPlayIntegrityConfig
///     name: default
///     properties:
///       project: my-project-name
///       appId: ${default.appId}
///       tokenTtl: 7200s
///     options:
///       dependsOn:
///         - ${wait30s}
/// ```
///
/// ## Import
///
/// PlayIntegrityConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/apps/{{app_id}}/playIntegrityConfig`
///
/// * `{{project}}/{{app_id}}`
///
/// * `{{app_id}}`
///
/// When using the `pulumi import` command, PlayIntegrityConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckPlayIntegrityConfig:AppCheckPlayIntegrityConfig default projects/{{project}}/apps/{{app_id}}/playIntegrityConfig
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckPlayIntegrityConfig:AppCheckPlayIntegrityConfig default {{project}}/{{app_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckPlayIntegrityConfig:AppCheckPlayIntegrityConfig default {{app_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod app_check_play_integrity_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppCheckPlayIntegrityConfigArgs {
        /// The ID of an
        /// [Android App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.androidApps#AndroidApp.FIELDS.app_id).
        ///
        ///
        /// - - -
        #[builder(into)]
        pub app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the duration for which App Check tokens exchanged from Play Integrity artifacts will be valid.
        /// If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        #[builder(into, default)]
        pub token_ttl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AppCheckPlayIntegrityConfigResult {
        /// The ID of an
        /// [Android App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.androidApps#AndroidApp.FIELDS.app_id).
        ///
        ///
        /// - - -
        pub app_id: pulumi_gestalt_rust::Output<String>,
        /// The relative resource name of the Play Integrity configuration object
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Specifies the duration for which App Check tokens exchanged from Play Integrity artifacts will be valid.
        /// If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        pub token_ttl: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AppCheckPlayIntegrityConfigArgs,
    ) -> AppCheckPlayIntegrityConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let app_id_binding = args.app_id.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let token_ttl_binding = args.token_ttl.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firebase/appCheckPlayIntegrityConfig:AppCheckPlayIntegrityConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appId".into(),
                    value: &app_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "tokenTtl".into(),
                    value: &token_ttl_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AppCheckPlayIntegrityConfigResult {
            app_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            token_ttl: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tokenTtl"),
            ),
        }
    }
}
