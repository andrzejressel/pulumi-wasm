pub mod get_function {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFunctionArgs {
        /// Name of the CloudFront function.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Function’s stage, either `DEVELOPMENT` or `LIVE`.
        #[builder(into)]
        pub stage: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetFunctionArgs,
    ) -> GetFunctionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let stage_binding = args.stage.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFunctionResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            code: pulumi_wasm_rust::__private::into_domain(o.extract_field("code")),
            comment: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("comment"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            key_value_store_associations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyValueStoreAssociations"),
            ),
            last_modified_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastModifiedTime"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            runtime: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("runtime"),
            ),
            stage: pulumi_wasm_rust::__private::into_domain(o.extract_field("stage")),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
        }
    }
}
