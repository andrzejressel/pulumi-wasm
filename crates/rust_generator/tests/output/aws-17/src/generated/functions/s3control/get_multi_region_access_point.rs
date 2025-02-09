#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_multi_region_access_point {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMultiRegionAccessPointArgs {
        /// The AWS account ID of the S3 Multi-Region Access Point. Defaults to automatically determined account ID of the AWS provider.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Multi-Region Access Point.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetMultiRegionAccessPointResult {
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The alias for the Multi-Region Access Point.
        pub alias: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the Multi-Region Access Point.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Timestamp when the resource has been created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The DNS domain name of the S3 Multi-Region Access Point in the format _`alias`_.accesspoint.s3-global.amazonaws.com. For more information, see the documentation on [Multi-Region Access Point Requests](https://docs.aws.amazon.com/AmazonS3/latest/userguide/MultiRegionAccessPointRequests.html).
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Public Access Block of the Multi-Region Access Point. Detailed below.
        pub public_access_blocks: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::s3control::GetMultiRegionAccessPointPublicAccessBlock,
            >,
        >,
        /// A collection of the regions and buckets associated with the Multi-Region Access Point.
        pub regions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::s3control::GetMultiRegionAccessPointRegion>,
        >,
        /// The current status of the Multi-Region Access Point.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetMultiRegionAccessPointArgs,
    ) -> GetMultiRegionAccessPointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:s3control/getMultiRegionAccessPoint:getMultiRegionAccessPoint"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetMultiRegionAccessPointResult {
            account_id: o.get_field("accountId"),
            alias: o.get_field("alias"),
            arn: o.get_field("arn"),
            created_at: o.get_field("createdAt"),
            domain_name: o.get_field("domainName"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            public_access_blocks: o.get_field("publicAccessBlocks"),
            regions: o.get_field("regions"),
            status: o.get_field("status"),
        }
    }
}
