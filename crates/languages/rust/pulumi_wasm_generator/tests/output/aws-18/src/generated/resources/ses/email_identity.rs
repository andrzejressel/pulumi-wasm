/// Provides an SES email identity resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = email_identity::create(
///         "example",
///         EmailIdentityArgs::builder().email("email@example.com").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SES email identities using the email address. For example:
///
/// ```sh
/// $ pulumi import aws:ses/emailIdentity:EmailIdentity example email@example.com
/// ```
pub mod email_identity {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailIdentityArgs {
        /// The email address to assign to SES.
        #[builder(into)]
        pub email: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EmailIdentityResult {
        /// The ARN of the email identity.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The email address to assign to SES.
        pub email: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EmailIdentityArgs,
    ) -> EmailIdentityResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let email_binding = args.email.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ses/emailIdentity:EmailIdentity".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "email".into(),
                    value: &email_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EmailIdentityResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            email: pulumi_wasm_rust::__private::into_domain(o.extract_field("email")),
        }
    }
}
