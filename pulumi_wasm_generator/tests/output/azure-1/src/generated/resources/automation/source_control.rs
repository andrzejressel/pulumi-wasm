/// Manages an Automation Source Control.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .location("${example.location}")
///             .name("example-account")
///             .resource_group_name("${example.name}")
///             .sku_name("Basic")
///             .build_struct(),
///     );
///     let exampleSourceControl = source_control::create(
///         "exampleSourceControl",
///         SourceControlArgs::builder()
///             .automation_account_id("${exampleAccount.id}")
///             .branch("main")
///             .folder_path("runbook")
///             .name("example")
///             .repository_url("https://github.com/foo/bat.git")
///             .security(
///                 SourceControlSecurity::builder()
///                     .token("ghp_xxx")
///                     .tokenType("PersonalAccessToken")
///                     .build_struct(),
///             )
///             .source_control_type("GitHub")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Automations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/sourceControl:SourceControl example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/sourceControls/sc1
/// ```
///
pub mod source_control {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SourceControlArgs {
        /// Whether auto async the Source Control.
        #[builder(into, default)]
        pub automatic_sync: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of Automation Account to manage this Source Control. Changing this forces a new Automation Source Control to be created.
        #[builder(into)]
        pub automation_account_id: pulumi_wasm_rust::Output<String>,
        /// Specify the repo branch of the Source Control. Empty value is valid only for `VsoTfvc`.
        #[builder(into, default)]
        pub branch: pulumi_wasm_rust::Output<Option<String>>,
        /// A short description of the Source Control.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The folder path of the source control. This Path must be relative.
        #[builder(into)]
        pub folder_path: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Automation Source Control. Changing this forces a new Automation Source Control to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether auto publish the Source Control. Defaults to `true`.
        #[builder(into, default)]
        pub publish_runbook_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Repository URL of the source control.
        #[builder(into)]
        pub repository_url: pulumi_wasm_rust::Output<String>,
        /// A `security` block as defined below.
        #[builder(into)]
        pub security: pulumi_wasm_rust::Output<
            super::super::types::automation::SourceControlSecurity,
        >,
        /// The source type of Source Control, possible vaules are `VsoGit`, `VsoTfvc` and `GitHub`, and the value is case sensitive.
        #[builder(into)]
        pub source_control_type: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SourceControlResult {
        /// Whether auto async the Source Control.
        pub automatic_sync: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of Automation Account to manage this Source Control. Changing this forces a new Automation Source Control to be created.
        pub automation_account_id: pulumi_wasm_rust::Output<String>,
        /// Specify the repo branch of the Source Control. Empty value is valid only for `VsoTfvc`.
        pub branch: pulumi_wasm_rust::Output<Option<String>>,
        /// A short description of the Source Control.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The folder path of the source control. This Path must be relative.
        pub folder_path: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Automation Source Control. Changing this forces a new Automation Source Control to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether auto publish the Source Control. Defaults to `true`.
        pub publish_runbook_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Repository URL of the source control.
        pub repository_url: pulumi_wasm_rust::Output<String>,
        /// A `security` block as defined below.
        pub security: pulumi_wasm_rust::Output<
            super::super::types::automation::SourceControlSecurity,
        >,
        /// The source type of Source Control, possible vaules are `VsoGit`, `VsoTfvc` and `GitHub`, and the value is case sensitive.
        pub source_control_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SourceControlArgs) -> SourceControlResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automatic_sync_binding = args.automatic_sync.get_inner();
        let automation_account_id_binding = args.automation_account_id.get_inner();
        let branch_binding = args.branch.get_inner();
        let description_binding = args.description.get_inner();
        let folder_path_binding = args.folder_path.get_inner();
        let name_binding = args.name.get_inner();
        let publish_runbook_enabled_binding = args.publish_runbook_enabled.get_inner();
        let repository_url_binding = args.repository_url.get_inner();
        let security_binding = args.security.get_inner();
        let source_control_type_binding = args.source_control_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/sourceControl:SourceControl".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automaticSync".into(),
                    value: &automatic_sync_binding,
                },
                register_interface::ObjectField {
                    name: "automationAccountId".into(),
                    value: &automation_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "branch".into(),
                    value: &branch_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "folderPath".into(),
                    value: &folder_path_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publishRunbookEnabled".into(),
                    value: &publish_runbook_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "repositoryUrl".into(),
                    value: &repository_url_binding,
                },
                register_interface::ObjectField {
                    name: "security".into(),
                    value: &security_binding,
                },
                register_interface::ObjectField {
                    name: "sourceControlType".into(),
                    value: &source_control_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "automaticSync".into(),
                },
                register_interface::ResultField {
                    name: "automationAccountId".into(),
                },
                register_interface::ResultField {
                    name: "branch".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "folderPath".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publishRunbookEnabled".into(),
                },
                register_interface::ResultField {
                    name: "repositoryUrl".into(),
                },
                register_interface::ResultField {
                    name: "security".into(),
                },
                register_interface::ResultField {
                    name: "sourceControlType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SourceControlResult {
            automatic_sync: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automaticSync").unwrap(),
            ),
            automation_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automationAccountId").unwrap(),
            ),
            branch: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("branch").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            folder_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("folderPath").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            publish_runbook_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publishRunbookEnabled").unwrap(),
            ),
            repository_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryUrl").unwrap(),
            ),
            security: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("security").unwrap(),
            ),
            source_control_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceControlType").unwrap(),
            ),
        }
    }
}
