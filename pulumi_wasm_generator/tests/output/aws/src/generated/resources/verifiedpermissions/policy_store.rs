/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = policy_store::create(
///         "example",
///         PolicyStoreArgs::builder()
///             .validation_settings(
///                 PolicyStoreValidationSettings::builder().mode("STRICT").build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Verified Permissions Policy Store using the `policy_store_id`. For example:
///
/// console
///
///  % pulumi import aws_verifiedpermissions_policy_store.example DxQg2j8xvXJQ1tQCYNWj9T
///
pub mod policy_store {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyStoreArgs {
        /// A description of the Policy Store.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Validation settings for the policy store.
        #[builder(into, default)]
        pub validation_settings: pulumi_wasm_rust::Output<
            Option<
                super::super::types::verifiedpermissions::PolicyStoreValidationSettings,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct PolicyStoreResult {
        /// The ARN of the Policy Store.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A description of the Policy Store.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Policy Store.
        pub policy_store_id: pulumi_wasm_rust::Output<String>,
        /// Validation settings for the policy store.
        pub validation_settings: pulumi_wasm_rust::Output<
            Option<
                super::super::types::verifiedpermissions::PolicyStoreValidationSettings,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PolicyStoreArgs) -> PolicyStoreResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let validation_settings_binding = args.validation_settings.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:verifiedpermissions/policyStore:PolicyStore".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "validationSettings".into(),
                    value: &validation_settings_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "policyStoreId".into(),
                },
                register_interface::ResultField {
                    name: "validationSettings".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PolicyStoreResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            policy_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyStoreId").unwrap(),
            ),
            validation_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationSettings").unwrap(),
            ),
        }
    }
}