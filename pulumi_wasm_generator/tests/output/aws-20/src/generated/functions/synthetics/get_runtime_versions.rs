pub mod get_runtime_versions {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRuntimeVersionsArgs {
        /// List of runtime versions. See `runtime_versions` attribute reference.
        #[builder(into, default)]
        pub runtime_versions: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::synthetics::GetRuntimeVersionsRuntimeVersion,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetRuntimeVersionsResult {
        /// Name of the AWS region from which runtime versions are fetched.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of runtime versions. See `runtime_versions` attribute reference.
        pub runtime_versions: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::synthetics::GetRuntimeVersionsRuntimeVersion,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRuntimeVersionsArgs) -> GetRuntimeVersionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let runtime_versions_binding = args.runtime_versions.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:synthetics/getRuntimeVersions:getRuntimeVersions".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "runtimeVersions".into(),
                    value: &runtime_versions_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "runtimeVersions".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRuntimeVersionsResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            runtime_versions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runtimeVersions").unwrap(),
            ),
        }
    }
}
