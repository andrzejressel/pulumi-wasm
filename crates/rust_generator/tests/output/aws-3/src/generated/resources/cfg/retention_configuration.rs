/// Provides a resource to manage the AWS Config retention configuration.
/// The retention configuration defines the number of days that AWS Config stores historical information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = retention_configuration::create(
///         "example",
///         RetentionConfigurationArgs::builder().retention_period_in_days(90).build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the AWS Config retention configuration using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cfg/retentionConfiguration:RetentionConfiguration example default
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod retention_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RetentionConfigurationArgs {
        /// The number of days AWS Config stores historical information.
        #[builder(into)]
        pub retention_period_in_days: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct RetentionConfigurationResult {
        /// The name of the retention configuration object. The object is always named **default**.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of days AWS Config stores historical information.
        pub retention_period_in_days: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RetentionConfigurationArgs,
    ) -> RetentionConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let retention_period_in_days_binding = args
            .retention_period_in_days
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cfg/retentionConfiguration:RetentionConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionPeriodInDays".into(),
                    value: retention_period_in_days_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RetentionConfigurationResult {
            name: o.get_field("name"),
            retention_period_in_days: o.get_field("retentionPeriodInDays"),
        }
    }
}
