pub mod get_resource_collection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResourceCollectionArgs {
        /// A collection of AWS CloudFormation stacks. See `cloudformation` below for additional details.
        #[builder(into, default)]
        pub cloudformations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::devopsguru::GetResourceCollectionCloudformation,
                >,
            >,
        >,
        /// AWS tags used to filter the resources in the resource collection. See `tags` below for additional details.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::devopsguru::GetResourceCollectionTag>>,
        >,
        /// Type of AWS resource collection to create. Valid values are `AWS_CLOUD_FORMATION`, `AWS_SERVICE`, and `AWS_TAGS`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetResourceCollectionResult {
        /// A collection of AWS CloudFormation stacks. See `cloudformation` below for additional details.
        pub cloudformations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::devopsguru::GetResourceCollectionCloudformation,
                >,
            >,
        >,
        /// Type of AWS resource collection to create (same value as `type`).
        pub id: pulumi_wasm_rust::Output<String>,
        /// AWS tags used to filter the resources in the resource collection. See `tags` below for additional details.
        pub tags: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::devopsguru::GetResourceCollectionTag>>,
        >,
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetResourceCollectionArgs) -> GetResourceCollectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cloudformations_binding = args.cloudformations.get_inner();
        let tags_binding = args.tags.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:devopsguru/getResourceCollection:getResourceCollection".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cloudformations".into(),
                    value: &cloudformations_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cloudformations".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetResourceCollectionResult {
            cloudformations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudformations").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}