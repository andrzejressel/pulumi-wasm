pub mod get_workspace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWorkspaceArgs {
        /// The name of the Databricks Workspace.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Name of the Resource Group where the Databricks Workspace exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the Databricks Workspace.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetWorkspaceResult {
        /// An `enhanced_security_compliance` block as documented below.
        pub enhanced_security_compliances: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::databricks::GetWorkspaceEnhancedSecurityCompliance,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure location where the Databricks Workspace exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `managed_disk_identity` block as documented below.
        pub managed_disk_identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::databricks::GetWorkspaceManagedDiskIdentity>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// SKU of this Databricks Workspace.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A `storage_account_identity` block as documented below.
        pub storage_account_identities: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::databricks::GetWorkspaceStorageAccountIdentity,
            >,
        >,
        /// A mapping of tags to assign to the Databricks Workspace.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Unique ID of this Databricks Workspace in Databricks management plane.
        pub workspace_id: pulumi_wasm_rust::Output<String>,
        /// URL this Databricks Workspace is accessible on.
        pub workspace_url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetWorkspaceArgs) -> GetWorkspaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:databricks/getWorkspace:getWorkspace".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "enhancedSecurityCompliances".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managedDiskIdentities".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountIdentities".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "workspaceId".into(),
                },
                register_interface::ResultField {
                    name: "workspaceUrl".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetWorkspaceResult {
            enhanced_security_compliances: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enhancedSecurityCompliances").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            managed_disk_identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedDiskIdentities").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            storage_account_identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountIdentities").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
            workspace_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceUrl").unwrap(),
            ),
        }
    }
}
