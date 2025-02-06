/// The [Turnstile Widget](https://developers.cloudflare.com/turnstile/) resource allows you to manage Cloudflare Turnstile Widgets.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = turnstile_widget::create(
///         "example",
///         TurnstileWidgetArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .bot_fight_mode(false)
///             .domains(vec!["example.com",])
///             .mode("invisible")
///             .name("example widget")
///             .region("world")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/turnstileWidget:TurnstileWidget example <account_id>/<site_key>
/// ```
///
pub mod turnstile_widget {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TurnstileWidgetArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If bot*fight*mode is set to true, Cloudflare issues computationally expensive challenges in response to malicious bots (Enterprise only).
        #[builder(into, default)]
        pub bot_fight_mode: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Domains where the widget is deployed
        #[builder(into)]
        pub domains: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Widget Mode. Available values: `non-interactive`, `invisible`, `managed`
        #[builder(into)]
        pub mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Human readable widget name.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Do not show any Cloudflare branding on the widget (Enterprise only).
        #[builder(into, default)]
        pub offlabel: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Region where this widget can be used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TurnstileWidgetResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// If bot*fight*mode is set to true, Cloudflare issues computationally expensive challenges in response to malicious bots (Enterprise only).
        pub bot_fight_mode: pulumi_gestalt_rust::Output<bool>,
        /// Domains where the widget is deployed
        pub domains: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Widget Mode. Available values: `non-interactive`, `invisible`, `managed`
        pub mode: pulumi_gestalt_rust::Output<String>,
        /// Human readable widget name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Do not show any Cloudflare branding on the widget (Enterprise only).
        pub offlabel: pulumi_gestalt_rust::Output<bool>,
        /// Region where this widget can be used.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// Secret key for this widget.
        pub secret: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TurnstileWidgetArgs,
    ) -> TurnstileWidgetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let bot_fight_mode_binding = args.bot_fight_mode.get_output(context).get_inner();
        let domains_binding = args.domains.get_output(context).get_inner();
        let mode_binding = args.mode.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let offlabel_binding = args.offlabel.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/turnstileWidget:TurnstileWidget".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "botFightMode".into(),
                    value: &bot_fight_mode_binding,
                },
                register_interface::ObjectField {
                    name: "domains".into(),
                    value: &domains_binding,
                },
                register_interface::ObjectField {
                    name: "mode".into(),
                    value: &mode_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "offlabel".into(),
                    value: &offlabel_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TurnstileWidgetResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            bot_fight_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("botFightMode"),
            ),
            domains: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domains"),
            ),
            mode: pulumi_gestalt_rust::__private::into_domain(o.extract_field("mode")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            offlabel: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("offlabel"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            secret: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secret"),
            ),
        }
    }
}
