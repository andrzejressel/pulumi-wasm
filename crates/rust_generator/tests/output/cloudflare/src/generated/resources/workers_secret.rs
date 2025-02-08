/// Provides a Cloudflare Worker secret resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let mySecret = workers_secret::create(
///         "mySecret",
///         WorkersSecretArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .name("MY_EXAMPLE_SECRET_TEXT")
///             .script_name("script_1")
///             .secret_text("my_secret_value")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/workersSecret:WorkersSecret example <account_id>/<script_name>/<secret_name>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workers_secret {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkersSecretArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Worker secret. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Worker script to associate the secret with. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub script_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The text of the Worker secret. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub secret_text: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkersSecretResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Worker secret. **Modifying this attribute will force creation of a new resource.**
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Worker script to associate the secret with. **Modifying this attribute will force creation of a new resource.**
        pub script_name: pulumi_gestalt_rust::Output<String>,
        /// The text of the Worker secret. **Modifying this attribute will force creation of a new resource.**
        pub secret_text: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WorkersSecretArgs,
    ) -> WorkersSecretResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let script_name_binding = args.script_name.get_output(context).get_inner();
        let secret_text_binding = args.secret_text.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/workersSecret:WorkersSecret".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "scriptName".into(),
                    value: &script_name_binding,
                },
                register_interface::ObjectField {
                    name: "secretText".into(),
                    value: &secret_text_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WorkersSecretResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            script_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scriptName"),
            ),
            secret_text: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secretText"),
            ),
        }
    }
}
