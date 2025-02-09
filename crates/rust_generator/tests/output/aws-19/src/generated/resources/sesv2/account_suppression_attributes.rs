/// Manages AWS SESv2 (Simple Email V2) account-level suppression attributes.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account_suppression_attributes::create(
///         "example",
///         AccountSuppressionAttributesArgs::builder()
///             .suppressed_reasons(vec!["COMPLAINT",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import account-level suppression attributes using the account ID. For example:
///
/// ```sh
/// $ pulumi import aws:sesv2/accountSuppressionAttributes:AccountSuppressionAttributes example 123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account_suppression_attributes {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountSuppressionAttributesArgs {
        /// A list that contains the reasons that email addresses will be automatically added to the suppression list for your account. Valid values: `COMPLAINT`, `BOUNCE`.
        #[builder(into)]
        pub suppressed_reasons: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct AccountSuppressionAttributesResult {
        /// A list that contains the reasons that email addresses will be automatically added to the suppression list for your account. Valid values: `COMPLAINT`, `BOUNCE`.
        pub suppressed_reasons: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountSuppressionAttributesArgs,
    ) -> AccountSuppressionAttributesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let suppressed_reasons_binding = args.suppressed_reasons.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sesv2/accountSuppressionAttributes:AccountSuppressionAttributes"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "suppressedReasons".into(),
                    value: suppressed_reasons_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccountSuppressionAttributesResult {
            suppressed_reasons: o.get_field("suppressedReasons"),
        }
    }
}
