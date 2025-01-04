/// Provides an Amazon Managed Grafana workspace API Key resource.
///
/// ## Example Usage
///
/// ### Basic configuration
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let key = workspace_api_key::create(
///         "key",
///         WorkspaceApiKeyArgs::builder()
///             .key_name("test-key")
///             .key_role("VIEWER")
///             .seconds_to_live(3600)
///             .workspace_id("${test.id}")
///             .build_struct(),
///     );
/// }
/// ```
pub mod workspace_api_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceApiKeyArgs {
        /// Specifies the name of the API key. Key names must be unique to the workspace.
        #[builder(into)]
        pub key_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the permission level of the API key. Valid values are `VIEWER`, `EDITOR`, or `ADMIN`.
        #[builder(into)]
        pub key_role: pulumi_wasm_rust::Output<String>,
        /// Specifies the time in seconds until the API key expires. Keys can be valid for up to 30 days.
        #[builder(into)]
        pub seconds_to_live: pulumi_wasm_rust::Output<i32>,
        /// The ID of the workspace that the API key is valid for.
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceApiKeyResult {
        /// The key token in JSON format. Use this value as a bearer token to authenticate HTTP requests to the workspace.
        pub key: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the API key. Key names must be unique to the workspace.
        pub key_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the permission level of the API key. Valid values are `VIEWER`, `EDITOR`, or `ADMIN`.
        pub key_role: pulumi_wasm_rust::Output<String>,
        /// Specifies the time in seconds until the API key expires. Keys can be valid for up to 30 days.
        pub seconds_to_live: pulumi_wasm_rust::Output<i32>,
        /// The ID of the workspace that the API key is valid for.
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WorkspaceApiKeyArgs) -> WorkspaceApiKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let key_name_binding = args.key_name.get_inner();
        let key_role_binding = args.key_role.get_inner();
        let seconds_to_live_binding = args.seconds_to_live.get_inner();
        let workspace_id_binding = args.workspace_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:grafana/workspaceApiKey:WorkspaceApiKey".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "keyName".into(),
                    value: &key_name_binding,
                },
                register_interface::ObjectField {
                    name: "keyRole".into(),
                    value: &key_role_binding,
                },
                register_interface::ObjectField {
                    name: "secondsToLive".into(),
                    value: &seconds_to_live_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "key".into(),
                },
                register_interface::ResultField {
                    name: "keyName".into(),
                },
                register_interface::ResultField {
                    name: "keyRole".into(),
                },
                register_interface::ResultField {
                    name: "secondsToLive".into(),
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
        WorkspaceApiKeyResult {
            key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("key").unwrap(),
            ),
            key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyName").unwrap(),
            ),
            key_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyRole").unwrap(),
            ),
            seconds_to_live: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondsToLive").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
        }
    }
}
