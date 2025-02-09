#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_share {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetShareArgs {
        /// One or more acl blocks as defined below.
        #[builder(into, default)]
        pub acls: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::storage::GetShareAcl>>,
        >,
        /// A map of custom file share metadata.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the share.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the storage account in which the share exists.
        ///
        /// > **NOTE:** One of `storage_account_name` or `storage_account_id` must be specified. When specifying `storage_account_id` the resource will use the Resource Manager API, rather than the Data Plane API.
        #[builder(into, default)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the storage account in which the share exists. This property is deprecated in favour of `storage_account_id`.
        #[builder(into, default)]
        pub storage_account_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetShareResult {
        /// One or more acl blocks as defined below.
        pub acls: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::storage::GetShareAcl>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A map of custom file share metadata.
        pub metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The quota of the File Share in GB.
        pub quota: pulumi_gestalt_rust::Output<i32>,
        pub resource_manager_id: pulumi_gestalt_rust::Output<String>,
        pub storage_account_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub storage_account_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetShareArgs,
    ) -> GetShareResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let acls_binding = args.acls.get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let name_binding = args.name.get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let storage_account_name_binding = args.storage_account_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:storage/getShare:getShare".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "acls".into(),
                    value: acls_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: metadata_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: storage_account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountName".into(),
                    value: storage_account_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetShareResult {
            acls: o.get_field("acls"),
            id: o.get_field("id"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            quota: o.get_field("quota"),
            resource_manager_id: o.get_field("resourceManagerId"),
            storage_account_id: o.get_field("storageAccountId"),
            storage_account_name: o.get_field("storageAccountName"),
        }
    }
}
