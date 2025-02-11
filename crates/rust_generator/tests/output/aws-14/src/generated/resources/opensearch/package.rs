/// Manages an AWS Opensearch Package.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   myOpensearchPackages:
///     type: aws:s3:BucketV2
///     name: my_opensearch_packages
///     properties:
///       bucket: my-opensearch-packages
///   example:
///     type: aws:s3:BucketObjectv2
///     properties:
///       bucket: ${myOpensearchPackages.bucket}
///       key: example.txt
///       source:
///         fn::FileAsset: ./example.txt
///       etag:
///         fn::invoke:
///           function: std:filemd5
///           arguments:
///             input: ./example.txt
///           return: result
///   examplePackage:
///     type: aws:opensearch:Package
///     name: example
///     properties:
///       packageName: example-txt
///       packageSource:
///         s3BucketName: ${myOpensearchPackages.bucket}
///         s3Key: ${example.key}
///       packageType: TXT-DICTIONARY
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS Opensearch Packages using the Package ID. For example:
///
/// ```sh
/// $ pulumi import aws:opensearch/package:Package example package-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod package {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PackageArgs {
        /// Description of the package.
        #[builder(into, default)]
        pub package_description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique name for the package.
        #[builder(into)]
        pub package_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block for the package source options.
        #[builder(into)]
        pub package_source: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::opensearch::PackagePackageSource,
        >,
        /// The type of package.
        #[builder(into)]
        pub package_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PackageResult {
        /// The current version of the package.
        pub available_package_version: pulumi_gestalt_rust::Output<String>,
        /// Description of the package.
        pub package_description: pulumi_gestalt_rust::Output<Option<String>>,
        pub package_id: pulumi_gestalt_rust::Output<String>,
        /// Unique name for the package.
        pub package_name: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for the package source options.
        pub package_source: pulumi_gestalt_rust::Output<
            super::super::types::opensearch::PackagePackageSource,
        >,
        /// The type of package.
        pub package_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PackageArgs,
    ) -> PackageResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let package_description_binding = args.package_description.get_output(context);
        let package_name_binding = args.package_name.get_output(context);
        let package_source_binding = args.package_source.get_output(context);
        let package_type_binding = args.package_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:opensearch/package:Package".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "packageDescription".into(),
                    value: &package_description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "packageName".into(),
                    value: &package_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "packageSource".into(),
                    value: &package_source_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "packageType".into(),
                    value: &package_type_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PackageResult {
            available_package_version: o.get_field("availablePackageVersion"),
            package_description: o.get_field("packageDescription"),
            package_id: o.get_field("packageId"),
            package_name: o.get_field("packageName"),
            package_source: o.get_field("packageSource"),
            package_type: o.get_field("packageType"),
        }
    }
}
