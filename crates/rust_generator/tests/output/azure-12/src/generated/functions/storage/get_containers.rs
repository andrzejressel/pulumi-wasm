#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_containers {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetContainersArgs {
        /// A prefix match used for the Storage Container `name` field.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Storage Account that the Storage Containers reside in.
        #[builder(into)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetContainersResult {
        /// A `containers` block as defined below.
        pub containers: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::storage::GetContainersContainer>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetContainersArgs,
    ) -> GetContainersResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_prefix_binding = args.name_prefix.get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:storage/getContainers:getContainers".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetContainersResult {
            containers: o.get_field("containers"),
            id: o.get_field("id"),
            name_prefix: o.get_field("namePrefix"),
            storage_account_id: o.get_field("storageAccountId"),
        }
    }
}
