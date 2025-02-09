#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTableArgs {
        /// Name of the Timestream database.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the Timestream table.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTableResult {
        /// ARN that uniquely identifies the table.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Time that table was created.
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        /// Name of database.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Last time table was updated.
        pub last_updated_time: pulumi_gestalt_rust::Output<String>,
        /// Object containing the following attributes to desribe magnetic store writes.
        pub magnetic_store_write_properties: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::timestreamwrite::GetTableMagneticStoreWriteProperty,
            >,
        >,
        /// Name of the table.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Object containing the following attributes to describe the retention duration for the memory and magnetic stores.
        pub retention_properties: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::timestreamwrite::GetTableRetentionProperty>,
        >,
        /// Object containing the following attributes to describe the schema of the table.
        pub schemas: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::timestreamwrite::GetTableSchema>,
        >,
        /// Current state of table.
        pub table_status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTableArgs,
    ) -> GetTableResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let database_name_binding = args.database_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:timestreamwrite/getTable:getTable".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseName".into(),
                    value: database_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTableResult {
            arn: o.get_field("arn"),
            creation_time: o.get_field("creationTime"),
            database_name: o.get_field("databaseName"),
            id: o.get_field("id"),
            last_updated_time: o.get_field("lastUpdatedTime"),
            magnetic_store_write_properties: o.get_field("magneticStoreWriteProperties"),
            name: o.get_field("name"),
            retention_properties: o.get_field("retentionProperties"),
            schemas: o.get_field("schemas"),
            table_status: o.get_field("tableStatus"),
        }
    }
}
