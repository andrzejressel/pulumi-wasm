/// Resource for managing an AWS DataZone Environment Blueprint Configuration.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:datazone:Domain
///     properties:
///       name: example_domain
///       domainExecutionRole: ${domainExecutionRole.arn}
///   exampleEnvironmentBlueprintConfiguration:
///     type: aws:datazone:EnvironmentBlueprintConfiguration
///     name: example
///     properties:
///       domainId: ${example.id}
///       environmentBlueprintId: ${defaultDataLake.id}
///       enabledRegions:
///         - us-east-1
///       regionalParameters:
///         us-east-1:
///           s3Location: s3://my-amazon-datazone-bucket
/// variables:
///   defaultDataLake:
///     fn::invoke:
///       function: aws:datazone:getEnvironmentBlueprint
///       arguments:
///         domainId: ${example.id}
///         name: DefaultDataLake
///         managed: true
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DataZone Environment Blueprint Configuration using the `domain_id` and `environment_blueprint_id`, separated by a `/`. For example:
///
/// ```sh
/// $ pulumi import aws:datazone/environmentBlueprintConfiguration:EnvironmentBlueprintConfiguration example domain-id-12345/environment-blueprint-id-54321
/// ```
pub mod environment_blueprint_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentBlueprintConfigurationArgs {
        /// ID of the Domain.
        #[builder(into)]
        pub domain_id: pulumi_wasm_rust::Output<String>,
        /// Regions in which the blueprint is enabled
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub enabled_regions: pulumi_wasm_rust::Output<Vec<String>>,
        /// ID of the Environment Blueprint
        #[builder(into)]
        pub environment_blueprint_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the manage access role with which this blueprint is created.
        #[builder(into, default)]
        pub manage_access_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the provisioning role with which this blueprint is created.
        #[builder(into, default)]
        pub provisioning_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Parameters for each region in which the blueprint is enabled
        #[builder(into, default)]
        pub regional_parameters: pulumi_wasm_rust::Output<
            Option<
                std::collections::HashMap<
                    String,
                    std::collections::HashMap<String, String>,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct EnvironmentBlueprintConfigurationResult {
        /// ID of the Domain.
        pub domain_id: pulumi_wasm_rust::Output<String>,
        /// Regions in which the blueprint is enabled
        ///
        /// The following arguments are optional:
        pub enabled_regions: pulumi_wasm_rust::Output<Vec<String>>,
        /// ID of the Environment Blueprint
        pub environment_blueprint_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the manage access role with which this blueprint is created.
        pub manage_access_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the provisioning role with which this blueprint is created.
        pub provisioning_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Parameters for each region in which the blueprint is enabled
        pub regional_parameters: pulumi_wasm_rust::Output<
            Option<
                std::collections::HashMap<
                    String,
                    std::collections::HashMap<String, String>,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: EnvironmentBlueprintConfigurationArgs,
    ) -> EnvironmentBlueprintConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_id_binding = args.domain_id.get_inner();
        let enabled_regions_binding = args.enabled_regions.get_inner();
        let environment_blueprint_id_binding = args.environment_blueprint_id.get_inner();
        let manage_access_role_arn_binding = args.manage_access_role_arn.get_inner();
        let provisioning_role_arn_binding = args.provisioning_role_arn.get_inner();
        let regional_parameters_binding = args.regional_parameters.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datazone/environmentBlueprintConfiguration:EnvironmentBlueprintConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainId".into(),
                    value: &domain_id_binding,
                },
                register_interface::ObjectField {
                    name: "enabledRegions".into(),
                    value: &enabled_regions_binding,
                },
                register_interface::ObjectField {
                    name: "environmentBlueprintId".into(),
                    value: &environment_blueprint_id_binding,
                },
                register_interface::ObjectField {
                    name: "manageAccessRoleArn".into(),
                    value: &manage_access_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "provisioningRoleArn".into(),
                    value: &provisioning_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "regionalParameters".into(),
                    value: &regional_parameters_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "domainId".into(),
                },
                register_interface::ResultField {
                    name: "enabledRegions".into(),
                },
                register_interface::ResultField {
                    name: "environmentBlueprintId".into(),
                },
                register_interface::ResultField {
                    name: "manageAccessRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "provisioningRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "regionalParameters".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EnvironmentBlueprintConfigurationResult {
            domain_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainId").unwrap(),
            ),
            enabled_regions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabledRegions").unwrap(),
            ),
            environment_blueprint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentBlueprintId").unwrap(),
            ),
            manage_access_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("manageAccessRoleArn").unwrap(),
            ),
            provisioning_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("provisioningRoleArn").unwrap(),
            ),
            regional_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("regionalParameters").unwrap(),
            ),
        }
    }
}
