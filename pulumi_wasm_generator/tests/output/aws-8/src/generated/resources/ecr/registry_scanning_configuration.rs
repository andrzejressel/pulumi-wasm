/// Provides an Elastic Container Registry Scanning Configuration. Can't be completely deleted, instead reverts to the default `BASIC` scanning configuration without rules.
///
/// ## Example Usage
///
/// ### Basic example
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let configuration = registry_scanning_configuration::create(
///         "configuration",
///         RegistryScanningConfigurationArgs::builder()
///             .rules(
///                 vec![
///                     RegistryScanningConfigurationRule::builder()
///                     .repositoryFilters(vec![RegistryScanningConfigurationRuleRepositoryFilter::builder()
///                     .filter("example").filterType("WILDCARD").build_struct(),])
///                     .scanFrequency("CONTINUOUS_SCAN").build_struct(),
///                 ],
///             )
///             .scan_type("ENHANCED")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Multiple rules
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = registry_scanning_configuration::create(
///         "test",
///         RegistryScanningConfigurationArgs::builder()
///             .rules(
///                 vec![
///                     RegistryScanningConfigurationRule::builder()
///                     .repositoryFilters(vec![RegistryScanningConfigurationRuleRepositoryFilter::builder()
///                     .filter("*").filterType("WILDCARD").build_struct(),])
///                     .scanFrequency("SCAN_ON_PUSH").build_struct(),
///                     RegistryScanningConfigurationRule::builder()
///                     .repositoryFilters(vec![RegistryScanningConfigurationRuleRepositoryFilter::builder()
///                     .filter("example").filterType("WILDCARD").build_struct(),])
///                     .scanFrequency("CONTINUOUS_SCAN").build_struct(),
///                 ],
///             )
///             .scan_type("ENHANCED")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ECR Scanning Configurations using the `registry_id`. For example:
///
/// ```sh
/// $ pulumi import aws:ecr/registryScanningConfiguration:RegistryScanningConfiguration example 123456789012
/// ```
pub mod registry_scanning_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegistryScanningConfigurationArgs {
        /// One or multiple blocks specifying scanning rules to determine which repository filters are used and at what frequency scanning will occur. See below for schema.
        #[builder(into, default)]
        pub rules: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::ecr::RegistryScanningConfigurationRule>>,
        >,
        /// the scanning type to set for the registry. Can be either `ENHANCED` or `BASIC`.
        #[builder(into)]
        pub scan_type: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RegistryScanningConfigurationResult {
        /// The registry ID the scanning configuration applies to.
        pub registry_id: pulumi_wasm_rust::Output<String>,
        /// One or multiple blocks specifying scanning rules to determine which repository filters are used and at what frequency scanning will occur. See below for schema.
        pub rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ecr::RegistryScanningConfigurationRule>>,
        >,
        /// the scanning type to set for the registry. Can be either `ENHANCED` or `BASIC`.
        pub scan_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RegistryScanningConfigurationArgs,
    ) -> RegistryScanningConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let rules_binding = args.rules.get_output(context).get_inner();
        let scan_type_binding = args.scan_type.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecr/registryScanningConfiguration:RegistryScanningConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
                register_interface::ObjectField {
                    name: "scanType".into(),
                    value: &scan_type_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RegistryScanningConfigurationResult {
            registry_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("registryId"),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(o.extract_field("rules")),
            scan_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scanType"),
            ),
        }
    }
}
