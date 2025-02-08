#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_logs {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLogsArgs {
        #[builder(into, default)]
        pub details: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Discard headers that docker appends to each log entry
        #[builder(into, default)]
        pub discard_headers: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub follow: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If true populate computed value `logs_list_string`
        #[builder(into, default)]
        pub logs_list_string_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the Docker Container
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub show_stderr: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub show_stdout: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub since: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub tail: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub timestamps: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub until: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetLogsResult {
        pub details: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Discard headers that docker appends to each log entry
        pub discard_headers: pulumi_gestalt_rust::Output<Option<bool>>,
        pub follow: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// If true populate computed value `logs_list_string`
        pub logs_list_string_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// List of container logs, each element is a line.
        pub logs_list_strings: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of the Docker Container
        pub name: pulumi_gestalt_rust::Output<String>,
        pub show_stderr: pulumi_gestalt_rust::Output<Option<bool>>,
        pub show_stdout: pulumi_gestalt_rust::Output<Option<bool>>,
        pub since: pulumi_gestalt_rust::Output<Option<String>>,
        pub tail: pulumi_gestalt_rust::Output<Option<String>>,
        pub timestamps: pulumi_gestalt_rust::Output<Option<bool>>,
        pub until: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetLogsArgs,
    ) -> GetLogsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let details_binding = args.details.get_output(context).get_inner();
        let discard_headers_binding = args
            .discard_headers
            .get_output(context)
            .get_inner();
        let follow_binding = args.follow.get_output(context).get_inner();
        let logs_list_string_enabled_binding = args
            .logs_list_string_enabled
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let show_stderr_binding = args.show_stderr.get_output(context).get_inner();
        let show_stdout_binding = args.show_stdout.get_output(context).get_inner();
        let since_binding = args.since.get_output(context).get_inner();
        let tail_binding = args.tail.get_output(context).get_inner();
        let timestamps_binding = args.timestamps.get_output(context).get_inner();
        let until_binding = args.until.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "docker:index/getLogs:getLogs".into(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLogsResult {
            details: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("details"),
            ),
            discard_headers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("discardHeaders"),
            ),
            follow: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("follow"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            logs_list_string_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logsListStringEnabled"),
            ),
            logs_list_strings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logsListStrings"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            show_stderr: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("showStderr"),
            ),
            show_stdout: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("showStdout"),
            ),
            since: pulumi_gestalt_rust::__private::into_domain(o.extract_field("since")),
            tail: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tail")),
            timestamps: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timestamps"),
            ),
            until: pulumi_gestalt_rust::__private::into_domain(o.extract_field("until")),
        }
    }
}
