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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupPolicyArgs,
    ) -> BackupPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let daily_backups_to_keep_binding = args
            .daily_backups_to_keep
            .get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let location_binding = args.location.get_output(context);
        let monthly_backups_to_keep_binding = args
            .monthly_backups_to_keep
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let weekly_backups_to_keep_binding = args
            .weekly_backups_to_keep
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:netapp/backupPolicy:BackupPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: account_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dailyBackupsToKeep".into(),
                    value: daily_backups_to_keep_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monthlyBackupsToKeep".into(),
                    value: monthly_backups_to_keep_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "weeklyBackupsToKeep".into(),
                    value: weekly_backups_to_keep_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BackupPolicyResult {
            account_name: o.get_field("accountName"),
            daily_backups_to_keep: o.get_field("dailyBackupsToKeep"),
            enabled: o.get_field("enabled"),
            location: o.get_field("location"),
            monthly_backups_to_keep: o.get_field("monthlyBackupsToKeep"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            weekly_backups_to_keep: o.get_field("weeklyBackupsToKeep"),
        }
    }
}
