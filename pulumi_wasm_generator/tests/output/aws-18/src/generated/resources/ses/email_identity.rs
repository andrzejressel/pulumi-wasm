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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailIdentityArgs {
        /// The email address to assign to SES.
        #[builder(into)]
        pub email: pulumi_wasm_rust::Output<String>,
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
    pub fn create(name: &str, args: EmailIdentityArgs) -> EmailIdentityResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let email_binding = args.email.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ses/emailIdentity:EmailIdentity".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "email".into(),
                    value: &email_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "email".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EmailIdentityResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("email").unwrap(),
            ),
        }
    }
}
