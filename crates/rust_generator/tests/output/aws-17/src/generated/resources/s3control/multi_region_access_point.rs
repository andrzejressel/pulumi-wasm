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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MultiRegionAccessPointArgs,
    ) -> MultiRegionAccessPointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let details_binding = args.details.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3control/multiRegionAccessPoint:MultiRegionAccessPoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "details".into(),
                    value: &details_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MultiRegionAccessPointResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            alias: pulumi_gestalt_rust::__private::into_domain(o.extract_field("alias")),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            details: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("details"),
            ),
            domain_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainName"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
        }
    }
}
