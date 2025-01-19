pub mod get_supported_instance_types {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSupportedInstanceTypesArgs {
        /// Amazon EMR release label. For more information about Amazon EMR releases and their included application versions and features, see the [Amazon EMR Release Guide](https://docs.aws.amazon.com/emr/latest/ReleaseGuide/emr-release-components.html).
        #[builder(into)]
        pub release_label: pulumi_wasm_rust::Output<String>,
        /// List of supported instance types. See `supported_instance_types` below.
        #[builder(into, default)]
        pub supported_instance_types: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::emr::GetSupportedInstanceTypesSupportedInstanceType,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSupportedInstanceTypesResult {
        pub id: pulumi_wasm_rust::Output<String>,
        pub release_label: pulumi_wasm_rust::Output<String>,
        /// List of supported instance types. See `supported_instance_types` below.
        pub supported_instance_types: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::emr::GetSupportedInstanceTypesSupportedInstanceType,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetSupportedInstanceTypesArgs,
    ) -> GetSupportedInstanceTypesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let release_label_binding = args.release_label.get_inner();
        let supported_instance_types_binding = args.supported_instance_types.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:emr/getSupportedInstanceTypes:getSupportedInstanceTypes".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "releaseLabel".into(),
                    value: &release_label_binding,
                },
                register_interface::ObjectField {
                    name: "supportedInstanceTypes".into(),
                    value: &supported_instance_types_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "releaseLabel".into(),
                },
                register_interface::ResultField {
                    name: "supportedInstanceTypes".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSupportedInstanceTypesResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            release_label: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("releaseLabel").unwrap(),
            ),
            supported_instance_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedInstanceTypes").unwrap(),
            ),
        }
    }
}
