/// Provides a resource to manage AWS ECR Basic Scan Type
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = account_setting::create(
///         "foo",
///         AccountSettingArgs::builder()
///             .name("BASIC_SCAN_TYPE_VERSION")
///             .value("CLAIR")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EMR Security Configurations using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:ecr/accountSetting:AccountSetting foo BASIC_SCAN_TYPE_VERSION
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account_setting {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountSettingArgs {
        /// The name of the ECR Scan Type. This should be `BASIC_SCAN_TYPE_VERSION`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The value of the ECR Scan Type. This can be `AWS_NATIVE` or `CLAIR`.
        #[builder(into)]
        pub value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccountSettingResult {
        /// The name of the ECR Scan Type. This should be `BASIC_SCAN_TYPE_VERSION`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The value of the ECR Scan Type. This can be `AWS_NATIVE` or `CLAIR`.
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountSettingArgs,
    ) -> AccountSettingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let value_binding = args.value.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ecr/accountSetting:AccountSetting".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "value".into(),
                    value: value_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccountSettingResult {
            name: o.get_field("name"),
            value: o.get_field("value"),
        }
    }
}
