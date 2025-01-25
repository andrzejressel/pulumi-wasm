pub mod get_workspace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWorkspaceArgs {
        /// ID of the directory for the WorkSpace. You have to specify `user_name` along with `directory_id`. You cannot combine this parameter with `workspace_id`.
        #[builder(into, default)]
        pub directory_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Tags for the WorkSpace.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// User name of the user for the WorkSpace. This user name must exist in the directory for the WorkSpace. You cannot combine this parameter with `workspace_id`.
        #[builder(into, default)]
        pub user_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ID of the WorkSpace. You cannot combine this parameter with `directory_id`.
        #[builder(into, default)]
        pub workspace_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetWorkspaceResult {
        pub bundle_id: pulumi_wasm_rust::Output<String>,
        /// Name of the WorkSpace, as seen by the operating system.
        pub computer_name: pulumi_wasm_rust::Output<String>,
        pub directory_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// IP address of the WorkSpace.
        pub ip_address: pulumi_wasm_rust::Output<String>,
        pub root_volume_encryption_enabled: pulumi_wasm_rust::Output<bool>,
        /// Operational state of the WorkSpace.
        pub state: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub user_name: pulumi_wasm_rust::Output<String>,
        pub user_volume_encryption_enabled: pulumi_wasm_rust::Output<bool>,
        pub volume_encryption_key: pulumi_wasm_rust::Output<String>,
        pub workspace_id: pulumi_wasm_rust::Output<String>,
        pub workspace_properties: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::workspaces::GetWorkspaceWorkspaceProperty>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetWorkspaceArgs,
    ) -> GetWorkspaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let directory_id_binding = args.directory_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let user_name_binding = args.user_name.get_output(context).get_inner();
        let workspace_id_binding = args.workspace_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:workspaces/getWorkspace:getWorkspace".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "directoryId".into(),
                    value: &directory_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "userName".into(),
                    value: &user_name_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bundleId".into(),
                },
                register_interface::ResultField {
                    name: "computerName".into(),
                },
                register_interface::ResultField {
                    name: "directoryId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipAddress".into(),
                },
                register_interface::ResultField {
                    name: "rootVolumeEncryptionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "userName".into(),
                },
                register_interface::ResultField {
                    name: "userVolumeEncryptionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "volumeEncryptionKey".into(),
                },
                register_interface::ResultField {
                    name: "workspaceId".into(),
                },
                register_interface::ResultField {
                    name: "workspaceProperties".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetWorkspaceResult {
            bundle_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bundleId").unwrap(),
            ),
            computer_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computerName").unwrap(),
            ),
            directory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("directoryId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddress").unwrap(),
            ),
            root_volume_encryption_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootVolumeEncryptionEnabled").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            user_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userName").unwrap(),
            ),
            user_volume_encryption_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userVolumeEncryptionEnabled").unwrap(),
            ),
            volume_encryption_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeEncryptionKey").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
            workspace_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceProperties").unwrap(),
            ),
        }
    }
}
