pub mod get_logs {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLogsArgs {
        #[builder(into, default)]
        pub details: pulumi_wasm_rust::Output<Option<bool>>,
        /// Discard headers that docker appends to each log entry
        #[builder(into, default)]
        pub discard_headers: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub follow: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true populate computed value `logs_list_string`
        #[builder(into, default)]
        pub logs_list_string_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Docker Container
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub show_stderr: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub show_stdout: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub since: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub tail: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub timestamps: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub until: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetLogsArgs) -> GetLogsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let details_binding = args.details.get_inner();
        let discard_headers_binding = args.discard_headers.get_inner();
        let follow_binding = args.follow.get_inner();
        let logs_list_string_enabled_binding = args.logs_list_string_enabled.get_inner();
        let name_binding = args.name.get_inner();
        let show_stderr_binding = args.show_stderr.get_inner();
        let show_stdout_binding = args.show_stdout.get_inner();
        let since_binding = args.since.get_inner();
        let tail_binding = args.tail.get_inner();
        let timestamps_binding = args.timestamps.get_inner();
        let until_binding = args.until.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "docker:index/getLogs:getLogs".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "details".into(),
                    value: &details_binding,
                },
                register_interface::ObjectField {
                    name: "discardHeaders".into(),
                    value: &discard_headers_binding,
                },
                register_interface::ObjectField {
                    name: "follow".into(),
                    value: &follow_binding,
                },
                register_interface::ObjectField {
                    name: "logsListStringEnabled".into(),
                    value: &logs_list_string_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "showStderr".into(),
                    value: &show_stderr_binding,
                },
                register_interface::ObjectField {
                    name: "showStdout".into(),
                    value: &show_stdout_binding,
                },
                register_interface::ObjectField {
                    name: "since".into(),
                    value: &since_binding,
                },
                register_interface::ObjectField {
                    name: "tail".into(),
                    value: &tail_binding,
                },
                register_interface::ObjectField {
                    name: "timestamps".into(),
                    value: &timestamps_binding,
                },
                register_interface::ObjectField {
                    name: "until".into(),
                    value: &until_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "details".into(),
                },
                register_interface::ResultField {
                    name: "discardHeaders".into(),
                },
                register_interface::ResultField {
                    name: "follow".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "logsListStringEnabled".into(),
                },
                register_interface::ResultField {
                    name: "logsListStrings".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "showStderr".into(),
                },
                register_interface::ResultField {
                    name: "showStdout".into(),
                },
                register_interface::ResultField {
                    name: "since".into(),
                },
                register_interface::ResultField {
                    name: "tail".into(),
                },
                register_interface::ResultField {
                    name: "timestamps".into(),
                },
                register_interface::ResultField {
                    name: "until".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLogsResult {
            details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("details").unwrap(),
            ),
            discard_headers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("discardHeaders").unwrap(),
            ),
            follow: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("follow").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            logs_list_string_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logsListStringEnabled").unwrap(),
            ),
            logs_list_strings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logsListStrings").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            show_stderr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("showStderr").unwrap(),
            ),
            show_stdout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("showStdout").unwrap(),
            ),
            since: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("since").unwrap(),
            ),
            tail: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tail").unwrap(),
            ),
            timestamps: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timestamps").unwrap(),
            ),
            until: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("until").unwrap(),
            ),
        }
    }
}
