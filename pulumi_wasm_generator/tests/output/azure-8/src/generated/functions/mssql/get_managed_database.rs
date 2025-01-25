pub mod get_managed_database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagedDatabaseArgs {
        /// The SQL Managed Instance ID.
        #[builder(into)]
        pub managed_instance_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of this Azure SQL Azure Managed Database.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetManagedDatabaseResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A `long_term_retention_policy` block as defined below.
        pub long_term_retention_policies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::mssql::GetManagedDatabaseLongTermRetentionPolicy,
            >,
        >,
        pub managed_instance_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Managed Instance.
        pub managed_instance_name: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `point_in_time_restore` block as defined below.
        pub point_in_time_restores: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::mssql::GetManagedDatabasePointInTimeRestore>,
        >,
        /// The name of the Resource Group where the Azure SQL Azure Managed Instance exists.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The backup retention period in days. This is how many days Point-in-Time Restore will be supported.
        pub short_term_retention_days: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetManagedDatabaseArgs,
    ) -> GetManagedDatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "longTermRetentionPolicies".into(),
                },
                register_interface::ResultField {
                    name: "managedInstanceId".into(),
                },
                register_interface::ResultField {
                    name: "managedInstanceName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pointInTimeRestores".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "shortTermRetentionDays".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetManagedDatabaseResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            long_term_retention_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("longTermRetentionPolicies").unwrap(),
            ),
            managed_instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedInstanceId").unwrap(),
            ),
            managed_instance_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedInstanceName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            point_in_time_restores: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pointInTimeRestores").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            short_term_retention_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shortTermRetentionDays").unwrap(),
            ),
        }
    }
}
