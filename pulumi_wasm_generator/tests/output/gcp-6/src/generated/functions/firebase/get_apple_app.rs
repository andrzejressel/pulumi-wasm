pub mod get_apple_app {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAppleAppArgs {
        /// The app_id of name of the Firebase iosApp.
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
    pub struct GetAppleAppResult {
        pub api_key_id: pulumi_wasm_rust::Output<String>,
        /// Immutable. The globally unique, Firebase-assigned identifier of the App.
        /// This identifier should be treated as an opaque token, as the data format is not specified.
        pub app_id: pulumi_wasm_rust::Output<String>,
        /// The automatically generated Apple ID assigned to the Apple app by Apple in the Apple App Store.
        pub app_store_id: pulumi_wasm_rust::Output<String>,
        /// The canonical bundle ID of the Apple app as it would appear in the Apple AppStore.
        pub bundle_id: pulumi_wasm_rust::Output<String>,
        pub deletion_policy: pulumi_wasm_rust::Output<String>,
        /// The user-assigned display name of the App.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The fully qualified resource name of the App, for example:
        /// projects/projectId/iosApps/appId
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The Apple Developer Team ID associated with the App in the App Store.
        pub team_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAppleAppArgs) -> GetAppleAppResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_id_binding = args.app_id.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:firebase/getAppleApp:getAppleApp".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiKeyId".into(),
                },
                register_interface::ResultField {
                    name: "appId".into(),
                },
                register_interface::ResultField {
                    name: "appStoreId".into(),
                },
                register_interface::ResultField {
                    name: "bundleId".into(),
                },
                register_interface::ResultField {
                    name: "deletionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "teamId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAppleAppResult {
            api_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiKeyId").unwrap(),
            ),
            app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appId").unwrap(),
            ),
            app_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appStoreId").unwrap(),
            ),
            bundle_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bundleId").unwrap(),
            ),
            deletion_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionPolicy").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            team_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("teamId").unwrap(),
            ),
        }
    }
}
