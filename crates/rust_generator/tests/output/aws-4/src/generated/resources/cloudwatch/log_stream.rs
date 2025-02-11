/// Provides a CloudWatch Log Stream resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = log_stream::create(
///         "foo",
///         LogStreamArgs::builder()
///             .log_group_name("${yada.name}")
///             .name("SampleLogStream1234")
///             .build_struct(),
///     );
///     let yada = log_group::create(
///         "yada",
///         LogGroupArgs::builder().name("Yada").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cloudwatch Log Stream using the stream's `log_group_name` and `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/logStream:LogStream foo Yada:SampleLogStream1234
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod log_stream {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogStreamArgs {
        /// The name of the log group under which the log stream is to be created.
        #[builder(into)]
        pub log_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the log stream. Must not be longer than 512 characters and must not contain `:`
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LogStreamResult {
        /// The Amazon Resource Name (ARN) specifying the log stream.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the log group under which the log stream is to be created.
        pub log_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the log stream. Must not be longer than 512 characters and must not contain `:`
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogStreamArgs,
    ) -> LogStreamResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let log_group_name_binding = args.log_group_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/logStream:LogStream".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logGroupName".into(),
                    value: &log_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LogStreamResult {
            arn: o.get_field("arn"),
            log_group_name: o.get_field("logGroupName"),
            name: o.get_field("name"),
        }
    }
}
