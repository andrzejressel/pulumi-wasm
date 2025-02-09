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
///             .name("example")
///             .build_struct(),
///     );
///     let exampleManagedDatabase = managed_database::create(
///         "exampleManagedDatabase",
///         ManagedDatabaseArgs::builder()
///             .managed_instance_id("${exampleManagedInstance.id}")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleManagedInstance = managed_instance::create(
///         "exampleManagedInstance",
///         ManagedInstanceArgs::builder()
///             .administrator_login("msadministrator")
///             .administrator_login_password("thisIsDog11")
///             .license_type("BasePrice")
///             .location("${example.location}")
///             .name("managedsqlinstance")
///             .resource_group_name("${example.name}")
///             .sku_name("GP_Gen5")
///             .storage_size_in_gb(32)
///             .subnet_id("${exampleSubnet.id}")
///             .vcores(4)
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.2.0/24",])
///             .name("example")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// SQL Managed Databases can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/managedDatabase:ManagedDatabase example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.Sql/managedInstances/myserver/databases/mydatabase
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod managed_database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedDatabaseArgs {
        /// A `long_term_retention_policy` block as defined below.
        #[builder(into, default)]
        pub long_term_retention_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mssql::ManagedDatabaseLongTermRetentionPolicy>,
        >,
        /// The ID of the Azure SQL Managed Instance on which to create this Managed Database. Changing this forces a new resource to be created.
        #[builder(into)]
        pub managed_instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Managed Database to create. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `point_in_time_restore` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub point_in_time_restore: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mssql::ManagedDatabasePointInTimeRestore>,
        >,
        /// The backup retention period in days. This is how many days Point-in-Time Restore will be supported.
        #[builder(into, default)]
        pub short_term_retention_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ManagedDatabaseResult {
        /// A `long_term_retention_policy` block as defined below.
        pub long_term_retention_policy: pulumi_gestalt_rust::Output<
            super::super::types::mssql::ManagedDatabaseLongTermRetentionPolicy,
        >,
        /// The ID of the Azure SQL Managed Instance on which to create this Managed Database. Changing this forces a new resource to be created.
        pub managed_instance_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Managed Database to create. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `point_in_time_restore` block as defined below. Changing this forces a new resource to be created.
        pub point_in_time_restore: pulumi_gestalt_rust::Output<
            Option<super::super::types::mssql::ManagedDatabasePointInTimeRestore>,
        >,
        /// The backup retention period in days. This is how many days Point-in-Time Restore will be supported.
        pub short_term_retention_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedDatabaseArgs,
    ) -> ManagedDatabaseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let long_term_retention_policy_binding = args
            .long_term_retention_policy
            .get_output(context);
        let managed_instance_id_binding = args.managed_instance_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let point_in_time_restore_binding = args
            .point_in_time_restore
            .get_output(context);
        let short_term_retention_days_binding = args
            .short_term_retention_days
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mssql/managedDatabase:ManagedDatabase".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "longTermRetentionPolicy".into(),
                    value: long_term_retention_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedInstanceId".into(),
                    value: managed_instance_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pointInTimeRestore".into(),
                    value: point_in_time_restore_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shortTermRetentionDays".into(),
                    value: short_term_retention_days_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ManagedDatabaseResult {
            long_term_retention_policy: o.get_field("longTermRetentionPolicy"),
            managed_instance_id: o.get_field("managedInstanceId"),
            name: o.get_field("name"),
            point_in_time_restore: o.get_field("pointInTimeRestore"),
            short_term_retention_days: o.get_field("shortTermRetentionDays"),
            tags: o.get_field("tags"),
        }
    }
}
