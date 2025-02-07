/// Provides a Cloudflare Account resource. Account is the basic resource for
/// working with Cloudflare zones, teams and users.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create(
///         "example",
///         AccountArgs::builder()
///             .enforce_twofactor(true)
///             .name("some-enterprise-account")
///             .type_("enterprise")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/account:Account example <account_id>
/// ```
///
pub mod account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        /// Whether 2FA is enforced on the account. Defaults to `false`.
        #[builder(into, default)]
        pub enforce_twofactor: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the account that is displayed in the Cloudflare dashboard.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Account type. Available values: `enterprise`, `standard`. Defaults to `standard`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        /// Whether 2FA is enforced on the account. Defaults to `false`.
        pub enforce_twofactor: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the account that is displayed in the Cloudflare dashboard.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Account type. Available values: `enterprise`, `standard`. Defaults to `standard`. **Modifying this attribute will force creation of a new resource.**
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AccountArgs,
    ) -> AccountResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let enforce_twofactor_binding = args
            .enforce_twofactor
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/account:Account".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enforceTwofactor".into(),
                    value: &enforce_twofactor_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccountResult {
            enforce_twofactor: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enforceTwofactor"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
