/// Provides a resource to manage an [Amazon Macie Classification Export Configuration](https://docs.aws.amazon.com/macie/latest/APIReference/classification-export-configuration.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create("example", AccountArgs::builder().build_struct());
///     let exampleClassificationExportConfiguration = classification_export_configuration::create(
///         "exampleClassificationExportConfiguration",
///         ClassificationExportConfigurationArgs::builder()
///             .s_3_destination(
///                 ClassificationExportConfigurationS3Destination::builder()
///                     .bucketName("${exampleAwsS3Bucket.bucket}")
///                     .keyPrefix("exampleprefix/")
///                     .kmsKeyArn("${exampleAwsKmsKey.arn}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_macie2_classification_export_configuration` using the account ID and region. For example:
///
/// ```sh
/// $ pulumi import aws:macie2/classificationExportConfiguration:ClassificationExportConfiguration example 123456789012:us-west-2
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod classification_export_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClassificationExportConfigurationArgs {
        /// Configuration block for a S3 Destination. Defined below
        #[builder(into, default)]
        pub s3_destination: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::macie2::ClassificationExportConfigurationS3Destination,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ClassificationExportConfigurationResult {
        /// Configuration block for a S3 Destination. Defined below
        pub s3_destination: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::macie2::ClassificationExportConfigurationS3Destination,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ClassificationExportConfigurationArgs,
    ) -> ClassificationExportConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let s3_destination_binding = args.s3_destination.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:macie2/classificationExportConfiguration:ClassificationExportConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "s3Destination".into(),
                    value: &s3_destination_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClassificationExportConfigurationResult {
            s3_destination: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("s3Destination"),
            ),
        }
    }
}
