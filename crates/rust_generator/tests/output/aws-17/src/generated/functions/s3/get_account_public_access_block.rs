#[allow(clippy::doc_lazy_continuation)]
pub mod get_account_public_access_block {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountPublicAccessBlockArgs {
        /// AWS account ID to configure. Defaults to automatically determined account ID of the AWS provider.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAccountPublicAccessBlockResult {
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether or not Amazon S3 should block public ACLs for buckets in this account is enabled. Returns as `true` or `false`.
        pub block_public_acls: pulumi_gestalt_rust::Output<bool>,
        /// Whether or not Amazon S3 should block public bucket policies for buckets in this account is enabled. Returns as `true` or `false`.
        pub block_public_policy: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Whether or not Amazon S3 should ignore public ACLs for buckets in this account is enabled. Returns as `true` or `false`.
        pub ignore_public_acls: pulumi_gestalt_rust::Output<bool>,
        /// Whether or not Amazon S3 should restrict public bucket policies for buckets in this account is enabled. Returns as `true` or `false`.
        pub restrict_public_buckets: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAccountPublicAccessBlockArgs,
    ) -> GetAccountPublicAccessBlockResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:s3/getAccountPublicAccessBlock:getAccountPublicAccessBlock"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAccountPublicAccessBlockResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            block_public_acls: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("blockPublicAcls"),
            ),
            block_public_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("blockPublicPolicy"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ignore_public_acls: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ignorePublicAcls"),
            ),
            restrict_public_buckets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("restrictPublicBuckets"),
            ),
        }
    }
}
