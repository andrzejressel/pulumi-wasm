pub mod get_project_service_account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProjectServiceAccountArgs {
        /// The project the unique service account was created for. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The project the lookup originates from. This field is used if you are making the request
        /// from a different account than the one you are finding the service account for.
        #[builder(into, default)]
        pub user_project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetProjectServiceAccountResult {
        /// The email address of the service account. This value is often used to refer to the service account
        /// in order to grant IAM permissions.
        pub email_address: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Identity of the service account in the form `serviceAccount:{email_address}`. This value is often used to refer to the service account in order to grant IAM permissions.
        pub member: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        pub user_project: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetProjectServiceAccountArgs) -> GetProjectServiceAccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let project_binding = args.project.get_inner();
        let user_project_binding = args.user_project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:storage/getProjectServiceAccount:getProjectServiceAccount"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "userProject".into(),
                    value: &user_project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "emailAddress".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "member".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "userProject".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetProjectServiceAccountResult {
            email_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailAddress").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            member: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("member").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            user_project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userProject").unwrap(),
            ),
        }
    }
}
