pub struct TurnstileWidgetArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub bot_fight_mode: pulumi_wasm_rust::Output<Option<bool>>,
    pub domains: pulumi_wasm_rust::Output<Vec<String>>,
    pub mode: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub offlabel: pulumi_wasm_rust::Output<Option<bool>>,
    pub region: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct TurnstileWidgetResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub bot_fight_mode: pulumi_wasm_rust::Output<bool>,
    pub domains: pulumi_wasm_rust::Output<Vec<String>>,
    pub mode: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub offlabel: pulumi_wasm_rust::Output<bool>,
    pub region: pulumi_wasm_rust::Output<String>,
    pub secret: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: TurnstileWidgetArgs) -> TurnstileWidgetResult {
    let result = crate::bindings::pulumi::cloudflare::turnstile_widget::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::turnstile_widget::Args {
            account_id: args.account_id.get_inner(),
            bot_fight_mode: args.bot_fight_mode.get_inner(),
            domains: args.domains.get_inner(),
            mode: args.mode.get_inner(),
            name: args.name.get_inner(),
            offlabel: args.offlabel.get_inner(),
            region: args.region.get_inner(),
        },
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
