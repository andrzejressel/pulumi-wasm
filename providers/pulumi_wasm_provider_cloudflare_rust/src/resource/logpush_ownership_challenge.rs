pub struct LogpushOwnershipChallengeArgs {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub destination_conf: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct LogpushOwnershipChallengeResult {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub destination_conf: pulumi_wasm_rust::Output<String>,
    pub ownership_challenge_filename: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub fn create(name: &str, args: LogpushOwnershipChallengeArgs) -> LogpushOwnershipChallengeResult {
    let result = crate::bindings::pulumi::cloudflare::logpush_ownership_challenge::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::logpush_ownership_challenge::Args {
            account_id: args.account_id.get_inner(),
            destination_conf: args.destination_conf.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    LogpushOwnershipChallengeResult {
        account_id: crate::into_domain(result.account_id),
        destination_conf: crate::into_domain(result.destination_conf),
        ownership_challenge_filename: crate::into_domain(result.ownership_challenge_filename),
        zone_id: crate::into_domain(result.zone_id),
    }
}
