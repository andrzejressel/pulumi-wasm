pub mod get_web_app_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWebAppConfigArgs {
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// the id of the firebase web app
        ///
        /// - - -
        #[builder(into)]
        pub web_app_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetWebAppConfigResult {
        /// The API key associated with the web App.
        pub api_key: pulumi_wasm_rust::Output<String>,
        /// The domain Firebase Auth configures for OAuth redirects, in the format:
        /// projectId.firebaseapp.com
        pub auth_domain: pulumi_wasm_rust::Output<String>,
        /// The default Firebase Realtime Database URL.
        pub database_url: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        /// The ID of the project's default GCP resource location. The location is one of the available GCP resource
        /// locations.
        /// This field is omitted if the default GCP resource location has not been finalized yet. To set your project's
        /// default GCP resource location, call defaultLocation.finalize after you add Firebase services to your project.
        pub location_id: pulumi_wasm_rust::Output<String>,
        /// The unique Google-assigned identifier of the Google Analytics web stream associated with the Firebase Web App.
        /// Firebase SDKs use this ID to interact with Google Analytics APIs.
        /// This field is only present if the App is linked to a web stream in a Google Analytics App + Web property.
        /// Learn more about this ID and Google Analytics web streams in the Analytics documentation.
        /// To generate a measurementId and link the Web App with a Google Analytics web stream,
        /// call projects.addGoogleAnalytics.
        pub measurement_id: pulumi_wasm_rust::Output<String>,
        /// The sender ID for use with Firebase Cloud Messaging.
        pub messaging_sender_id: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The default Cloud Storage for Firebase storage bucket name.
        pub storage_bucket: pulumi_wasm_rust::Output<String>,
        pub web_app_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetWebAppConfigArgs,
    ) -> GetWebAppConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let project_binding = args.project.get_output(context).get_inner();
        let web_app_id_binding = args.web_app_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:firebase/getWebAppConfig:getWebAppConfig".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "webAppId".into(),
                    value: &web_app_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiKey".into(),
                },
                register_interface::ResultField {
                    name: "authDomain".into(),
                },
                register_interface::ResultField {
                    name: "databaseUrl".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "locationId".into(),
                },
                register_interface::ResultField {
                    name: "measurementId".into(),
                },
                register_interface::ResultField {
                    name: "messagingSenderId".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "storageBucket".into(),
                },
                register_interface::ResultField {
                    name: "webAppId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetWebAppConfigResult {
            api_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiKey").unwrap(),
            ),
            auth_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authDomain").unwrap(),
            ),
            database_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseUrl").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locationId").unwrap(),
            ),
            measurement_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("measurementId").unwrap(),
            ),
            messaging_sender_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("messagingSenderId").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            storage_bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageBucket").unwrap(),
            ),
            web_app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webAppId").unwrap(),
            ),
        }
    }
}
