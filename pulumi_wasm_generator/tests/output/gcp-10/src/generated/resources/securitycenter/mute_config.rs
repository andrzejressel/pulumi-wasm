/// Mute Findings is a volume management feature in Security Command Center
/// that lets you manually or programmatically hide irrelevant findings,
/// and create filters to automatically silence existing and future
/// findings based on criteria you specify.
///
///
/// To get more information about MuteConfig, see:
///
/// * [API documentation](https://cloud.google.com/security-command-center/docs/reference/rest/v1/organizations.muteConfigs)
///
/// ## Example Usage
///
/// ### Scc Mute Config
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = mute_config::create(
///         "default",
///         MuteConfigArgs::builder()
///             .description("My Mute Config")
///             .filter("category: \"OS_VULNERABILITY\"")
///             .mute_config_id("my-config")
///             .parent("organizations/123456789")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// MuteConfig can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, MuteConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securitycenter/muteConfig:MuteConfig default {{name}}
/// ```
///
pub mod mute_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MuteConfigArgs {
        /// A description of the mute config.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// An expression that defines the filter to apply across create/update
        /// events of findings. While creating a filter string, be mindful of
        /// the scope in which the mute configuration is being created. E.g.,
        /// If a filter contains project = X but is created under the
        /// project = Y scope, it might not match any findings.
        #[builder(into)]
        pub filter: pulumi_wasm_rust::Output<String>,
        /// Unique identifier provided by the client within the parent scope.
        #[builder(into)]
        pub mute_config_id: pulumi_wasm_rust::Output<String>,
        /// Resource name of the new mute configs's parent. Its format is
        /// "organizations/[organization_id]", "folders/[folder_id]", or
        /// "projects/[project_id]".
        ///
        ///
        /// - - -
        #[builder(into)]
        pub parent: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct MuteConfigResult {
        /// The time at which the mute config was created. This field is set by
        /// the server and will be ignored if provided on config creation.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// A description of the mute config.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// An expression that defines the filter to apply across create/update
        /// events of findings. While creating a filter string, be mindful of
        /// the scope in which the mute configuration is being created. E.g.,
        /// If a filter contains project = X but is created under the
        /// project = Y scope, it might not match any findings.
        pub filter: pulumi_wasm_rust::Output<String>,
        /// Email address of the user who last edited the mute config. This
        /// field is set by the server and will be ignored if provided on
        /// config creation or update.
        pub most_recent_editor: pulumi_wasm_rust::Output<String>,
        /// Unique identifier provided by the client within the parent scope.
        pub mute_config_id: pulumi_wasm_rust::Output<String>,
        /// Name of the mute config. Its format is
        /// organizations/{organization}/muteConfigs/{configId},
        /// folders/{folder}/muteConfigs/{configId},
        /// or projects/{project}/muteConfigs/{configId}
        pub name: pulumi_wasm_rust::Output<String>,
        /// Resource name of the new mute configs's parent. Its format is
        /// "organizations/[organization_id]", "folders/[folder_id]", or
        /// "projects/[project_id]".
        ///
        ///
        /// - - -
        pub parent: pulumi_wasm_rust::Output<String>,
        /// Output only. The most recent time at which the mute config was
        /// updated. This field is set by the server and will be ignored if
        /// provided on config creation or update.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MuteConfigArgs) -> MuteConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let filter_binding = args.filter.get_inner();
        let mute_config_id_binding = args.mute_config_id.get_inner();
        let parent_binding = args.parent.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securitycenter/muteConfig:MuteConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "muteConfigId".into(),
                    value: &mute_config_id_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "filter".into(),
                },
                register_interface::ResultField {
                    name: "mostRecentEditor".into(),
                },
                register_interface::ResultField {
                    name: "muteConfigId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MuteConfigResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filter").unwrap(),
            ),
            most_recent_editor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mostRecentEditor").unwrap(),
            ),
            mute_config_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("muteConfigId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
