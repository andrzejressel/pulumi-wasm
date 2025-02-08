#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_sink {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSinkArgs {
        /// The identifier for the resource.
        /// Examples:
        ///
        /// - `projects/[PROJECT_ID]/sinks/[SINK_NAME]`
        /// - `organizations/[ORGANIZATION_ID]/sinks/[SINK_NAME]`
        /// -  `billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_NAME]`
        /// - `folders/[FOLDER_ID]/sinks/[SINK_NAME]`
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSinkResult {
        /// Options that affect sinks exporting data to BigQuery. Structure is documented below.
        pub bigquery_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::logging::GetSinkBigqueryOption>,
        >,
        /// A description of this exclusion.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The destination of the sink (or, in other words, where logs are written to).
        pub destination: pulumi_gestalt_rust::Output<String>,
        /// Whether this exclusion is disabled and it does not exclude any log entries.
        pub disabled: pulumi_gestalt_rust::Output<bool>,
        /// Log entries that match any of the exclusion filters are not exported. Structure is documented below.
        pub exclusions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::logging::GetSinkExclusion>,
        >,
        /// An advanced logs filter that matches the log entries to be excluded.
        pub filter: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A client-assigned identifier, such as `load-balancer-exclusion`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The identity associated with this sink. This identity must be granted write access to the configured `destination`.
        pub writer_identity: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSinkArgs,
    ) -> GetSinkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:logging/getSink:getSink".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSinkResult {
            bigquery_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bigqueryOptions"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            destination: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destination"),
            ),
            disabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disabled"),
            ),
            exclusions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("exclusions"),
            ),
            filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filter"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            writer_identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("writerIdentity"),
            ),
        }
    }
}
