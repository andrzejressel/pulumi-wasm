/// Manages an Automation Source Control.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod source_control {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SourceControlArgs {
        /// Whether auto async the Source Control.
        #[builder(into, default)]
        pub automatic_sync: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of Automation Account to manage this Source Control. Changing this forces a new Automation Source Control to be created.
        #[builder(into)]
        pub automation_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specify the repo branch of the Source Control. Empty value is valid only for `VsoTfvc`.
        #[builder(into, default)]
        pub branch: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A short description of the Source Control.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The folder path of the source control. This Path must be relative.
        #[builder(into)]
        pub folder_path: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Automation Source Control. Changing this forces a new Automation Source Control to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether auto publish the Source Control. Defaults to `true`.
        #[builder(into, default)]
        pub publish_runbook_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Repository URL of the source control.
        #[builder(into)]
        pub repository_url: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `security` block as defined below.
        #[builder(into)]
        pub security: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::automation::SourceControlSecurity,
        >,
        /// The source type of Source Control, possible vaules are `VsoGit`, `VsoTfvc` and `GitHub`, and the value is case sensitive.
        #[builder(into)]
        pub source_control_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SourceControlResult {
        /// Whether auto async the Source Control.
        pub automatic_sync: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of Automation Account to manage this Source Control. Changing this forces a new Automation Source Control to be created.
        pub automation_account_id: pulumi_gestalt_rust::Output<String>,
        /// Specify the repo branch of the Source Control. Empty value is valid only for `VsoTfvc`.
        pub branch: pulumi_gestalt_rust::Output<Option<String>>,
        /// A short description of the Source Control.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The folder path of the source control. This Path must be relative.
        pub folder_path: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Automation Source Control. Changing this forces a new Automation Source Control to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether auto publish the Source Control. Defaults to `true`.
        pub publish_runbook_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Repository URL of the source control.
        pub repository_url: pulumi_gestalt_rust::Output<String>,
        /// A `security` block as defined below.
        pub security: pulumi_gestalt_rust::Output<
            super::super::types::automation::SourceControlSecurity,
        >,
        /// The source type of Source Control, possible vaules are `VsoGit`, `VsoTfvc` and `GitHub`, and the value is case sensitive.
        pub source_control_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SourceControlArgs,
    ) -> SourceControlResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let automatic_sync_binding = args.automatic_sync.get_output(context);
        let automation_account_id_binding = args
            .automation_account_id
            .get_output(context);
        let branch_binding = args.branch.get_output(context);
        let description_binding = args.description.get_output(context);
        let folder_path_binding = args.folder_path.get_output(context);
        let name_binding = args.name.get_output(context);
        let publish_runbook_enabled_binding = args
            .publish_runbook_enabled
            .get_output(context);
        let repository_url_binding = args.repository_url.get_output(context);
        let security_binding = args.security.get_output(context);
        let source_control_type_binding = args.source_control_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:automation/sourceControl:SourceControl".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automaticSync".into(),
                    value: automatic_sync_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automationAccountId".into(),
                    value: automation_account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "branch".into(),
                    value: branch_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "folderPath".into(),
                    value: folder_path_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publishRunbookEnabled".into(),
                    value: publish_runbook_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositoryUrl".into(),
                    value: repository_url_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "security".into(),
                    value: security_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceControlType".into(),
                    value: source_control_type_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SourceControlResult {
            automatic_sync: o.get_field("automaticSync"),
            automation_account_id: o.get_field("automationAccountId"),
            branch: o.get_field("branch"),
            description: o.get_field("description"),
            folder_path: o.get_field("folderPath"),
            name: o.get_field("name"),
            publish_runbook_enabled: o.get_field("publishRunbookEnabled"),
            repository_url: o.get_field("repositoryUrl"),
            security: o.get_field("security"),
            source_control_type: o.get_field("sourceControlType"),
        }
    }
}
