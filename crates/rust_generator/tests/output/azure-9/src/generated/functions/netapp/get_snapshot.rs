#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSnapshotArgs {
        /// The name of the NetApp Account where the NetApp Pool exists.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the NetApp Snapshot.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the NetApp Pool where the NetApp Volume exists.
        #[builder(into)]
        pub pool_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the NetApp Snapshot exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the NetApp Volume where the NetApp Snapshot exists.
        #[builder(into)]
        pub volume_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSnapshotResult {
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the NetApp Snapshot exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub pool_name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub volume_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSnapshotArgs,
    ) -> GetSnapshotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let pool_name_binding = args.pool_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let volume_name_binding = args.volume_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:netapp/getSnapshot:getSnapshot".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: account_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "poolName".into(),
                    value: pool_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "volumeName".into(),
                    value: volume_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSnapshotResult {
            account_name: o.get_field("accountName"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            pool_name: o.get_field("poolName"),
            resource_group_name: o.get_field("resourceGroupName"),
            volume_name: o.get_field("volumeName"),
        }
    }
}
