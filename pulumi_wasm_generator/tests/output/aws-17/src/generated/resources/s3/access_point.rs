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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod access_point {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessPointArgs {
        /// AWS account ID for the owner of the bucket for which you want to create an access point. Defaults to automatically determined account ID of the AWS provider.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of an AWS Partition S3 General Purpose Bucket or the ARN of S3 on Outposts Bucket that you want to associate this access point with.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::InputOrOutput<String>,
        /// AWS account ID associated with the S3 bucket associated with this access point.
        #[builder(into, default)]
        pub bucket_account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name you want to assign to this access point. See the [AWS documentation](https://docs.aws.amazon.com/AmazonS3/latest/userguide/creating-access-points.html?icmpid=docs_amazons3_console#access-points-names) for naming conditions.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Valid JSON document that specifies the policy that you want to apply to this access point. Removing `policy` from your configuration or setting `policy` to null or an empty string (i.e., `policy = ""`) _will not_ delete the policy since it could have been set by `aws.s3control.AccessPointPolicy`. To remove the `policy`, set it to `"{}"` (an empty JSON document).
        #[builder(into, default)]
        pub policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration block to manage the `PublicAccessBlock` configuration that you want to apply to this Amazon S3 bucket. You can enable the configuration options in any combination. Detailed below.
        #[builder(into, default)]
        pub public_access_block_configuration: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::s3::AccessPointPublicAccessBlockConfiguration>,
        >,
        /// Configuration block to restrict access to this access point to requests from the specified Virtual Private Cloud (VPC). Required for S3 on Outposts. Detailed below.
        #[builder(into, default)]
        pub vpc_configuration: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::s3::AccessPointVpcConfiguration>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccessPointResult {
        /// AWS account ID for the owner of the bucket for which you want to create an access point. Defaults to automatically determined account ID of the AWS provider.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Alias of the S3 Access Point.
        pub alias: pulumi_wasm_rust::Output<String>,
        /// ARN of the S3 Access Point.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name of an AWS Partition S3 General Purpose Bucket or the ARN of S3 on Outposts Bucket that you want to associate this access point with.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// AWS account ID associated with the S3 bucket associated with this access point.
        pub bucket_account_id: pulumi_wasm_rust::Output<String>,
        /// DNS domain name of the S3 Access Point in the format _`name`_-_`account_id`_.s3-accesspoint._region_.amazonaws.com.
        /// Note: S3 access points only support secure access by HTTPS. HTTP isn't supported.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// VPC endpoints for the S3 Access Point.
        pub endpoints: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Indicates whether this access point currently has a policy that allows public access.
        pub has_public_access_policy: pulumi_wasm_rust::Output<bool>,
        /// Name you want to assign to this access point. See the [AWS documentation](https://docs.aws.amazon.com/AmazonS3/latest/userguide/creating-access-points.html?icmpid=docs_amazons3_console#access-points-names) for naming conditions.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Indicates whether this access point allows access from the public Internet. Values are `VPC` (the access point doesn't allow access from the public Internet) and `Internet` (the access point allows access from the public Internet, subject to the access point and bucket access policies).
        pub network_origin: pulumi_wasm_rust::Output<String>,
        /// Valid JSON document that specifies the policy that you want to apply to this access point. Removing `policy` from your configuration or setting `policy` to null or an empty string (i.e., `policy = ""`) _will not_ delete the policy since it could have been set by `aws.s3control.AccessPointPolicy`. To remove the `policy`, set it to `"{}"` (an empty JSON document).
        pub policy: pulumi_wasm_rust::Output<String>,
        /// Configuration block to manage the `PublicAccessBlock` configuration that you want to apply to this Amazon S3 bucket. You can enable the configuration options in any combination. Detailed below.
        pub public_access_block_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::s3::AccessPointPublicAccessBlockConfiguration>,
        >,
        /// Configuration block to restrict access to this access point to requests from the specified Virtual Private Cloud (VPC). Required for S3 on Outposts. Detailed below.
        pub vpc_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::s3::AccessPointVpcConfiguration>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AccessPointArgs,
    ) -> AccessPointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let bucket_binding = args.bucket.get_output(context).get_inner();
        let bucket_account_id_binding = args
            .bucket_account_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let policy_binding = args.policy.get_output(context).get_inner();
        let public_access_block_configuration_binding = args
            .public_access_block_configuration
            .get_output(context)
            .get_inner();
        let vpc_configuration_binding = args
            .vpc_configuration
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/accessPoint:AccessPoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "bucketAccountId".into(),
                    value: &bucket_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "publicAccessBlockConfiguration".into(),
                    value: &public_access_block_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "vpcConfiguration".into(),
                    value: &vpc_configuration_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "alias".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "bucketAccountId".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "endpoints".into(),
                },
                register_interface::ResultField {
                    name: "hasPublicAccessPolicy".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkOrigin".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "publicAccessBlockConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "vpcConfiguration".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccessPointResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alias").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            bucket_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucketAccountId").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoints").unwrap(),
            ),
            has_public_access_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hasPublicAccessPolicy").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_origin: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkOrigin").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            public_access_block_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicAccessBlockConfiguration").unwrap(),
            ),
            vpc_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcConfiguration").unwrap(),
            ),
        }
    }
}
