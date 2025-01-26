/// The [Email Routing Address](https://developers.cloudflare.com/email-routing/setup/email-routing-addresses/#destination-addresses) resource allows you to manage Cloudflare Email Routing Destination Addresses.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = email_routing_address::create(
///         "example",
///         EmailRoutingAddressArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .email("user@example.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/emailRoutingAddress:EmailRoutingAddress example <account_id>/<email_routing_id>
/// ```
///
pub mod email_routing_address {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailRoutingAddressArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The contact email address of the user.
        #[builder(into)]
        pub email: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EmailRoutingAddressResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The date and time the destination address has been created.
        pub created: pulumi_wasm_rust::Output<String>,
        /// The contact email address of the user.
        pub email: pulumi_wasm_rust::Output<String>,
        /// The date and time the destination address has been modified.
        pub modified: pulumi_wasm_rust::Output<String>,
        /// Destination address identifier.
        pub tag: pulumi_wasm_rust::Output<String>,
        /// The date and time the destination address has been verified. Null means not verified yet.
        pub verified: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EmailRoutingAddressArgs,
    ) -> EmailRoutingAddressResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let email_binding = args.email.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/emailRoutingAddress:EmailRoutingAddress".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "email".into(),
                    value: &email_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EmailRoutingAddressResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            created: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("created"),
            ),
            email: pulumi_wasm_rust::__private::into_domain(o.extract_field("email")),
            modified: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("modified"),
            ),
            tag: pulumi_wasm_rust::__private::into_domain(o.extract_field("tag")),
            verified: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("verified"),
            ),
        }
    }
}
