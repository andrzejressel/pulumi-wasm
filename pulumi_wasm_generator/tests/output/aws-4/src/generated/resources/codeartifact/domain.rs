/// Provides a CodeArtifact Domain Resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = domain::create(
///         "example",
///         DomainArgs::builder().domain("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeArtifact Domain using the CodeArtifact Domain arn. For example:
///
/// ```sh
/// $ pulumi import aws:codeartifact/domain:Domain example arn:aws:codeartifact:us-west-2:012345678912:domain/tf-acc-test-8593714120730241305
/// ```
pub mod domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainArgs {
        /// The name of the domain to create. All domain names in an AWS Region that are in the same AWS account must be unique. The domain name is used as the prefix in DNS hostnames. Do not use sensitive information in a domain name because it is publicly discoverable.
        #[builder(into)]
        pub domain: pulumi_wasm_rust::InputOrOutput<String>,
        /// The encryption key for the domain. This is used to encrypt content stored in a domain. The KMS Key Amazon Resource Name (ARN). The default aws/codeartifact AWS KMS master key is used if this element is absent.
        #[builder(into, default)]
        pub encryption_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainResult {
        /// The ARN of the Domain.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The total size of all assets in the domain.
        pub asset_size_bytes: pulumi_wasm_rust::Output<String>,
        /// A timestamp that represents the date and time the domain was created in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub created_time: pulumi_wasm_rust::Output<String>,
        /// The name of the domain to create. All domain names in an AWS Region that are in the same AWS account must be unique. The domain name is used as the prefix in DNS hostnames. Do not use sensitive information in a domain name because it is publicly discoverable.
        pub domain: pulumi_wasm_rust::Output<String>,
        /// The encryption key for the domain. This is used to encrypt content stored in a domain. The KMS Key Amazon Resource Name (ARN). The default aws/codeartifact AWS KMS master key is used if this element is absent.
        pub encryption_key: pulumi_wasm_rust::Output<String>,
        /// The AWS account ID that owns the domain.
        pub owner: pulumi_wasm_rust::Output<String>,
        /// The number of repositories in the domain.
        pub repository_count: pulumi_wasm_rust::Output<i32>,
        /// The ARN of the Amazon S3 bucket that is used to store package assets in the domain.
        pub s3_bucket_arn: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DomainArgs,
    ) -> DomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_binding = args.domain.get_output(context).get_inner();
        let encryption_key_binding = args.encryption_key.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codeartifact/domain:Domain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionKey".into(),
                    value: &encryption_key_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DomainResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            asset_size_bytes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("assetSizeBytes"),
            ),
            created_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdTime"),
            ),
            domain: pulumi_wasm_rust::__private::into_domain(o.extract_field("domain")),
            encryption_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encryptionKey"),
            ),
            owner: pulumi_wasm_rust::__private::into_domain(o.extract_field("owner")),
            repository_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("repositoryCount"),
            ),
            s3_bucket_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("s3BucketArn"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
