/// Enables or disables the Security Center Auto Provisioning feature for the subscription
///
/// > **Note:** The `azure.securitycenter.AutoProvisioning` resource has been deprecated because [the auto provisioning capability will be deprecated by end of Novemember of 2024](https://learn.microsoft.com/en-us/azure/defender-for-cloud/prepare-deprecation-log-analytics-mma-agent#log-analytics-agent-autoprovisioning-experience---deprecation-plan) and will be removed in v5.0 of the AzureRM Provider.
///
/// > **NOTE:** There is no resource name required, it will always be "default"
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = auto_provisioning::create(
///         "example",
///         AutoProvisioningArgs::builder().auto_provision("On").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Security Center Auto Provisioning can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:securitycenter/autoProvisioning:AutoProvisioning example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Security/autoProvisioningSettings/default
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod auto_provisioning {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutoProvisioningArgs {
        /// Should the security agent be automatically provisioned on Virtual Machines in this subscription? Possible values are `On` (to install the security agent automatically, if it's missing) or `Off` (to not install the security agent automatically).
        #[builder(into)]
        pub auto_provision: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AutoProvisioningResult {
        /// Should the security agent be automatically provisioned on Virtual Machines in this subscription? Possible values are `On` (to install the security agent automatically, if it's missing) or `Off` (to not install the security agent automatically).
        pub auto_provision: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AutoProvisioningArgs,
    ) -> AutoProvisioningResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_provision_binding = args.auto_provision.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:securitycenter/autoProvisioning:AutoProvisioning".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoProvision".into(),
                    value: auto_provision_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AutoProvisioningResult {
            auto_provision: o.get_field("autoProvision"),
        }
    }
}
