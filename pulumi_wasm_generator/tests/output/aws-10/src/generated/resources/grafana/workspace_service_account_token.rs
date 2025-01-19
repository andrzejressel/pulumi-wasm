/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workspace_service_account::create(
///         "example",
///         WorkspaceServiceAccountArgs::builder()
///             .grafana_role("ADMIN")
///             .name("example-admin")
///             .workspace_id("${exampleAwsGrafanaWorkspace.id}")
///             .build_struct(),
///     );
///     let exampleWorkspaceServiceAccountToken = workspace_service_account_token::create(
///         "exampleWorkspaceServiceAccountToken",
///         WorkspaceServiceAccountTokenArgs::builder()
///             .name("example-key")
///             .seconds_to_live(3600)
///             .service_account_id("${example.serviceAccountId}")
///             .workspace_id("${exampleAwsGrafanaWorkspace.id}")
///             .build_struct(),
///     );
/// }
/// ```
pub mod workspace_service_account_token {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceServiceAccountTokenArgs {
        /// A name for the token to create. The name must be unique within the workspace.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Sets how long the token will be valid, in seconds. You can set the time up to 30 days in the future.
        #[builder(into)]
        pub seconds_to_live: pulumi_wasm_rust::Output<i32>,
        /// The ID of the service account for which to create a token.
        #[builder(into)]
        pub service_account_id: pulumi_wasm_rust::Output<String>,
        /// The Grafana workspace with which the service account token is associated.
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceServiceAccountTokenResult {
        /// Specifies when the service account token was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// Specifies when the service account token will expire.
        pub expires_at: pulumi_wasm_rust::Output<String>,
        /// The key for the service account token. Used when making calls to the Grafana HTTP APIs to authenticate and authorize the requests.
        pub key: pulumi_wasm_rust::Output<String>,
        /// A name for the token to create. The name must be unique within the workspace.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Sets how long the token will be valid, in seconds. You can set the time up to 30 days in the future.
        pub seconds_to_live: pulumi_wasm_rust::Output<i32>,
        /// The ID of the service account for which to create a token.
        pub service_account_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the service account token in the given Grafana workspace.
        pub service_account_token_id: pulumi_wasm_rust::Output<String>,
        /// The Grafana workspace with which the service account token is associated.
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: WorkspaceServiceAccountTokenArgs,
    ) -> WorkspaceServiceAccountTokenResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let seconds_to_live_binding = args.seconds_to_live.get_inner();
        let service_account_id_binding = args.service_account_id.get_inner();
        let workspace_id_binding = args.workspace_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:grafana/workspaceServiceAccountToken:WorkspaceServiceAccountToken"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "secondsToLive".into(),
                    value: &seconds_to_live_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccountId".into(),
                    value: &service_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "expiresAt".into(),
                },
                register_interface::ResultField {
                    name: "key".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "secondsToLive".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccountId".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccountTokenId".into(),
                },
                register_interface::ResultField {
                    name: "workspaceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkspaceServiceAccountTokenResult {
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            expires_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiresAt").unwrap(),
            ),
            key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("key").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            seconds_to_live: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondsToLive").unwrap(),
            ),
            service_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccountId").unwrap(),
            ),
            service_account_token_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccountTokenId").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
        }
    }
}
