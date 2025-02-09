/// Resource for managing an AWS SESv2 (Simple Email V2) Account VDM Attributes.
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
///     let example = account_vdm_attributes::create(
///         "example",
///         AccountVdmAttributesArgs::builder()
///             .dashboard_attributes(
///                 AccountVdmAttributesDashboardAttributes::builder()
///                     .engagementMetrics("ENABLED")
///                     .build_struct(),
///             )
///             .guardian_attributes(
///                 AccountVdmAttributesGuardianAttributes::builder()
///                     .optimizedSharedDelivery("ENABLED")
///                     .build_struct(),
///             )
///             .vdm_enabled("ENABLED")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SESv2 (Simple Email V2) Account VDM Attributes using the word `ses-account-vdm-attributes`. For example:
///
/// ```sh
/// $ pulumi import aws:sesv2/accountVdmAttributes:AccountVdmAttributes example ses-account-vdm-attributes
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account_vdm_attributes {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountVdmAttributesArgs {
        /// Specifies additional settings for your VDM configuration as applicable to the Dashboard.
        #[builder(into, default)]
        pub dashboard_attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sesv2::AccountVdmAttributesDashboardAttributes>,
        >,
        /// Specifies additional settings for your VDM configuration as applicable to the Guardian.
        #[builder(into, default)]
        pub guardian_attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sesv2::AccountVdmAttributesGuardianAttributes>,
        >,
        /// Specifies the status of your VDM configuration. Valid values: `ENABLED`, `DISABLED`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub vdm_enabled: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccountVdmAttributesResult {
        /// Specifies additional settings for your VDM configuration as applicable to the Dashboard.
        pub dashboard_attributes: pulumi_gestalt_rust::Output<
            super::super::types::sesv2::AccountVdmAttributesDashboardAttributes,
        >,
        /// Specifies additional settings for your VDM configuration as applicable to the Guardian.
        pub guardian_attributes: pulumi_gestalt_rust::Output<
            super::super::types::sesv2::AccountVdmAttributesGuardianAttributes,
        >,
        /// Specifies the status of your VDM configuration. Valid values: `ENABLED`, `DISABLED`.
        ///
        /// The following arguments are optional:
        pub vdm_enabled: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountVdmAttributesArgs,
    ) -> AccountVdmAttributesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dashboard_attributes_binding = args.dashboard_attributes.get_output(context);
        let guardian_attributes_binding = args.guardian_attributes.get_output(context);
        let vdm_enabled_binding = args.vdm_enabled.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sesv2/accountVdmAttributes:AccountVdmAttributes".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dashboardAttributes".into(),
                    value: dashboard_attributes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "guardianAttributes".into(),
                    value: guardian_attributes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vdmEnabled".into(),
                    value: vdm_enabled_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccountVdmAttributesResult {
            dashboard_attributes: o.get_field("dashboardAttributes"),
            guardian_attributes: o.get_field("guardianAttributes"),
            vdm_enabled: o.get_field("vdmEnabled"),
        }
    }
}
