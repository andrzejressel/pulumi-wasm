pub mod get_instance_profiles {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceProfilesArgs {
        /// IAM role name.
        #[builder(into)]
        pub role_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceProfilesResult {
        /// Set of ARNs of instance profiles.
        pub arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Set of IAM instance profile names.
        pub names: pulumi_wasm_rust::Output<Vec<String>>,
        /// Set of IAM instance profile paths.
        pub paths: pulumi_wasm_rust::Output<Vec<String>>,
        pub role_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetInstanceProfilesArgs,
    ) -> GetInstanceProfilesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let role_name_binding = args.role_name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iam/getInstanceProfiles:getInstanceProfiles".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "roleName".into(),
                    value: &role_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arns".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "names".into(),
                },
                register_interface::ResultField {
                    name: "paths".into(),
                },
                register_interface::ResultField {
                    name: "roleName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetInstanceProfilesResult {
            arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arns").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("names").unwrap(),
            ),
            paths: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("paths").unwrap(),
            ),
            role_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleName").unwrap(),
            ),
        }
    }
}
