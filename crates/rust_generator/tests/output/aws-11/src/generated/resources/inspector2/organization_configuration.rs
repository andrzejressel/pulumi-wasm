/// Resource for managing an Amazon Inspector Organization Configuration.
///
/// > **NOTE:** In order for this resource to work, the account you use must be an Inspector Delegated Admin Account.
///
/// > **NOTE:** When this resource is deleted, EC2, ECR, Lambda, and Lambda code scans will no longer be automatically enabled for new members of your Amazon Inspector organization.
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
///     let example = organization_configuration::create(
///         "example",
///         OrganizationConfigurationArgs::builder()
///             .auto_enable(
///                 OrganizationConfigurationAutoEnable::builder()
///                     .ec2(true)
///                     .ecr(false)
///                     .lambda(true)
///                     .lambdaCode(true)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
pub mod organization_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationConfigurationArgs {
        /// Configuration block for auto enabling. See below.
        #[builder(into)]
        pub auto_enable: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::inspector2::OrganizationConfigurationAutoEnable,
        >,
    }
    #[allow(dead_code)]
    pub struct OrganizationConfigurationResult {
        /// Configuration block for auto enabling. See below.
        pub auto_enable: pulumi_gestalt_rust::Output<
            super::super::types::inspector2::OrganizationConfigurationAutoEnable,
        >,
        /// Whether your configuration reached the max account limit.
        pub max_account_limit_reached: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: OrganizationConfigurationArgs,
    ) -> OrganizationConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let auto_enable_binding = args.auto_enable.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:inspector2/organizationConfiguration:OrganizationConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoEnable".into(),
                    value: &auto_enable_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        OrganizationConfigurationResult {
            auto_enable: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoEnable"),
            ),
            max_account_limit_reached: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxAccountLimitReached"),
            ),
        }
    }
}
