/// Resource for managing an Amazon Security Lake AWS Log Source.
///
/// > **NOTE:** A single `aws.securitylake.AwsLogSource` should be used to configure a log source across all regions and accounts.
///
/// > **NOTE:** The underlying `aws.securitylake.DataLake` must be configured before creating the `aws.securitylake.AwsLogSource`. Use a `depends_on` statement.
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
///     let example = aws_log_source::create(
///         "example",
///         AwsLogSourceArgs::builder()
///             .source(
///                 AwsLogSourceSource::builder()
///                     .accounts(vec!["123456789012",])
///                     .regions(vec!["eu-west-1",])
///                     .sourceName("ROUTE53")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS log sources using the source name. For example:
///
/// ```sh
/// $ pulumi import aws:securitylake/awsLogSource:AwsLogSource example ROUTE53
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod aws_log_source {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AwsLogSourceArgs {
        /// Specify the natively-supported AWS service to add as a source in Security Lake.
        #[builder(into, default)]
        pub source: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::securitylake::AwsLogSourceSource>,
        >,
    }
    #[allow(dead_code)]
    pub struct AwsLogSourceResult {
        /// Specify the natively-supported AWS service to add as a source in Security Lake.
        pub source: pulumi_gestalt_rust::Output<
            Option<super::super::types::securitylake::AwsLogSourceSource>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AwsLogSourceArgs,
    ) -> AwsLogSourceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let source_binding = args.source.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:securitylake/awsLogSource:AwsLogSource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: source_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AwsLogSourceResult {
            source: o.get_field("source"),
        }
    }
}
