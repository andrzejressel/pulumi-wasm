/// Enable (Opt-In) or Disable (Opt-Out) a particular Region for an AWS account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = region::create(
///         "example",
///         RegionArgs::builder().enabled(true).region_name("ap-southeast-3").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`. For example:
///
/// ```sh
/// $ pulumi import aws:account/region:Region example ap-southeast-3
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod region {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionArgs {
        /// The ID of the target account when managing member accounts. Will manage current user's account by default if omitted. To use this parameter, the caller must be an identity in the organization's management account or a delegated administrator account. The specified account ID must also be a member account in the same organization. The organization must have all features enabled, and the organization must have trusted access enabled for the Account Management service, and optionally a delegated admin account assigned.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the region is enabled.
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The region name to manage.
        #[builder(into)]
        pub region_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RegionResult {
        /// The ID of the target account when managing member accounts. Will manage current user's account by default if omitted. To use this parameter, the caller must be an identity in the organization's management account or a delegated administrator account. The specified account ID must also be a member account in the same organization. The organization must have all features enabled, and the organization must have trusted access enabled for the Account Management service, and optionally a delegated admin account assigned.
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the region is enabled.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The region opt status.
        pub opt_status: pulumi_gestalt_rust::Output<String>,
        /// The region name to manage.
        pub region_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegionArgs,
    ) -> RegionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let region_name_binding = args.region_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:account/region:Region".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "regionName".into(),
                    value: region_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegionResult {
            account_id: o.get_field("accountId"),
            enabled: o.get_field("enabled"),
            opt_status: o.get_field("optStatus"),
            region_name: o.get_field("regionName"),
        }
    }
}
