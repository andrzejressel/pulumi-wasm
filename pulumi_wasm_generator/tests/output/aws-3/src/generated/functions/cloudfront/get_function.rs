pub mod get_function {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFunctionArgs {
        /// Name of the CloudFront function.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Function’s stage, either `DEVELOPMENT` or `LIVE`.
        #[builder(into)]
        pub stage: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetFunctionResult {
        /// ARN identifying your CloudFront Function.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Source code of the function
        pub code: pulumi_wasm_rust::Output<String>,
        /// Comment.
        pub comment: pulumi_wasm_rust::Output<String>,
        /// ETag hash of the function
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of `aws.cloudfront.KeyValueStore` ARNs associated to the function.
        pub key_value_store_associations: pulumi_wasm_rust::Output<Vec<String>>,
        /// When this resource was last modified.
        pub last_modified_time: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Identifier of the function's runtime.
        pub runtime: pulumi_wasm_rust::Output<String>,
        pub stage: pulumi_wasm_rust::Output<String>,
        /// Status of the function. Can be `UNPUBLISHED`, `UNASSOCIATED` or `ASSOCIATED`.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetFunctionArgs) -> GetFunctionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let stage_binding = args.stage.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudfront/getFunction:getFunction".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "stage".into(),
                    value: &stage_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "code".into(),
                },
                register_interface::ResultField {
                    name: "comment".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "keyValueStoreAssociations".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedTime".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "runtime".into(),
                },
                register_interface::ResultField {
                    name: "stage".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFunctionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("code").unwrap(),
            ),
            comment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("comment").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            key_value_store_associations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyValueStoreAssociations").unwrap(),
            ),
            last_modified_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedTime").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            runtime: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runtime").unwrap(),
            ),
            stage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stage").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
