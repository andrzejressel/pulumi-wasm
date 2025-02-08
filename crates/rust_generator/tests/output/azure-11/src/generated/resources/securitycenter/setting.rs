/// Manages the Data Access Settings for Azure Security Center.
///
/// > **NOTE:** This resource requires the `Owner` permission on the Subscription.
///
/// > **NOTE:** Deletion of this resource disables the setting.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = setting::create(
///         "example",
///         SettingArgs::builder().enabled(true).setting_name("MCAS").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// The setting can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:securitycenter/setting:Setting example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Security/settings/<setting_name>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod setting {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SettingArgs {
        /// Boolean flag to enable/disable data access.
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The setting to manage. Possible values are `MCAS` , `WDATP`, `WDATP_EXCLUDE_LINUX_PUBLIC_PREVIEW`, `WDATP_UNIFIED_SOLUTION` and `Sentinel`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub setting_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SettingResult {
        /// Boolean flag to enable/disable data access.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The setting to manage. Possible values are `MCAS` , `WDATP`, `WDATP_EXCLUDE_LINUX_PUBLIC_PREVIEW`, `WDATP_UNIFIED_SOLUTION` and `Sentinel`. Changing this forces a new resource to be created.
        pub setting_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SettingArgs,
    ) -> SettingResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let setting_name_binding = args.setting_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:securitycenter/setting:Setting".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "settingName".into(),
                    value: &setting_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SettingResult {
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            setting_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("settingName"),
            ),
        }
    }
}
