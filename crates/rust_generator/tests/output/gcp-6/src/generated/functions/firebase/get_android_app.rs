pub mod get_android_app {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAndroidAppArgs {
        /// The app_id of name of the Firebase androidApp.
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
    pub struct GetAndroidAppResult {
        pub api_key_id: pulumi_gestalt_rust::Output<String>,
        /// Immutable. The globally unique, Firebase-assigned identifier of the AndroidApp.
        /// This identifier should be treated as an opaque token, as the data format is not specified.
        pub app_id: pulumi_gestalt_rust::Output<String>,
        pub deletion_policy: pulumi_gestalt_rust::Output<String>,
        /// The user-assigned display name of the AndroidApp.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// This checksum is computed by the server based on the value of other fields, and it may be sent
        /// with update requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The fully qualified resource name of the AndroidApp, for example:
        /// projects/projectId/androidApps/appId
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The canonical package name of the Android app as would appear in the Google Play Developer Console.
        pub package_name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        /// The SHA1 certificate hashes for the AndroidApp.
        pub sha1_hashes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The SHA256 certificate hashes for the AndroidApp.
        pub sha256_hashes: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAndroidAppArgs,
    ) -> GetAndroidAppResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let app_id_binding = args.app_id.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:firebase/getAndroidApp:getAndroidApp".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appId".into(),
                    value: &app_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAndroidAppResult {
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
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
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
