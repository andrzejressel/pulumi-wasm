pub mod get_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfigurationArgs {
        /// Specifies the name of the Maintenance Configuration.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the Resource Group where this Maintenance Configuration exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetConfigurationResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The in guest user patch mode.
        pub in_guest_user_patch_mode: pulumi_wasm_rust::Output<String>,
        /// An `install_patches` block as defined below.
        pub install_patches: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::maintenance::GetConfigurationInstallPatch>,
        >,
        /// The Azure location where the resource exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The properties assigned to the resource.
        pub properties: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The scope of the Maintenance Configuration.
        pub scope: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The visibility of the Maintenance Configuration.
        pub visibility: pulumi_wasm_rust::Output<String>,
        /// A `window` block as defined below.
        pub windows: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::maintenance::GetConfigurationWindow>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetConfigurationArgs,
    ) -> GetConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:maintenance/getConfiguration:getConfiguration".into(),
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
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetConfigurationResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            in_guest_user_patch_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("inGuestUserPatchMode"),
            ),
            install_patches: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("installPatches"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            properties: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("properties"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(o.extract_field("scope")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            visibility: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("visibility"),
            ),
            windows: pulumi_wasm_rust::__private::into_domain(o.extract_field("windows")),
        }
    }
}
