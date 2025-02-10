/// Provides a resource to manage an S3 Access Point.
///
/// > **NOTE on Access Points and Access Point Policies:** This provider provides both a standalone Access Point Policy resource and an Access Point resource with a resource policy defined in-line. You cannot use an Access Point with in-line resource policy in conjunction with an Access Point Policy resource. Doing so will cause a conflict of policies and will overwrite the access point's resource policy.
///
/// > Advanced usage: To use a custom API endpoint for this resource, use the `s3control` endpoint provider configuration), not the `s3` endpoint provider configuration.
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ### AWS Partition General Purpose Bucket
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_v_2::create(
///         "example",
///         BucketV2Args::builder().bucket("example").build_struct(),
///     );
///     let exampleAccessPoint = access_point::create(
///         "exampleAccessPoint",
///         AccessPointArgs::builder().bucket("${example.id}").name("example").build_struct(),
///     );
/// }
/// ```
///
/// ### S3 on Outposts Bucket
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket::create(
///         "example",
///         BucketArgs::builder().bucket("example").build_struct(),
///     );
///     let exampleAccessPoint = access_point::create(
///         "exampleAccessPoint",
///         AccessPointArgs::builder()
///             .bucket("${example.arn}")
///             .name("example")
///             .vpc_configuration(
///                 AccessPointVpcConfiguration::builder()
///                     .vpcId("${exampleVpc.id}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleVpc = vpc::create(
///         "exampleVpc",
///         VpcArgs::builder().cidr_block("10.0.0.0/16").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Import using the ARN for Access Points associated with an S3 on Outposts Bucket:
///
/// __Using `pulumi import` to import.__ For example:
///
/// Import using the `account_id` and `name` separated by a colon (`:`) for Access Points associated with an AWS Partition S3 Bucket:
///
/// ```sh
/// $ pulumi import aws:s3/accessPoint:AccessPoint example 123456789012:example
/// ```
/// Import using the ARN for Access Points associated with an S3 on Outposts Bucket:
///
/// ```sh
/// $ pulumi import aws:s3/accessPoint:AccessPoint example arn:aws:s3-outposts:us-east-1:123456789012:outpost/op-1234567890123456/accesspoint/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod access_point {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessPointArgs {
        /// AWS account ID for the owner of the bucket for which you want to create an access point. Defaults to automatically determined account ID of the AWS provider.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of an AWS Partition S3 General Purpose Bucket or the ARN of S3 on Outposts Bucket that you want to associate this access point with.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// AWS account ID associated with the S3 bucket associated with this access point.
        #[builder(into, default)]
        pub bucket_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name you want to assign to this access point. See the [AWS documentation](https://docs.aws.amazon.com/AmazonS3/latest/userguide/creating-access-points.html?icmpid=docs_amazons3_console#access-points-names) for naming conditions.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Valid JSON document that specifies the policy that you want to apply to this access point. Removing `policy` from your configuration or setting `policy` to null or an empty string (i.e., `policy = ""`) _will not_ delete the policy since it could have been set by `aws.s3control.AccessPointPolicy`. To remove the `policy`, set it to `"{}"` (an empty JSON document).
        #[builder(into, default)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block to manage the `PublicAccessBlock` configuration that you want to apply to this Amazon S3 bucket. You can enable the configuration options in any combination. Detailed below.
        #[builder(into, default)]
        pub public_access_block_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3::AccessPointPublicAccessBlockConfiguration>,
        >,
        /// Configuration block to restrict access to this access point to requests from the specified Virtual Private Cloud (VPC). Required for S3 on Outposts. Detailed below.
        #[builder(into, default)]
        pub vpc_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3::AccessPointVpcConfiguration>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccessPointResult {
        /// AWS account ID for the owner of the bucket for which you want to create an access point. Defaults to automatically determined account ID of the AWS provider.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Alias of the S3 Access Point.
        pub alias: pulumi_gestalt_rust::Output<String>,
        /// ARN of the S3 Access Point.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of an AWS Partition S3 General Purpose Bucket or the ARN of S3 on Outposts Bucket that you want to associate this access point with.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID associated with the S3 bucket associated with this access point.
        pub bucket_account_id: pulumi_gestalt_rust::Output<String>,
        /// DNS domain name of the S3 Access Point in the format _`name`_-_`account_id`_.s3-accesspoint._region_.amazonaws.com.
        /// Note: S3 access points only support secure access by HTTPS. HTTP isn't supported.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// VPC endpoints for the S3 Access Point.
        pub endpoints: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Indicates whether this access point currently has a policy that allows public access.
        pub has_public_access_policy: pulumi_gestalt_rust::Output<bool>,
        /// Name you want to assign to this access point. See the [AWS documentation](https://docs.aws.amazon.com/AmazonS3/latest/userguide/creating-access-points.html?icmpid=docs_amazons3_console#access-points-names) for naming conditions.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether this access point allows access from the public Internet. Values are `VPC` (the access point doesn't allow access from the public Internet) and `Internet` (the access point allows access from the public Internet, subject to the access point and bucket access policies).
        pub network_origin: pulumi_gestalt_rust::Output<String>,
        /// Valid JSON document that specifies the policy that you want to apply to this access point. Removing `policy` from your configuration or setting `policy` to null or an empty string (i.e., `policy = ""`) _will not_ delete the policy since it could have been set by `aws.s3control.AccessPointPolicy`. To remove the `policy`, set it to `"{}"` (an empty JSON document).
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// Configuration block to manage the `PublicAccessBlock` configuration that you want to apply to this Amazon S3 bucket. You can enable the configuration options in any combination. Detailed below.
        pub public_access_block_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::s3::AccessPointPublicAccessBlockConfiguration>,
        >,
        /// Configuration block to restrict access to this access point to requests from the specified Virtual Private Cloud (VPC). Required for S3 on Outposts. Detailed below.
        pub vpc_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::s3::AccessPointVpcConfiguration>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessPointArgs,
    ) -> AccessPointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let bucket_binding = args.bucket.get_output(context);
        let bucket_account_id_binding = args.bucket_account_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let public_access_block_configuration_binding = args
            .public_access_block_configuration
            .get_output(context);
        let vpc_configuration_binding = args.vpc_configuration.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3/accessPoint:AccessPoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucketAccountId".into(),
                    value: bucket_account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicAccessBlockConfiguration".into(),
                    value: public_access_block_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcConfiguration".into(),
                    value: vpc_configuration_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccessPointResult {
            account_id: o.get_field("accountId"),
            alias: o.get_field("alias"),
            arn: o.get_field("arn"),
            bucket: o.get_field("bucket"),
            bucket_account_id: o.get_field("bucketAccountId"),
            domain_name: o.get_field("domainName"),
            endpoints: o.get_field("endpoints"),
            has_public_access_policy: o.get_field("hasPublicAccessPolicy"),
            name: o.get_field("name"),
            network_origin: o.get_field("networkOrigin"),
            policy: o.get_field("policy"),
            public_access_block_configuration: o
                .get_field("publicAccessBlockConfiguration"),
            vpc_configuration: o.get_field("vpcConfiguration"),
        }
    }
}
