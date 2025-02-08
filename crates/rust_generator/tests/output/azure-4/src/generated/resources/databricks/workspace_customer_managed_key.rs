///
///
/// ## Import
///
/// Databrick Workspaces can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:databricks/workspaceCustomerManagedKey:WorkspaceCustomerManagedKey workspace1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Databricks/workspaces/workspace1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workspace_customer_managed_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceCustomerManagedKeyArgs {
        #[builder(into)]
        pub key_vault_key_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The unique identifier of the databricks workspace in Databricks control plane.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceCustomerManagedKeyResult {
        pub key_vault_key_id: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier of the databricks workspace in Databricks control plane.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WorkspaceCustomerManagedKeyArgs,
    ) -> WorkspaceCustomerManagedKeyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let key_vault_key_id_binding = args
            .key_vault_key_id
            .get_output(context)
            .get_inner();
        let workspace_id_binding = args.workspace_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:databricks/workspaceCustomerManagedKey:WorkspaceCustomerManagedKey"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "keyVaultKeyId".into(),
                    value: &key_vault_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WorkspaceCustomerManagedKeyResult {
            key_vault_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultKeyId"),
            ),
            workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceId"),
            ),
        }
    }
}
