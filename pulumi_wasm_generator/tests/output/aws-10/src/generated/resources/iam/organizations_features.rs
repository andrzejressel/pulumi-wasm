/// Manages centralized root access features across AWS member accounts managed using AWS Organizations. More information about managing root access in IAM can be found in the [Centralize root access for member accounts](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_root-enable-root-access.html).
///
/// > **NOTE:** The AWS account utilizing this resource must be an Organizations management account. Also, you must enable trusted access for AWS Identity and Access Management in AWS Organizations.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = organization::create(
///         "example",
///         OrganizationArgs::builder()
///             .aws_service_access_principals(vec!["iam.amazonaws.com",])
///             .feature_set("ALL")
///             .build_struct(),
///     );
///     let exampleOrganizationsFeatures = organizations_features::create(
///         "exampleOrganizationsFeatures",
///         OrganizationsFeaturesArgs::builder()
///             .enabled_features(vec!["RootCredentialsManagement", "RootSessions",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import root access features using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/organizationsFeatures:OrganizationsFeatures example o-1234567
/// ```
pub mod organizations_features {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationsFeaturesArgs {
        /// List of IAM features to enable. Valid values are `RootCredentialsManagement` and `RootSessions`.
        #[builder(into)]
        pub enabled_features: pulumi_wasm_rust::Output<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationsFeaturesResult {
        /// List of IAM features to enable. Valid values are `RootCredentialsManagement` and `RootSessions`.
        pub enabled_features: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OrganizationsFeaturesArgs,
    ) -> OrganizationsFeaturesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enabled_features_binding = args.enabled_features.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/organizationsFeatures:OrganizationsFeatures".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabledFeatures".into(),
                    value: &enabled_features_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "enabledFeatures".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OrganizationsFeaturesResult {
            enabled_features: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabledFeatures").unwrap(),
            ),
        }
    }
}
