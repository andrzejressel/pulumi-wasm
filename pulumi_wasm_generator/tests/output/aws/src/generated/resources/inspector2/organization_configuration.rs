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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationConfigurationArgs {
        /// Configuration block for auto enabling. See below.
        #[builder(into)]
        pub auto_enable: pulumi_wasm_rust::Output<
            super::super::types::inspector2::OrganizationConfigurationAutoEnable,
        >,
    }
    #[allow(dead_code)]
    pub struct OrganizationConfigurationResult {
        /// Configuration block for auto enabling. See below.
        pub auto_enable: pulumi_wasm_rust::Output<
            super::super::types::inspector2::OrganizationConfigurationAutoEnable,
        >,
        /// Whether your configuration reached the max account limit.
        pub max_account_limit_reached: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OrganizationConfigurationArgs,
    ) -> OrganizationConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_enable_binding = args.auto_enable.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:inspector2/organizationConfiguration:OrganizationConfiguration"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoEnable".into(),
                    value: &auto_enable_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoEnable".into(),
                },
                register_interface::ResultField {
                    name: "maxAccountLimitReached".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OrganizationConfigurationResult {
            auto_enable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoEnable").unwrap(),
            ),
            max_account_limit_reached: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxAccountLimitReached").unwrap(),
            ),
        }
    }
}