/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy_store {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyStoreArgs {
        /// A description of the Policy Store.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Validation settings for the policy store.
        #[builder(into, default)]
        pub validation_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::verifiedpermissions::PolicyStoreValidationSettings,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct PolicyStoreResult {
        /// The ARN of the Policy Store.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A description of the Policy Store.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Policy Store.
        pub policy_store_id: pulumi_gestalt_rust::Output<String>,
        /// Validation settings for the policy store.
        pub validation_settings: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::verifiedpermissions::PolicyStoreValidationSettings,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyStoreArgs,
    ) -> PolicyStoreResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let validation_settings_binding = args.validation_settings.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:verifiedpermissions/policyStore:PolicyStore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validationSettings".into(),
                    value: validation_settings_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicyStoreResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            policy_store_id: o.get_field("policyStoreId"),
            validation_settings: o.get_field("validationSettings"),
        }
    }
}
