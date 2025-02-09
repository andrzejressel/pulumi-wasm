/// Manages a NetApp Backup Policy.
///
/// ## NetApp Backup Policy Usage
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backup_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPolicyArgs {
        /// The name of the NetApp account in which the NetApp Policy should be created under. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Provides the number of daily backups to keep, defaults to `2` which is the minimum, maximum is 1019.
        #[builder(into, default)]
        pub daily_backups_to_keep: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Whether the Backup Policy is enabled. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Provides the number of monthly backups to keep, defaults to `1` which is the minimum, maximum is 1019.
        ///
        /// > **Note:** Currently, the combined (daily + weekly + monthy) retention counts cannot exceed 1019.
        #[builder(into, default)]
        pub monthly_backups_to_keep: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the NetApp Backup Policy. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group where the NetApp Backup Policy should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Provides the number of weekly backups to keep, defaults to `1` which is the minimum, maximum is 1019.
        #[builder(into, default)]
        pub weekly_backups_to_keep: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct BackupPolicyResult {
        /// The name of the NetApp account in which the NetApp Policy should be created under. Changing this forces a new resource to be created.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// Provides the number of daily backups to keep, defaults to `2` which is the minimum, maximum is 1019.
        pub daily_backups_to_keep: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Whether the Backup Policy is enabled. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Provides the number of monthly backups to keep, defaults to `1` which is the minimum, maximum is 1019.
        ///
        /// > **Note:** Currently, the combined (daily + weekly + monthy) retention counts cannot exceed 1019.
        pub monthly_backups_to_keep: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the NetApp Backup Policy. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group where the NetApp Backup Policy should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Provides the number of weekly backups to keep, defaults to `1` which is the minimum, maximum is 1019.
        pub weekly_backups_to_keep: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BackupPolicyArgs,
    ) -> BackupPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_name_binding_1 = args.account_name.get_output(context);
        let account_name_binding = account_name_binding_1.get_inner();
        let daily_backups_to_keep_binding_1 = args
            .daily_backups_to_keep
            .get_output(context);
        let daily_backups_to_keep_binding = daily_backups_to_keep_binding_1.get_inner();
        let enabled_binding_1 = args.enabled.get_output(context);
        let enabled_binding = enabled_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let monthly_backups_to_keep_binding_1 = args
            .monthly_backups_to_keep
            .get_output(context);
        let monthly_backups_to_keep_binding = monthly_backups_to_keep_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let weekly_backups_to_keep_binding_1 = args
            .weekly_backups_to_keep
            .get_output(context);
        let weekly_backups_to_keep_binding = weekly_backups_to_keep_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:netapp/backupPolicy:BackupPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        BackupPolicyResult {
            account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountName"),
            ),
            daily_backups_to_keep: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dailyBackupsToKeep"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            monthly_backups_to_keep: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monthlyBackupsToKeep"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            weekly_backups_to_keep: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("weeklyBackupsToKeep"),
            ),
        }
    }
}
