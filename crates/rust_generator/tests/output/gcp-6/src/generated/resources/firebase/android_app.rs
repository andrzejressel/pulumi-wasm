/// ## Example Usage
///
/// ### Firebase Android App Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = android_app::create(
///         "basic",
///         AndroidAppArgs::builder()
///             .display_name("Display Name Basic")
///             .package_name("android.package.app")
///             .project("my-project-name")
///             .sha_1_hashes(vec!["2145bdf698b8715039bd0e83f2069bed435ac21c",])
///             .sha_256_hashes(
///                 vec!["2145bdf698b8715039bd0e83f2069bed435ac21ca1b2c3d4e5f6123456789abc",],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firebase Android App Custom Api Key
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let android = api_key::create(
///         "android",
///         ApiKeyArgs::builder()
///             .display_name("Display Name")
///             .name("api-key")
///             .project("my-project-name")
///             .restrictions(
///                 ApiKeyRestrictions::builder()
///                     .androidKeyRestrictions(
///                         ApiKeyRestrictionsAndroidKeyRestrictions::builder()
///                             .allowedApplications(
///                                 vec![
///                                     ApiKeyRestrictionsAndroidKeyRestrictionsAllowedApplication::builder()
///                                     .packageName("android.package.app")
///                                     .sha1Fingerprint("2145bdf698b8715039bd0e83f2069bed435ac21c")
///                                     .build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let default = android_app::create(
///         "default",
///         AndroidAppArgs::builder()
///             .api_key_id("${android.uid}")
///             .display_name("Display Name")
///             .package_name("android.package.app")
///             .project("my-project-name")
///             .sha_1_hashes(vec!["2145bdf698b8715039bd0e83f2069bed435ac21c",])
///             .sha_256_hashes(
///                 vec!["2145bdf698b8715039bd0e83f2069bed435ac21ca1b2c3d4e5f6123456789abc",],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// AndroidApp can be imported using any of these accepted formats:
///
/// * `{{project}} projects/{{project}}/androidApps/{{app_id}}`
///
/// * `projects/{{project}}/androidApps/{{app_id}}`
///
/// * `{{project}}/{{project}}/{{app_id}}`
///
/// * `androidApps/{{app_id}}`
///
/// * `{{app_id}}`
///
/// When using the `pulumi import` command, AndroidApp can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/androidApp:AndroidApp default "{{project}} projects/{{project}}/androidApps/{{app_id}}"
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/androidApp:AndroidApp default projects/{{project}}/androidApps/{{app_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/androidApp:AndroidApp default {{project}}/{{project}}/{{app_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/androidApp:AndroidApp default androidApps/{{app_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/androidApp:AndroidApp default {{app_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod android_app {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AndroidAppArgs {
        /// The globally unique, Google-assigned identifier (UID) for the Firebase API key associated with the AndroidApp.
        /// If apiKeyId is not set during creation, then Firebase automatically associates an apiKeyId with the AndroidApp.
        /// This auto-associated key may be an existing valid key or, if no valid key exists, a new one will be provisioned.
        #[builder(into, default)]
        pub api_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub deletion_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The user-assigned display name of the AndroidApp.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The canonical package name of the Android app as would appear in the Google Play
        /// Developer Console.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub package_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The SHA1 certificate hashes for the AndroidApp.
        #[builder(into, default)]
        pub sha1_hashes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The SHA256 certificate hashes for the AndroidApp.
        #[builder(into, default)]
        pub sha256_hashes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct AndroidAppResult {
        /// The globally unique, Google-assigned identifier (UID) for the Firebase API key associated with the AndroidApp.
        /// If apiKeyId is not set during creation, then Firebase automatically associates an apiKeyId with the AndroidApp.
        /// This auto-associated key may be an existing valid key or, if no valid key exists, a new one will be provisioned.
        pub api_key_id: pulumi_gestalt_rust::Output<String>,
        /// The globally unique, Firebase-assigned identifier of the AndroidApp.
        /// This identifier should be treated as an opaque token, as the data format is not specified.
        pub app_id: pulumi_gestalt_rust::Output<String>,
        pub deletion_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The user-assigned display name of the AndroidApp.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// This checksum is computed by the server based on the value of other fields, and it may be sent
        /// with update requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The fully qualified resource name of the AndroidApp, for example:
        /// projects/projectId/androidApps/appId
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The canonical package name of the Android app as would appear in the Google Play
        /// Developer Console.
        ///
        ///
        /// - - -
        pub package_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The SHA1 certificate hashes for the AndroidApp.
        pub sha1_hashes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The SHA256 certificate hashes for the AndroidApp.
        pub sha256_hashes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AndroidAppArgs,
    ) -> AndroidAppResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let api_key_id_binding = args.api_key_id.get_output(context).get_inner();
        let deletion_policy_binding = args
            .deletion_policy
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let package_name_binding = args.package_name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let sha1_hashes_binding = args.sha1_hashes.get_output(context).get_inner();
        let sha256_hashes_binding = args.sha256_hashes.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firebase/androidApp:AndroidApp".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiKeyId".into(),
                    value: &api_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "deletionPolicy".into(),
                    value: &deletion_policy_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "packageName".into(),
                    value: &package_name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "sha1Hashes".into(),
                    value: &sha1_hashes_binding,
                },
                register_interface::ObjectField {
                    name: "sha256Hashes".into(),
                    value: &sha256_hashes_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AndroidAppResult {
            api_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiKeyId"),
            ),
            app_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appId"),
            ),
            deletion_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionPolicy"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            package_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("packageName"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            sha1_hashes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sha1Hashes"),
            ),
            sha256_hashes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sha256Hashes"),
            ),
        }
    }
}
