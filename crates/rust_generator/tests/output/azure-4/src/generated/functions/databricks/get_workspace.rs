#[allow(clippy::doc_lazy_continuation)]
pub mod get_workspace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWorkspaceArgs {
        /// The name of the Databricks Workspace.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the Databricks Workspace exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the Databricks Workspace.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetWorkspaceResult {
        /// An `enhanced_security_compliance` block as documented below.
        pub enhanced_security_compliances: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::databricks::GetWorkspaceEnhancedSecurityCompliance,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure location where the Databricks Workspace exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `managed_disk_identity` block as documented below.
        pub managed_disk_identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::databricks::GetWorkspaceManagedDiskIdentity>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// SKU of this Databricks Workspace.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// A `storage_account_identity` block as documented below.
        pub storage_account_identities: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::databricks::GetWorkspaceStorageAccountIdentity,
            >,
        >,
        /// A mapping of tags to assign to the Databricks Workspace.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Unique ID of this Databricks Workspace in Databricks management plane.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
        /// URL this Databricks Workspace is accessible on.
        pub workspace_url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetWorkspaceArgs,
    ) -> GetWorkspaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:databricks/getWorkspace:getWorkspace".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetWorkspaceResult {
            enhanced_security_compliances: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enhancedSecurityCompliances"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            managed_disk_identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedDiskIdentities"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sku")),
            storage_account_identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountIdentities"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceId"),
            ),
            workspace_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceUrl"),
            ),
        }
    }
}
