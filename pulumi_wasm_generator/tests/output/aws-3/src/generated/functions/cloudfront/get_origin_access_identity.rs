pub mod get_origin_access_identity {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOriginAccessIdentityArgs {
        /// The identifier for the origin access identity. For example: `E1ZAKK699EOLAL`.
        #[builder(into)]
        pub id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetOriginAccessIdentityResult {
        /// Internal value used by CloudFront to allow future
        /// updates to the origin access identity.
        pub caller_reference: pulumi_wasm_rust::Output<String>,
        /// A shortcut to the full path for the
        /// origin access identity to use in CloudFront, see below.
        pub cloudfront_access_identity_path: pulumi_wasm_rust::Output<String>,
        /// An optional comment for the origin access identity.
        pub comment: pulumi_wasm_rust::Output<String>,
        /// Current version of the origin access identity's information.
        /// For example: `E2QWRUHAPOMQZL`.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Pre-generated ARN for use in S3 bucket policies (see below).
        /// Example: `arn:aws:iam::cloudfront:user/CloudFront Origin Access Identity
        /// E2QWRUHAPOMQZL`.
        pub iam_arn: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Amazon S3 canonical user ID for the origin
        /// access identity, which you use when giving the origin access identity read
        /// permission to an object in Amazon S3.
        pub s3_canonical_user_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetOriginAccessIdentityArgs,
    ) -> GetOriginAccessIdentityResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudfront/getOriginAccessIdentity:getOriginAccessIdentity"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "callerReference".into(),
                },
                register_interface::ResultField {
                    name: "cloudfrontAccessIdentityPath".into(),
                },
                register_interface::ResultField {
                    name: "comment".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "iamArn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "s3CanonicalUserId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetOriginAccessIdentityResult {
            caller_reference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("callerReference").unwrap(),
            ),
            cloudfront_access_identity_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudfrontAccessIdentityPath").unwrap(),
            ),
            comment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("comment").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            iam_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamArn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            s3_canonical_user_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3CanonicalUserId").unwrap(),
            ),
        }
    }
}
