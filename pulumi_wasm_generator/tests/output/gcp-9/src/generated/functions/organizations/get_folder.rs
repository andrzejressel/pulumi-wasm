pub mod get_folder {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFolderArgs {
        /// The name of the Folder in the form `{folder_id}` or `folders/{folder_id}`.
        #[builder(into)]
        pub folder: pulumi_wasm_rust::InputOrOutput<String>,
        /// `true` to find the organization that the folder belongs, `false` to avoid the lookup. It searches up the tree. (defaults to `false`)
        #[builder(into, default)]
        pub lookup_organization: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetFolderResult {
        /// Timestamp when the Organization was created. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub deletion_protection: pulumi_wasm_rust::Output<bool>,
        /// The folder's display name.
        pub display_name: pulumi_wasm_rust::Output<String>,
        pub folder: pulumi_wasm_rust::Output<String>,
        pub folder_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Folder's current lifecycle state.
        pub lifecycle_state: pulumi_wasm_rust::Output<String>,
        pub lookup_organization: pulumi_wasm_rust::Output<Option<bool>>,
        /// The resource name of the Folder in the form `folders/{folder_id}`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// If `lookup_organization` is enable, the resource name of the Organization that the folder belongs.
        pub organization: pulumi_wasm_rust::Output<String>,
        /// The resource name of the parent Folder or Organization.
        pub parent: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetFolderArgs,
    ) -> GetFolderResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let folder_binding = args.folder.get_output(context).get_inner();
        let lookup_organization_binding = args
            .lookup_organization
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:organizations/getFolder:getFolder".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "folder".into(),
                    value: &folder_binding,
                },
                register_interface::ObjectField {
                    name: "lookupOrganization".into(),
                    value: &lookup_organization_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFolderResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            folder: pulumi_wasm_rust::__private::into_domain(o.extract_field("folder")),
            folder_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("folderId"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            lifecycle_state: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lifecycleState"),
            ),
            lookup_organization: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lookupOrganization"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            organization: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("organization"),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(o.extract_field("parent")),
        }
    }
}
