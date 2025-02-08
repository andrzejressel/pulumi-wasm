/// Provides a resource to manage an S3 Object Lambda Access Point.
/// An Object Lambda access point is associated with exactly one standard access point and thus one Amazon S3 bucket.
///
/// ## Example Usage
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
///     let exampleObjectLambdaAccessPoint = object_lambda_access_point::create(
///         "exampleObjectLambdaAccessPoint",
///         ObjectLambdaAccessPointArgs::builder()
///             .configuration(
///                 ObjectLambdaAccessPointConfiguration::builder()
///                     .supportingAccessPoint("${exampleAccessPoint.arn}")
///                     .transformationConfigurations(
///                         vec![
///                             ObjectLambdaAccessPointConfigurationTransformationConfiguration::builder()
///                             .actions(vec!["GetObject",])
///                             .contentTransformation(ObjectLambdaAccessPointConfigurationTransformationConfigurationContentTransformation::builder()
///                             .awsLambda(ObjectLambdaAccessPointConfigurationTransformationConfigurationContentTransformationAwsLambda::builder()
///                             .functionArn("${exampleAwsLambdaFunction.arn}")
///                             .build_struct()).build_struct()).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Object Lambda Access Points using the `account_id` and `name`, separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:s3control/objectLambdaAccessPoint:ObjectLambdaAccessPoint example 123456789012:example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod object_lambda_access_point {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ObjectLambdaAccessPointArgs {
        /// The AWS account ID for the owner of the bucket for which you want to create an Object Lambda Access Point. Defaults to automatically determined account ID of the AWS provider.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A configuration block containing details about the Object Lambda Access Point. See Configuration below for more details.
        #[builder(into)]
        pub configuration: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::s3control::ObjectLambdaAccessPointConfiguration,
        >,
        /// The name for this Object Lambda Access Point.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ObjectLambdaAccessPointResult {
        /// The AWS account ID for the owner of the bucket for which you want to create an Object Lambda Access Point. Defaults to automatically determined account ID of the AWS provider.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Alias for the S3 Object Lambda Access Point.
        pub alias: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the Object Lambda Access Point.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A configuration block containing details about the Object Lambda Access Point. See Configuration below for more details.
        pub configuration: pulumi_gestalt_rust::Output<
            super::super::types::s3control::ObjectLambdaAccessPointConfiguration,
        >,
        /// The name for this Object Lambda Access Point.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ObjectLambdaAccessPointArgs,
    ) -> ObjectLambdaAccessPointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let configuration_binding = args.configuration.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3control/objectLambdaAccessPoint:ObjectLambdaAccessPoint"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ObjectLambdaAccessPointResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            alias: pulumi_gestalt_rust::__private::into_domain(o.extract_field("alias")),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configuration"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
