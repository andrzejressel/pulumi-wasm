pub mod get_testable_permissions {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTestablePermissionsArgs {
        /// The level of support for custom roles. Can be one of `"NOT_SUPPORTED"`, `"SUPPORTED"`, `"TESTING"`. Default is `"SUPPORTED"`
        #[builder(into, default)]
        pub custom_support_level: pulumi_wasm_rust::Output<Option<String>>,
        /// See [full resource name documentation](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more detail.
        #[builder(into)]
        pub full_resource_name: pulumi_wasm_rust::Output<String>,
        /// The acceptable release stages of the permission in the output. Note that `BETA` does not include permissions in `GA`, but you can specify both with `["GA", "BETA"]` for example. Can be a list of `"ALPHA"`, `"BETA"`, `"GA"`, `"DEPRECATED"`. Default is `["GA"]`.
        #[builder(into, default)]
        pub stages: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct GetTestablePermissionsResult {
        /// The the support level of this permission for custom roles.
        pub custom_support_level: pulumi_wasm_rust::Output<Option<String>>,
        pub full_resource_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A list of permissions matching the provided input. Structure is defined below.
        pub permissions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::iam::GetTestablePermissionsPermission>,
        >,
        pub stages: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetTestablePermissionsArgs) -> GetTestablePermissionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let custom_support_level_binding = args.custom_support_level.get_inner();
        let full_resource_name_binding = args.full_resource_name.get_inner();
        let stages_binding = args.stages.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:iam/getTestablePermissions:getTestablePermissions".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customSupportLevel".into(),
                    value: &custom_support_level_binding,
                },
                register_interface::ObjectField {
                    name: "fullResourceName".into(),
                    value: &full_resource_name_binding,
                },
                register_interface::ObjectField {
                    name: "stages".into(),
                    value: &stages_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "customSupportLevel".into(),
                },
                register_interface::ResultField {
                    name: "fullResourceName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "permissions".into(),
                },
                register_interface::ResultField {
                    name: "stages".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTestablePermissionsResult {
            custom_support_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customSupportLevel").unwrap(),
            ),
            full_resource_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fullResourceName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissions").unwrap(),
            ),
            stages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stages").unwrap(),
            ),
        }
    }
}
