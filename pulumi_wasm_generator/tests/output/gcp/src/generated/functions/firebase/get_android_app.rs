pub mod get_android_app {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAndroidAppArgs {
        /// The app_id of name of the Firebase androidApp.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub app_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAndroidAppResult {
        pub api_key_id: pulumi_wasm_rust::Output<String>,
        /// Immutable. The globally unique, Firebase-assigned identifier of the AndroidApp.
        /// This identifier should be treated as an opaque token, as the data format is not specified.
        pub app_id: pulumi_wasm_rust::Output<String>,
        pub deletion_policy: pulumi_wasm_rust::Output<String>,
        /// The user-assigned display name of the AndroidApp.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// This checksum is computed by the server based on the value of other fields, and it may be sent
        /// with update requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The fully qualified resource name of the AndroidApp, for example:
        /// projects/projectId/androidApps/appId
        pub name: pulumi_wasm_rust::Output<String>,
        /// The canonical package name of the Android app as would appear in the Google Play Developer Console.
        pub package_name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The SHA1 certificate hashes for the AndroidApp.
        pub sha1_hashes: pulumi_wasm_rust::Output<Vec<String>>,
        /// The SHA256 certificate hashes for the AndroidApp.
        pub sha256_hashes: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAndroidAppArgs) -> GetAndroidAppResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_id_binding = args.app_id.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:firebase/getAndroidApp:getAndroidApp".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiKeyId".into(),
                },
                register_interface::ResultField {
                    name: "appId".into(),
                },
                register_interface::ResultField {
                    name: "deletionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "packageName".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "sha1Hashes".into(),
                },
                register_interface::ResultField {
                    name: "sha256Hashes".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAndroidAppResult {
            api_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiKeyId").unwrap(),
            ),
            app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appId").unwrap(),
            ),
            deletion_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionPolicy").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            package_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("packageName").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            sha1_hashes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sha1Hashes").unwrap(),
            ),
            sha256_hashes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sha256Hashes").unwrap(),
            ),
        }
    }
}
