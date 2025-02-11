/// Provides a resource to manage an S3 Multi-Region Access Point associated with specified buckets.
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ### Multiple AWS Buckets in Different Regions
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let barBucket = bucket_v_2::create(
///         "barBucket",
///         BucketV2Args::builder().bucket("example-bucket-bar").build_struct(),
///     );
///     let example = multi_region_access_point::create(
///         "example",
///         MultiRegionAccessPointArgs::builder()
///             .details(
///                 MultiRegionAccessPointDetails::builder()
///                     .name("example")
///                     .regions(
///                         vec![
///                             MultiRegionAccessPointDetailsRegion::builder()
///                             .bucket("${fooBucket.id}").build_struct(),
///                             MultiRegionAccessPointDetailsRegion::builder()
///                             .bucket("${barBucket.id}").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let fooBucket = bucket_v_2::create(
///         "fooBucket",
///         BucketV2Args::builder().bucket("example-bucket-foo").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Multi-Region Access Points using the `account_id` and `name` of the Multi-Region Access Point separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:s3control/multiRegionAccessPoint:MultiRegionAccessPoint example 123456789012:example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod multi_region_access_point {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MultiRegionAccessPointArgs {
        /// The AWS account ID for the owner of the buckets for which you want to create a Multi-Region Access Point. Defaults to automatically determined account ID of the AWS provider.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A configuration block containing details about the Multi-Region Access Point. See Details Configuration Block below for more details
        #[builder(into)]
        pub details: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::s3control::MultiRegionAccessPointDetails,
        >,
    }
    #[allow(dead_code)]
    pub struct MultiRegionAccessPointResult {
        /// The AWS account ID for the owner of the buckets for which you want to create a Multi-Region Access Point. Defaults to automatically determined account ID of the AWS provider.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The alias for the Multi-Region Access Point.
        pub alias: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the Multi-Region Access Point.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A configuration block containing details about the Multi-Region Access Point. See Details Configuration Block below for more details
        pub details: pulumi_gestalt_rust::Output<
            super::super::types::s3control::MultiRegionAccessPointDetails,
        >,
        /// The DNS domain name of the S3 Multi-Region Access Point in the format _`alias`_.accesspoint.s3-global.amazonaws.com. For more information, see the documentation on [Multi-Region Access Point Requests](https://docs.aws.amazon.com/AmazonS3/latest/userguide/MultiRegionAccessPointRequests.html).
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// The current status of the Multi-Region Access Point. One of: `READY`, `INCONSISTENT_ACROSS_REGIONS`, `CREATING`, `PARTIALLY_CREATED`, `PARTIALLY_DELETED`, `DELETING`.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MultiRegionAccessPointArgs,
    ) -> MultiRegionAccessPointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let details_binding = args.details.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3control/multiRegionAccessPoint:MultiRegionAccessPoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "details".into(),
                    value: &details_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MultiRegionAccessPointResult {
            account_id: o.get_field("accountId"),
            alias: o.get_field("alias"),
            arn: o.get_field("arn"),
            details: o.get_field("details"),
            domain_name: o.get_field("domainName"),
            status: o.get_field("status"),
        }
    }
}
