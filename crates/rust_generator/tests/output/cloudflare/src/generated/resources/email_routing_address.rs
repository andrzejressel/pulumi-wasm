/// The [Email Routing Address](https://developers.cloudflare.com/email-routing/setup/email-routing-addresses/#destination-addresses) resource allows you to manage Cloudflare Email Routing Destination Addresses.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod email_routing_address {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailRoutingAddressArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The contact email address of the user.
        #[builder(into)]
        pub email: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EmailRoutingAddressResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The date and time the destination address has been created.
        pub created: pulumi_gestalt_rust::Output<String>,
        /// The contact email address of the user.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// The date and time the destination address has been modified.
        pub modified: pulumi_gestalt_rust::Output<String>,
        /// Destination address identifier.
        pub tag: pulumi_gestalt_rust::Output<String>,
        /// The date and time the destination address has been verified. Null means not verified yet.
        pub verified: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EmailRoutingAddressArgs,
    ) -> EmailRoutingAddressResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let email_binding = args.email.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/emailRoutingAddress:EmailRoutingAddress".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "email".into(),
                    value: &email_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EmailRoutingAddressResult {
            account_id: o.get_field("accountId"),
            created: o.get_field("created"),
            email: o.get_field("email"),
            modified: o.get_field("modified"),
            tag: o.get_field("tag"),
            verified: o.get_field("verified"),
        }
    }
}
