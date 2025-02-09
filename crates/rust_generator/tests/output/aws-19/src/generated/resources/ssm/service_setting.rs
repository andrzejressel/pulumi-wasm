/// This setting defines how a user interacts with or uses a service or a feature of a service.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testSetting = service_setting::create(
///         "testSetting",
///         ServiceSettingArgs::builder()
///             .setting_id(
///                 "arn:aws:ssm:us-east-1:123456789012:servicesetting/ssm/parameter-store/high-throughput-enabled",
///             )
///             .setting_value("true")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS SSM Service Setting using the `setting_id`. For example:
///
/// ```sh
/// $ pulumi import aws:ssm/serviceSetting:ServiceSetting example arn:aws:ssm:us-east-1:123456789012:servicesetting/ssm/parameter-store/high-throughput-enabled
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_setting {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceSettingArgs {
        /// ID of the service setting.
        #[builder(into)]
        pub setting_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Value of the service setting.
        #[builder(into)]
        pub setting_value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceSettingResult {
        /// ARN of the service setting.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ID of the service setting.
        pub setting_id: pulumi_gestalt_rust::Output<String>,
        /// Value of the service setting.
        pub setting_value: pulumi_gestalt_rust::Output<String>,
        /// Status of the service setting. Value can be `Default`, `Customized` or `PendingUpdate`.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceSettingArgs,
    ) -> ServiceSettingResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let setting_id_binding = args.setting_id.get_output(context);
        let setting_value_binding = args.setting_value.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssm/serviceSetting:ServiceSetting".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "settingId".into(),
                    value: setting_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "settingValue".into(),
                    value: setting_value_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceSettingResult {
            arn: o.get_field("arn"),
            setting_id: o.get_field("settingId"),
            setting_value: o.get_field("settingValue"),
            status: o.get_field("status"),
        }
    }
}
