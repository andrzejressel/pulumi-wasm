#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatabaseArgs {
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDatabaseResult {
        /// The ARN that uniquely identifies this database.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Creation time of database.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the KMS key used to encrypt the data stored in the database.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Last time database was updated.
        pub last_updated_time: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Total number of tables in the Timestream database.
        pub table_count: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDatabaseArgs,
    ) -> GetDatabaseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:timestreamwrite/getDatabase:getDatabase".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDatabaseResult {
            arn: o.get_field("arn"),
            created_time: o.get_field("createdTime"),
            id: o.get_field("id"),
            kms_key_id: o.get_field("kmsKeyId"),
            last_updated_time: o.get_field("lastUpdatedTime"),
            name: o.get_field("name"),
            table_count: o.get_field("tableCount"),
        }
    }
}
