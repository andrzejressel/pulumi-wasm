#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTableArgs {
        /// The name of the Table.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Storage Account where the Table exists.
        #[builder(into)]
        pub storage_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTableResult {
        /// A mapping of ACLs for this Table.
        pub acls: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::storage::GetTableAcl>,
        >,
        /// The ID of the Storage Table.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Resource Manager ID of this Storage Table.
        pub resource_manager_id: pulumi_gestalt_rust::Output<String>,
        pub storage_account_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTableArgs,
    ) -> GetTableResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let storage_account_name_binding = args.storage_account_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:storage/getTable:getTable".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountName".into(),
                    value: storage_account_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTableResult {
            acls: o.get_field("acls"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            resource_manager_id: o.get_field("resourceManagerId"),
            storage_account_name: o.get_field("storageAccountName"),
        }
    }
}
