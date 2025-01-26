pub mod get_multi_region_access_point {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMultiRegionAccessPointArgs {
        /// The AWS account ID of the S3 Multi-Region Access Point. Defaults to automatically determined account ID of the AWS provider.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Multi-Region Access Point.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetMultiRegionAccessPointResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The alias for the Multi-Region Access Point.
        pub alias: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the Multi-Region Access Point.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Timestamp when the resource has been created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// The DNS domain name of the S3 Multi-Region Access Point in the format _`alias`_.accesspoint.s3-global.amazonaws.com. For more information, see the documentation on [Multi-Region Access Point Requests](https://docs.aws.amazon.com/AmazonS3/latest/userguide/MultiRegionAccessPointRequests.html).
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Public Access Block of the Multi-Region Access Point. Detailed below.
        pub public_access_blocks: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::s3control::GetMultiRegionAccessPointPublicAccessBlock,
            >,
        >,
        /// A collection of the regions and buckets associated with the Multi-Region Access Point.
        pub regions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::s3control::GetMultiRegionAccessPointRegion>,
        >,
        /// The current status of the Multi-Region Access Point.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetMultiRegionAccessPointArgs,
    ) -> GetMultiRegionAccessPointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:s3control/getMultiRegionAccessPoint:getMultiRegionAccessPoint"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetMultiRegionAccessPointResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            alias: pulumi_wasm_rust::__private::into_domain(o.extract_field("alias")),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            created_at: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdAt"),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("domainName"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            public_access_blocks: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicAccessBlocks"),
            ),
            regions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("regions"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
        }
    }
}
