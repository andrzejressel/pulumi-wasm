/// Manages an App Service Source Control Slot.
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
///     let exampleLinuxWebApp = linux_web_app::create(
///         "exampleLinuxWebApp",
///         LinuxWebAppArgs::builder()
///             .location("${exampleServicePlan.location}")
///             .name("example-web-app")
///             .resource_group_name("${example.name}")
///             .service_plan_id("${exampleServicePlan.id}")
///             .site_config(LinuxWebAppSiteConfig::builder().build_struct())
///             .build_struct(),
///     );
///     let exampleLinuxWebAppSlot = linux_web_app_slot::create(
///         "exampleLinuxWebAppSlot",
///         LinuxWebAppSlotArgs::builder()
///             .app_service_id("${exampleLinuxWebApp.id}")
///             .name("example-slot")
///             .site_config(LinuxWebAppSlotSiteConfig::builder().build_struct())
///             .build_struct(),
///     );
///     let exampleServicePlan = service_plan::create(
///         "exampleServicePlan",
///         ServicePlanArgs::builder()
///             .location("${example.location}")
///             .name("example-plan")
///             .os_type("Linux")
///             .resource_group_name("${example.name}")
///             .sku_name("P1v2")
///             .build_struct(),
///     );
///     let exampleSourceControlSlot = source_control_slot::create(
///         "exampleSourceControlSlot",
///         SourceControlSlotArgs::builder()
///             .branch("master")
///             .repo_url("https://github.com/Azure-Samples/python-docs-hello-world")
///             .slot_id("${exampleLinuxWebAppSlot.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// an App Service Source Control Slot can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/sourceControlSlot:SourceControlSlot example "/subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Web/sites/site1/slots/slot1"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod source_control_slot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SourceControlSlotArgs {
        /// The URL for the repository. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub branch: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `github_action_configuration` block as detailed below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub github_action_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::appservice::SourceControlSlotGithubActionConfiguration,
            >,
        >,
        /// The branch name to use for deployments. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub repo_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should the Deployment Rollback be enabled? Defaults to `false` Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub rollback_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Linux or Windows Web App Slot. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Function App Slots are not supported at this time.
        #[builder(into)]
        pub slot_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Should the Slot use local Git configuration. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub use_local_git: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Should code be deployed manually. Set to `true` to disable continuous integration, such as webhooks into online repos such as GitHub. Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub use_manual_integration: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The repository specified is Mercurial. Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub use_mercurial: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SourceControlSlotResult {
        /// The URL for the repository. Changing this forces a new resource to be created.
        pub branch: pulumi_gestalt_rust::Output<String>,
        /// A `github_action_configuration` block as detailed below. Changing this forces a new resource to be created.
        pub github_action_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::appservice::SourceControlSlotGithubActionConfiguration,
            >,
        >,
        /// The branch name to use for deployments. Changing this forces a new resource to be created.
        pub repo_url: pulumi_gestalt_rust::Output<String>,
        /// Should the Deployment Rollback be enabled? Defaults to `false` Changing this forces a new resource to be created.
        pub rollback_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The SCM Type in use. This value is decoded by the service from the repository information supplied.
        pub scm_type: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Linux or Windows Web App Slot. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Function App Slots are not supported at this time.
        pub slot_id: pulumi_gestalt_rust::Output<String>,
        /// Should the Slot use local Git configuration. Changing this forces a new resource to be created.
        pub use_local_git: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should code be deployed manually. Set to `true` to disable continuous integration, such as webhooks into online repos such as GitHub. Defaults to `false`. Changing this forces a new resource to be created.
        pub use_manual_integration: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The repository specified is Mercurial. Defaults to `false`. Changing this forces a new resource to be created.
        pub use_mercurial: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Indicates if the Slot uses a GitHub action for deployment. This value is decoded by the service from the repository information supplied.
        pub uses_github_action: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SourceControlSlotArgs,
    ) -> SourceControlSlotResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let branch_binding = args.branch.get_output(context).get_inner();
        let github_action_configuration_binding = args
            .github_action_configuration
            .get_output(context)
            .get_inner();
        let repo_url_binding = args.repo_url.get_output(context).get_inner();
        let rollback_enabled_binding = args
            .rollback_enabled
            .get_output(context)
            .get_inner();
        let slot_id_binding = args.slot_id.get_output(context).get_inner();
        let use_local_git_binding = args.use_local_git.get_output(context).get_inner();
        let use_manual_integration_binding = args
            .use_manual_integration
            .get_output(context)
            .get_inner();
        let use_mercurial_binding = args.use_mercurial.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/sourceControlSlot:SourceControlSlot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "branch".into(),
                    value: &branch_binding,
                },
                register_interface::ObjectField {
                    name: "githubActionConfiguration".into(),
                    value: &github_action_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "repoUrl".into(),
                    value: &repo_url_binding,
                },
                register_interface::ObjectField {
                    name: "rollbackEnabled".into(),
                    value: &rollback_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "slotId".into(),
                    value: &slot_id_binding,
                },
                register_interface::ObjectField {
                    name: "useLocalGit".into(),
                    value: &use_local_git_binding,
                },
                register_interface::ObjectField {
                    name: "useManualIntegration".into(),
                    value: &use_manual_integration_binding,
                },
                register_interface::ObjectField {
                    name: "useMercurial".into(),
                    value: &use_mercurial_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SourceControlSlotResult {
            branch: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("branch"),
            ),
            github_action_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("githubActionConfiguration"),
            ),
            repo_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("repoUrl"),
            ),
            rollback_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rollbackEnabled"),
            ),
            scm_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scmType"),
            ),
            slot_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("slotId"),
            ),
            use_local_git: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("useLocalGit"),
            ),
            use_manual_integration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("useManualIntegration"),
            ),
            use_mercurial: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("useMercurial"),
            ),
            uses_github_action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("usesGithubAction"),
            ),
        }
    }
}
