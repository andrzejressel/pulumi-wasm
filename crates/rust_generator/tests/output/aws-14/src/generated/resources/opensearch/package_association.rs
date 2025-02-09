/// Manages an AWS Opensearch Package Association.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = package::create(
///         "example",
///         PackageArgs::builder()
///             .package_name("example-txt")
///             .package_source(
///                 PackagePackageSource::builder()
///                     .s3BucketName("${myOpensearchPackages.bucket}")
///                     .s3Key("${exampleAwsS3Object.key}")
///                     .build_struct(),
///             )
///             .package_type("TXT-DICTIONARY")
///             .build_struct(),
///     );
///     let examplePackageAssociation = package_association::create(
///         "examplePackageAssociation",
///         PackageAssociationArgs::builder()
///             .domain_name("${myDomain.domainName}")
///             .package_id("${example.id}")
///             .build_struct(),
///     );
///     let myDomain = domain::create(
///         "myDomain",
///         DomainArgs::builder()
///             .cluster_config(
///                 DomainClusterConfig::builder()
///                     .instanceType("r4.large.search")
///                     .build_struct(),
///             )
///             .domain_name("my-opensearch-domain")
///             .engine_version("Elasticsearch_7.10")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod package_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PackageAssociationArgs {
        /// Name of the domain to associate the package with.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Internal ID of the package to associate with a domain.
        #[builder(into)]
        pub package_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PackageAssociationResult {
        /// Name of the domain to associate the package with.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// Internal ID of the package to associate with a domain.
        pub package_id: pulumi_gestalt_rust::Output<String>,
        pub reference_path: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PackageAssociationArgs,
    ) -> PackageAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_name_binding = args.domain_name.get_output(context);
        let package_id_binding = args.package_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:opensearch/packageAssociation:PackageAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: domain_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "packageId".into(),
                    value: package_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PackageAssociationResult {
            domain_name: o.get_field("domainName"),
            package_id: o.get_field("packageId"),
            reference_path: o.get_field("referencePath"),
        }
    }
}
