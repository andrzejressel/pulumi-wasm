/// Manages a Container App Environment Storage.
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
pub mod environment_storage {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentStorageArgs {
        /// The Storage Account Access Key.
        #[builder(into)]
        pub access_key: pulumi_wasm_rust::Output<String>,
        /// The access mode to connect this storage to the Container App. Possible values include `ReadOnly` and `ReadWrite`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub access_mode: pulumi_wasm_rust::Output<String>,
        /// The Azure Storage Account in which the Share to be used is located. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Container App Environment to which this storage belongs. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_app_environment_id: pulumi_wasm_rust::Output<String>,
        /// The name for this Container App Environment Storage. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Azure Storage Share to use. Changing this forces a new resource to be created.
        #[builder(into)]
        pub share_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct EnvironmentStorageResult {
        /// The Storage Account Access Key.
        pub access_key: pulumi_wasm_rust::Output<String>,
        /// The access mode to connect this storage to the Container App. Possible values include `ReadOnly` and `ReadWrite`. Changing this forces a new resource to be created.
        pub access_mode: pulumi_wasm_rust::Output<String>,
        /// The Azure Storage Account in which the Share to be used is located. Changing this forces a new resource to be created.
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Container App Environment to which this storage belongs. Changing this forces a new resource to be created.
        pub container_app_environment_id: pulumi_wasm_rust::Output<String>,
        /// The name for this Container App Environment Storage. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Azure Storage Share to use. Changing this forces a new resource to be created.
        pub share_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EnvironmentStorageArgs) -> EnvironmentStorageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_key_binding = args.access_key.get_inner();
        let access_mode_binding = args.access_mode.get_inner();
        let account_name_binding = args.account_name.get_inner();
        let container_app_environment_id_binding = args
            .container_app_environment_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let share_name_binding = args.share_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerapp/environmentStorage:EnvironmentStorage".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessKey".into(),
                },
                register_interface::ResultField {
                    name: "accessMode".into(),
                },
                register_interface::ResultField {
                    name: "accountName".into(),
                },
                register_interface::ResultField {
                    name: "containerAppEnvironmentId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "shareName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EnvironmentStorageResult {
            access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessKey").unwrap(),
            ),
            access_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessMode").unwrap(),
            ),
            account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountName").unwrap(),
            ),
            container_app_environment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerAppEnvironmentId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            share_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shareName").unwrap(),
            ),
        }
    }
}