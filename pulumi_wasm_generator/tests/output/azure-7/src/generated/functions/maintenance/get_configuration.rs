pub mod get_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfigurationArgs {
        /// Specifies the name of the Maintenance Configuration.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group where this Maintenance Configuration exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
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
    pub fn invoke(args: GetConfigurationArgs) -> GetConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "inGuestUserPatchMode".into(),
                },
                register_interface::ResultField {
                    name: "installPatches".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "properties".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "visibility".into(),
                },
                register_interface::ResultField {
                    name: "windows".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetConfigurationResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            in_guest_user_patch_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inGuestUserPatchMode").unwrap(),
            ),
            install_patches: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("installPatches").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("properties").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            visibility: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("visibility").unwrap(),
            ),
            windows: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("windows").unwrap(),
            ),
        }
    }
}
