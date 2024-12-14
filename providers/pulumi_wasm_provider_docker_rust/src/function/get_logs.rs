//! `docker.getLogs` provides logs from specific container

    #[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetLogsArgs {
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub details: pulumi_wasm_rust::Output<Option<bool>>,
    /// Discard headers that docker appends to each log entry
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub discard_headers: pulumi_wasm_rust::Output<Option<bool>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub follow: pulumi_wasm_rust::Output<Option<bool>>,
    /// If true populate computed value `logs_list_string`
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub logs_list_string_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the Docker Container
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub show_stderr: pulumi_wasm_rust::Output<Option<bool>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub show_stdout: pulumi_wasm_rust::Output<Option<bool>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub since: pulumi_wasm_rust::Output<Option<String>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub tail: pulumi_wasm_rust::Output<Option<String>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub timestamps: pulumi_wasm_rust::Output<Option<bool>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub until: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct GetLogsResult {
    pub details: pulumi_wasm_rust::Output<Option<bool>>,
    /// Discard headers that docker appends to each log entry
    pub discard_headers: pulumi_wasm_rust::Output<Option<bool>>,
    pub follow: pulumi_wasm_rust::Output<Option<bool>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// If true populate computed value `logs_list_string`
    pub logs_list_string_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// List of container logs, each element is a line.
    pub logs_list_strings: pulumi_wasm_rust::Output<Vec<String>>,
    /// The name of the Docker Container
    pub name: pulumi_wasm_rust::Output<String>,
    pub show_stderr: pulumi_wasm_rust::Output<Option<bool>>,
    pub show_stdout: pulumi_wasm_rust::Output<Option<bool>>,
    pub since: pulumi_wasm_rust::Output<Option<String>>,
    pub tail: pulumi_wasm_rust::Output<Option<String>>,
    pub timestamps: pulumi_wasm_rust::Output<Option<bool>>,
    pub until: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetLogsArgs
) -> GetLogsResult {

    let result = crate::bindings::pulumi::docker::get_logs::invoke(
        &crate::bindings::pulumi::docker::get_logs::Args {
                details: &args.details.get_inner(),
                discard_headers: &args.discard_headers.get_inner(),
                follow: &args.follow.get_inner(),
                logs_list_string_enabled: &args.logs_list_string_enabled.get_inner(),
                name: &args.name.get_inner(),
                show_stderr: &args.show_stderr.get_inner(),
                show_stdout: &args.show_stdout.get_inner(),
                since: &args.since.get_inner(),
                tail: &args.tail.get_inner(),
                timestamps: &args.timestamps.get_inner(),
                until: &args.until.get_inner(),
        }
    );

    GetLogsResult {
        details: crate::into_domain(result.details),
        discard_headers: crate::into_domain(result.discard_headers),
        follow: crate::into_domain(result.follow),
        id: crate::into_domain(result.id),
        logs_list_string_enabled: crate::into_domain(result.logs_list_string_enabled),
        logs_list_strings: crate::into_domain(result.logs_list_strings),
        name: crate::into_domain(result.name),
        show_stderr: crate::into_domain(result.show_stderr),
        show_stdout: crate::into_domain(result.show_stdout),
        since: crate::into_domain(result.since),
        tail: crate::into_domain(result.tail),
        timestamps: crate::into_domain(result.timestamps),
        until: crate::into_domain(result.until),
    }
}
