pub mod get_origin_access_identities {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOriginAccessIdentitiesArgs {
        /// Filter origin access identities by comment.
        #[builder(into, default)]
        pub comments: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct GetOriginAccessIdentitiesResult {
        pub comments: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Set of ARNs of the matched origin access identities.
        pub iam_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Set of ids of the matched origin access identities.
        pub ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Set of S3 canonical user IDs of the matched origin access identities.
        pub s3_canonical_user_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetOriginAccessIdentitiesArgs,
    ) -> GetOriginAccessIdentitiesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let comments_binding = args.comments.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudfront/getOriginAccessIdentities:getOriginAccessIdentities"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "comments".into(),
                    value: &comments_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "comments".into(),
                },
                register_interface::ResultField {
                    name: "iamArns".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ids".into(),
                },
                register_interface::ResultField {
                    name: "s3CanonicalUserIds".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetOriginAccessIdentitiesResult {
            comments: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("comments").unwrap(),
            ),
            iam_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamArns").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ids").unwrap(),
            ),
            s3_canonical_user_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3CanonicalUserIds").unwrap(),
            ),
        }
    }
}
