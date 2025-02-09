#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_origin_access_identities {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOriginAccessIdentitiesArgs {
        /// Filter origin access identities by comment.
        #[builder(into, default)]
        pub comments: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct GetOriginAccessIdentitiesResult {
        pub comments: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Set of ARNs of the matched origin access identities.
        pub iam_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Set of ids of the matched origin access identities.
        pub ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Set of S3 canonical user IDs of the matched origin access identities.
        pub s3_canonical_user_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetOriginAccessIdentitiesArgs,
    ) -> GetOriginAccessIdentitiesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let comments_binding_1 = args.comments.get_output(context);
        let comments_binding = comments_binding_1.get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetOriginAccessIdentitiesResult {
            comments: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("comments"),
            ),
            iam_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iamArns"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ids: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ids")),
            s3_canonical_user_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("s3CanonicalUserIds"),
            ),
        }
    }
}
