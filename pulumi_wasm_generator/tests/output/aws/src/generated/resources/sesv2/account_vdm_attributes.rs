/// Resource for managing an AWS SESv2 (Simple Email V2) Account VDM Attributes.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod account_vdm_attributes {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountVdmAttributesArgs {
        /// Specifies additional settings for your VDM configuration as applicable to the Dashboard.
        #[builder(into, default)]
        pub dashboard_attributes: pulumi_wasm_rust::Output<
            Option<super::super::types::sesv2::AccountVdmAttributesDashboardAttributes>,
        >,
        /// Specifies additional settings for your VDM configuration as applicable to the Guardian.
        #[builder(into, default)]
        pub guardian_attributes: pulumi_wasm_rust::Output<
            Option<super::super::types::sesv2::AccountVdmAttributesGuardianAttributes>,
        >,
        /// Specifies the status of your VDM configuration. Valid values: `ENABLED`, `DISABLED`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub vdm_enabled: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AccountVdmAttributesResult {
        /// Specifies additional settings for your VDM configuration as applicable to the Dashboard.
        pub dashboard_attributes: pulumi_wasm_rust::Output<
            super::super::types::sesv2::AccountVdmAttributesDashboardAttributes,
        >,
        /// Specifies additional settings for your VDM configuration as applicable to the Guardian.
        pub guardian_attributes: pulumi_wasm_rust::Output<
            super::super::types::sesv2::AccountVdmAttributesGuardianAttributes,
        >,
        /// Specifies the status of your VDM configuration. Valid values: `ENABLED`, `DISABLED`.
        ///
        /// The following arguments are optional:
        pub vdm_enabled: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AccountVdmAttributesArgs,
    ) -> AccountVdmAttributesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dashboard_attributes_binding = args.dashboard_attributes.get_inner();
        let guardian_attributes_binding = args.guardian_attributes.get_inner();
        let vdm_enabled_binding = args.vdm_enabled.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sesv2/accountVdmAttributes:AccountVdmAttributes".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dashboardAttributes".into(),
                    value: &dashboard_attributes_binding,
                },
                register_interface::ObjectField {
                    name: "guardianAttributes".into(),
                    value: &guardian_attributes_binding,
                },
                register_interface::ObjectField {
                    name: "vdmEnabled".into(),
                    value: &vdm_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dashboardAttributes".into(),
                },
                register_interface::ResultField {
                    name: "guardianAttributes".into(),
                },
                register_interface::ResultField {
                    name: "vdmEnabled".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccountVdmAttributesResult {
            dashboard_attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dashboardAttributes").unwrap(),
            ),
            guardian_attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("guardianAttributes").unwrap(),
            ),
            vdm_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vdmEnabled").unwrap(),
            ),
        }
    }
}
