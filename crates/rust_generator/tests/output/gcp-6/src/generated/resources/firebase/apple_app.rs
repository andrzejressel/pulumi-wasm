/// ## Example Usage
///
/// ### Firebase Apple App Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = apple_app::create(
///         "default",
///         AppleAppArgs::builder()
///             .bundle_id("apple.app.12345")
///             .display_name("Display Name Basic")
///             .project("my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firebase Apple App Full
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let apple = api_key::create(
///         "apple",
///         ApiKeyArgs::builder()
///             .display_name("Display Name Full")
///             .name("api-key")
///             .project("my-project-name")
///             .restrictions(
///                 ApiKeyRestrictions::builder()
///                     .iosKeyRestrictions(
///                         ApiKeyRestrictionsIosKeyRestrictions::builder()
///                             .allowedBundleIds(vec!["apple.app.12345",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let full = apple_app::create(
///         "full",
///         AppleAppArgs::builder()
///             .api_key_id("${apple.uid}")
///             .app_store_id("12345")
///             .bundle_id("apple.app.12345")
///             .display_name("Display Name Full")
///             .project("my-project-name")
///             .team_id("9987654321")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// AppleApp can be imported using any of these accepted formats:
///
/// * `{{project}} projects/{{project}}/iosApps/{{app_id}}`
///
/// * `projects/{{project}}/iosApps/{{app_id}}`
///
/// * `{{project}}/{{project}}/{{app_id}}`
///
/// * `iosApps/{{app_id}}`
///
/// * `{{app_id}}`
///
/// When using the `pulumi import` command, AppleApp can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/appleApp:AppleApp default "{{project}} projects/{{project}}/iosApps/{{app_id}}"
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/appleApp:AppleApp default projects/{{project}}/iosApps/{{app_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/appleApp:AppleApp default {{project}}/{{project}}/{{app_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/appleApp:AppleApp default iosApps/{{app_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/appleApp:AppleApp default {{app_id}}
/// ```
///
pub mod apple_app {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppleAppArgs {
        /// The globally unique, Google-assigned identifier (UID) for the Firebase API key associated with the AppleApp.
        /// If apiKeyId is not set during creation, then Firebase automatically associates an apiKeyId with the AppleApp.
        /// This auto-associated key may be an existing valid key or, if no valid key exists, a new one will be provisioned.
        #[builder(into, default)]
        pub api_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The automatically generated Apple ID assigned to the Apple app by Apple in the Apple App Store.
        #[builder(into, default)]
        pub app_store_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The canonical bundle ID of the Apple app as it would appear in the Apple AppStore.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub bundle_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub deletion_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The user-assigned display name of the App.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Apple Developer Team ID associated with the App in the App Store.
        #[builder(into, default)]
        pub team_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AppleAppResult {
        /// The globally unique, Google-assigned identifier (UID) for the Firebase API key associated with the AppleApp.
        /// If apiKeyId is not set during creation, then Firebase automatically associates an apiKeyId with the AppleApp.
        /// This auto-associated key may be an existing valid key or, if no valid key exists, a new one will be provisioned.
        pub api_key_id: pulumi_gestalt_rust::Output<String>,
        /// The globally unique, Firebase-assigned identifier of the App.
        /// This identifier should be treated as an opaque token, as the data format is not specified.
        pub app_id: pulumi_gestalt_rust::Output<String>,
        /// The automatically generated Apple ID assigned to the Apple app by Apple in the Apple App Store.
        pub app_store_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The canonical bundle ID of the Apple app as it would appear in the Apple AppStore.
        ///
        ///
        /// - - -
        pub bundle_id: pulumi_gestalt_rust::Output<String>,
        pub deletion_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The user-assigned display name of the App.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The fully qualified resource name of the App, for example:
        /// projects/projectId/iosApps/appId
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The Apple Developer Team ID associated with the App in the App Store.
        pub team_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AppleAppArgs,
    ) -> AppleAppResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let api_key_id_binding = args.api_key_id.get_output(context).get_inner();
        let app_store_id_binding = args.app_store_id.get_output(context).get_inner();
        let bundle_id_binding = args.bundle_id.get_output(context).get_inner();
        let deletion_policy_binding = args
            .deletion_policy
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let team_id_binding = args.team_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firebase/appleApp:AppleApp".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiKeyId".into(),
                    value: &api_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "appStoreId".into(),
                    value: &app_store_id_binding,
                },
                register_interface::ObjectField {
                    name: "bundleId".into(),
                    value: &bundle_id_binding,
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
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "teamId".into(),
                    value: &team_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AppleAppResult {
            api_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiKeyId"),
            ),
            app_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appId"),
            ),
            app_store_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appStoreId"),
            ),
            bundle_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bundleId"),
            ),
            deletion_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionPolicy"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            team_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("teamId"),
            ),
        }
    }
}
