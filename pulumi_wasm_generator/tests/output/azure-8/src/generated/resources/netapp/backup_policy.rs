/// Manages a NetApp Backup Policy.
///
/// ## NetApp Backup Policy Usage
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
///             .name("example-netappaccount")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleBackupPolicy = backup_policy::create(
///         "exampleBackupPolicy",
///         BackupPolicyArgs::builder()
///             .account_name("${exampleAccount.name}")
///             .enabled(true)
///             .location("${example.location}")
///             .name("example-netappbackuppolicy")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// NetApp Backup Policy can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:netapp/backupPolicy:BackupPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.NetApp/netAppAccounts/account1/backupPolicies/backuppolicy1
/// ```
///
pub mod backup_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPolicyArgs {
        /// The name of the NetApp account in which the NetApp Policy should be created under. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// Provides the number of daily backups to keep, defaults to `2` which is the minimum, maximum is 1019.
        #[builder(into, default)]
        pub daily_backups_to_keep: pulumi_wasm_rust::Output<Option<i32>>,
        /// Whether the Backup Policy is enabled. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Provides the number of monthly backups to keep, defaults to `1` which is the minimum, maximum is 1019.
        ///
        /// > **Note:** Currently, the combined (daily + weekly + monthy) retention counts cannot exceed 1019.
        #[builder(into, default)]
        pub monthly_backups_to_keep: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the NetApp Backup Policy. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group where the NetApp Backup Policy should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Provides the number of weekly backups to keep, defaults to `1` which is the minimum, maximum is 1019.
        #[builder(into, default)]
        pub weekly_backups_to_keep: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct BackupPolicyResult {
        /// The name of the NetApp account in which the NetApp Policy should be created under. Changing this forces a new resource to be created.
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// Provides the number of daily backups to keep, defaults to `2` which is the minimum, maximum is 1019.
        pub daily_backups_to_keep: pulumi_wasm_rust::Output<Option<i32>>,
        /// Whether the Backup Policy is enabled. Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Provides the number of monthly backups to keep, defaults to `1` which is the minimum, maximum is 1019.
        ///
        /// > **Note:** Currently, the combined (daily + weekly + monthy) retention counts cannot exceed 1019.
        pub monthly_backups_to_keep: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the NetApp Backup Policy. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group where the NetApp Backup Policy should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Provides the number of weekly backups to keep, defaults to `1` which is the minimum, maximum is 1019.
        pub weekly_backups_to_keep: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BackupPolicyArgs) -> BackupPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_inner();
        let daily_backups_to_keep_binding = args.daily_backups_to_keep.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let location_binding = args.location.get_inner();
        let monthly_backups_to_keep_binding = args.monthly_backups_to_keep.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let weekly_backups_to_keep_binding = args.weekly_backups_to_keep.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:netapp/backupPolicy:BackupPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "dailyBackupsToKeep".into(),
                    value: &daily_backups_to_keep_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "monthlyBackupsToKeep".into(),
                    value: &monthly_backups_to_keep_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "weeklyBackupsToKeep".into(),
                    value: &weekly_backups_to_keep_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountName".into(),
                },
                register_interface::ResultField {
                    name: "dailyBackupsToKeep".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "monthlyBackupsToKeep".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "weeklyBackupsToKeep".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BackupPolicyResult {
            account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountName").unwrap(),
            ),
            daily_backups_to_keep: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dailyBackupsToKeep").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            monthly_backups_to_keep: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monthlyBackupsToKeep").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            weekly_backups_to_keep: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("weeklyBackupsToKeep").unwrap(),
            ),
        }
    }
}
