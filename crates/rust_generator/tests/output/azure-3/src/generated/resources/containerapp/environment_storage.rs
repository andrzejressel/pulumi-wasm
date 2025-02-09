/// Manages a Container App Environment Storage.
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
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("azureteststorage")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("acctest-01")
///             .resource_group_name("${example.name}")
///             .retention_in_days(30)
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleEnvironment = environment::create(
///         "exampleEnvironment",
///         EnvironmentArgs::builder()
///             .location("${example.location}")
///             .log_analytics_workspace_id("${exampleAnalyticsWorkspace.id}")
///             .name("myEnvironment")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleEnvironmentStorage = environment_storage::create(
///         "exampleEnvironmentStorage",
///         EnvironmentStorageArgs::builder()
///             .access_key("${exampleAccount.primaryAccessKey}")
///             .access_mode("ReadOnly")
///             .account_name("${exampleAccount.name}")
///             .container_app_environment_id("${exampleEnvironment.id}")
///             .name("mycontainerappstorage")
///             .share_name("${exampleShare.name}")
///             .build_struct(),
///     );
///     let exampleShare = share::create(
///         "exampleShare",
///         ShareArgs::builder()
///             .name("sharename")
///             .quota(5)
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// A Container App Environment Storage can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerapp/environmentStorage:EnvironmentStorage example "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.App/managedEnvironments/myEnvironment/storages/mystorage"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod environment_storage {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentStorageArgs {
        /// The Storage Account Access Key.
        #[builder(into)]
        pub access_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The access mode to connect this storage to the Container App. Possible values include `ReadOnly` and `ReadWrite`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub access_mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Storage Account in which the Share to be used is located. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Container App Environment to which this storage belongs. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_app_environment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name for this Container App Environment Storage. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Azure Storage Share to use. Changing this forces a new resource to be created.
        #[builder(into)]
        pub share_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EnvironmentStorageResult {
        /// The Storage Account Access Key.
        pub access_key: pulumi_gestalt_rust::Output<String>,
        /// The access mode to connect this storage to the Container App. Possible values include `ReadOnly` and `ReadWrite`. Changing this forces a new resource to be created.
        pub access_mode: pulumi_gestalt_rust::Output<String>,
        /// The Azure Storage Account in which the Share to be used is located. Changing this forces a new resource to be created.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Container App Environment to which this storage belongs. Changing this forces a new resource to be created.
        pub container_app_environment_id: pulumi_gestalt_rust::Output<String>,
        /// The name for this Container App Environment Storage. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Azure Storage Share to use. Changing this forces a new resource to be created.
        pub share_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EnvironmentStorageArgs,
    ) -> EnvironmentStorageResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let access_key_binding_1 = args.access_key.get_output(context);
        let access_key_binding = access_key_binding_1.get_inner();
        let access_mode_binding_1 = args.access_mode.get_output(context);
        let access_mode_binding = access_mode_binding_1.get_inner();
        let account_name_binding_1 = args.account_name.get_output(context);
        let account_name_binding = account_name_binding_1.get_inner();
        let container_app_environment_id_binding_1 = args
            .container_app_environment_id
            .get_output(context);
        let container_app_environment_id_binding = container_app_environment_id_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let share_name_binding_1 = args.share_name.get_output(context);
        let share_name_binding = share_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerapp/environmentStorage:EnvironmentStorage".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessKey".into(),
                    value: &access_key_binding,
                },
                register_interface::ObjectField {
                    name: "accessMode".into(),
                    value: &access_mode_binding,
                },
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "containerAppEnvironmentId".into(),
                    value: &container_app_environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "shareName".into(),
                    value: &share_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EnvironmentStorageResult {
            access_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessKey"),
            ),
            access_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessMode"),
            ),
            account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountName"),
            ),
            container_app_environment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("containerAppEnvironmentId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            share_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shareName"),
            ),
        }
    }
}
