pub mod get_bundle {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBundleArgs {
        /// ID of the bundle.
        #[builder(into, default)]
        pub bundle_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the bundle. You cannot combine this parameter with `bundle_id`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Owner of the bundles. You have to leave it blank for own bundles. You cannot combine this parameter with `bundle_id`.
        #[builder(into, default)]
        pub owner: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBundleResult {
        /// The ID of the bundle.
        pub bundle_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The compute type. See supported fields below.
        pub compute_types: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::workspaces::GetBundleComputeType>,
        >,
        /// The description of the bundle.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Name of the compute type.
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The owner of the bundle.
        pub owner: pulumi_wasm_rust::Output<Option<String>>,
        /// The root volume. See supported fields below.
        pub root_storages: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::workspaces::GetBundleRootStorage>,
        >,
        /// The user storage. See supported fields below.
        pub user_storages: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::workspaces::GetBundleUserStorage>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetBundleArgs,
    ) -> GetBundleResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bundle_id_binding = args.bundle_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let owner_binding = args.owner.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:workspaces/getBundle:getBundle".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bundleId".into(),
                    value: &bundle_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "owner".into(),
                    value: &owner_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetBundleResult {
            bundle_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bundleId"),
            ),
            compute_types: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("computeTypes"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            owner: pulumi_wasm_rust::__private::into_domain(o.extract_field("owner")),
            root_storages: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rootStorages"),
            ),
            user_storages: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userStorages"),
            ),
        }
    }
}
