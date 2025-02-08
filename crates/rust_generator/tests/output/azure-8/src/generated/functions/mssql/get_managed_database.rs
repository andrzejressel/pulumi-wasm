#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_managed_database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagedDatabaseArgs {
        /// The SQL Managed Instance ID.
        #[builder(into)]
        pub managed_instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of this Azure SQL Azure Managed Database.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetManagedDatabaseResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `long_term_retention_policy` block as defined below.
        pub long_term_retention_policies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::mssql::GetManagedDatabaseLongTermRetentionPolicy,
            >,
        >,
        pub managed_instance_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Managed Instance.
        pub managed_instance_name: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `point_in_time_restore` block as defined below.
        pub point_in_time_restores: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::mssql::GetManagedDatabasePointInTimeRestore>,
        >,
        /// The name of the Resource Group where the Azure SQL Azure Managed Instance exists.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The backup retention period in days. This is how many days Point-in-Time Restore will be supported.
        pub short_term_retention_days: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetManagedDatabaseArgs,
    ) -> GetManagedDatabaseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let managed_instance_id_binding = args
            .managed_instance_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:mssql/getManagedDatabase:getManagedDatabase".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "managedInstanceId".into(),
                    value: &managed_instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetManagedDatabaseResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            long_term_retention_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("longTermRetentionPolicies"),
            ),
            managed_instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedInstanceId"),
            ),
            managed_instance_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedInstanceName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            point_in_time_restores: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pointInTimeRestores"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            short_term_retention_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shortTermRetentionDays"),
            ),
        }
    }
}
