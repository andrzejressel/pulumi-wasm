//! The [Turnstile Widget](https://developers.cloudflare.com/turnstile/) resource allows you to manage Cloudflare Turnstile Widgets.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = turnstile_widget::create(
//!         "example",
//!         TurnstileWidgetArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .bot_fight_mode(false)
//!             .domains(vec!["example.com",])
//!             .mode("invisible")
//!             .name("example widget")
//!             .region("world")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/turnstileWidget:TurnstileWidget example <account_id>/<site_key>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TurnstileWidgetArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// If bot*fight*mode is set to true, Cloudflare issues computationally expensive challenges in response to malicious bots (Enterprise only).
    #[builder(into, default)]
    pub bot_fight_mode: pulumi_wasm_rust::Output<Option<bool>>,
    /// Domains where the widget is deployed
    #[builder(into)]
    pub domains: pulumi_wasm_rust::Output<Vec<String>>,
    /// Widget Mode. Available values: `non-interactive`, `invisible`, `managed`
    #[builder(into)]
    pub mode: pulumi_wasm_rust::Output<String>,
    /// Human readable widget name.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Do not show any Cloudflare branding on the widget (Enterprise only).
    #[builder(into, default)]
    pub offlabel: pulumi_wasm_rust::Output<Option<bool>>,
    /// Region where this widget can be used.
    #[builder(into, default)]
    pub region: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct TurnstileWidgetResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// If bot*fight*mode is set to true, Cloudflare issues computationally expensive challenges in response to malicious bots (Enterprise only).
    pub bot_fight_mode: pulumi_wasm_rust::Output<bool>,
    /// Domains where the widget is deployed
    pub domains: pulumi_wasm_rust::Output<Vec<String>>,
    /// Widget Mode. Available values: `non-interactive`, `invisible`, `managed`
    pub mode: pulumi_wasm_rust::Output<String>,
    /// Human readable widget name.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Do not show any Cloudflare branding on the widget (Enterprise only).
    pub offlabel: pulumi_wasm_rust::Output<bool>,
    /// Region where this widget can be used.
    pub region: pulumi_wasm_rust::Output<String>,
    /// Secret key for this widget.
    pub secret: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: TurnstileWidgetArgs
) -> TurnstileWidgetResult {

    let result = crate::bindings::pulumi::cloudflare::turnstile_widget::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::turnstile_widget::Args {
                account_id: &args.account_id.get_inner(),
                bot_fight_mode: &args.bot_fight_mode.get_inner(),
                domains: &args.domains.get_inner(),
                mode: &args.mode.get_inner(),
                name: &args.name.get_inner(),
                offlabel: &args.offlabel.get_inner(),
                region: &args.region.get_inner(),
        }
    );

    TurnstileWidgetResult {
        account_id: crate::into_domain(result.account_id),
        bot_fight_mode: crate::into_domain(result.bot_fight_mode),
        domains: crate::into_domain(result.domains),
        mode: crate::into_domain(result.mode),
        name: crate::into_domain(result.name),
        offlabel: crate::into_domain(result.offlabel),
        region: crate::into_domain(result.region),
        secret: crate::into_domain(result.secret),
    }
}
