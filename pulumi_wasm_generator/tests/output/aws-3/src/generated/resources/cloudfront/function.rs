/// Provides a CloudFront Function resource. With CloudFront Functions in Amazon CloudFront, you can write lightweight functions in JavaScript for high-scale, latency-sensitive CDN customizations.
///
/// See [CloudFront Functions](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-functions.html)
///
/// > **NOTE:** You cannot delete a function if it’s associated with a cache behavior. First, update your distributions to remove the function association from all cache behaviors, then delete the function.
///
/// ## Example Usage
///
/// ## Import
///
/// Using `pulumi import`, import CloudFront Functions using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/function:Function test my_test_function
/// ```
pub mod function {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FunctionArgs {
        /// Source code of the function
        #[builder(into)]
        pub code: pulumi_wasm_rust::Output<String>,
        /// Comment.
        #[builder(into, default)]
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// List of `aws.cloudfront.KeyValueStore` ARNs to be associated to the function. AWS limits associations to on key value store per function.
        #[builder(into, default)]
        pub key_value_store_associations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Unique name for your CloudFront Function.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to publish creation/change as Live CloudFront Function Version. Defaults to `true`.
        #[builder(into, default)]
        pub publish: pulumi_wasm_rust::Output<Option<bool>>,
        /// Identifier of the function's runtime. Valid values are `cloudfront-js-1.0` and `cloudfront-js-2.0`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub runtime: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct FunctionResult {
        /// Amazon Resource Name (ARN) identifying your CloudFront Function.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Source code of the function
        pub code: pulumi_wasm_rust::Output<String>,
        /// Comment.
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// ETag hash of the function. This is the value for the `DEVELOPMENT` stage of the function.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// List of `aws.cloudfront.KeyValueStore` ARNs to be associated to the function. AWS limits associations to on key value store per function.
        pub key_value_store_associations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// ETag hash of any `LIVE` stage of the function.
        pub live_stage_etag: pulumi_wasm_rust::Output<String>,
        /// Unique name for your CloudFront Function.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether to publish creation/change as Live CloudFront Function Version. Defaults to `true`.
        pub publish: pulumi_wasm_rust::Output<Option<bool>>,
        /// Identifier of the function's runtime. Valid values are `cloudfront-js-1.0` and `cloudfront-js-2.0`.
        ///
        /// The following arguments are optional:
        pub runtime: pulumi_wasm_rust::Output<String>,
        /// Status of the function. Can be `UNPUBLISHED`, `UNASSOCIATED` or `ASSOCIATED`.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FunctionArgs) -> FunctionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let code_binding = args.code.get_inner();
        let comment_binding = args.comment.get_inner();
        let key_value_store_associations_binding = args
            .key_value_store_associations
            .get_inner();
        let name_binding = args.name.get_inner();
        let publish_binding = args.publish.get_inner();
        let runtime_binding = args.runtime.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudfront/function:Function".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "code".into(),
                    value: &code_binding,
                },
                register_interface::ObjectField {
                    name: "comment".into(),
                    value: &comment_binding,
                },
                register_interface::ObjectField {
                    name: "keyValueStoreAssociations".into(),
                    value: &key_value_store_associations_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publish".into(),
                    value: &publish_binding,
                },
                register_interface::ObjectField {
                    name: "runtime".into(),
                    value: &runtime_binding,
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
                    name: "keyValueStoreAssociations".into(),
                },
                register_interface::ResultField {
                    name: "liveStageEtag".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publish".into(),
                },
                register_interface::ResultField {
                    name: "runtime".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FunctionResult {
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
            key_value_store_associations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyValueStoreAssociations").unwrap(),
            ),
            live_stage_etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("liveStageEtag").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            publish: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publish").unwrap(),
            ),
            runtime: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runtime").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
