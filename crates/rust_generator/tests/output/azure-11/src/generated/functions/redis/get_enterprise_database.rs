#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_enterprise_database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEnterpriseDatabaseArgs {
        /// The resource ID of Redis Enterprise Cluster which hosts the Redis Enterprise Database instance.
        #[builder(into)]
        pub cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Redis Enterprise Database.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetEnterpriseDatabaseResult {
        /// The Redis Enterprise Cluster ID that is hosting the Redis Enterprise Database.
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Linked Database Group Nickname for the Redis Enterprise Database instance.
        pub linked_database_group_nickname: pulumi_gestalt_rust::Output<String>,
        /// The Linked Database list for the Redis Enterprise Database instance.
        pub linked_database_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Redis Enterprise Database name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Primary Access Key for the Redis Enterprise Database instance.
        pub primary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Access Key for the Redis Enterprise Database instance.
        pub secondary_access_key: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetEnterpriseDatabaseArgs,
    ) -> GetEnterpriseDatabaseResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_id_binding = args.cluster_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:redis/getEnterpriseDatabase:getEnterpriseDatabase".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetEnterpriseDatabaseResult {
            cluster_id: o.get_field("clusterId"),
            id: o.get_field("id"),
            linked_database_group_nickname: o.get_field("linkedDatabaseGroupNickname"),
            linked_database_ids: o.get_field("linkedDatabaseIds"),
            name: o.get_field("name"),
            primary_access_key: o.get_field("primaryAccessKey"),
            secondary_access_key: o.get_field("secondaryAccessKey"),
        }
    }
}
