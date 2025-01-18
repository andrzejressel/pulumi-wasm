pub mod get_sink {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
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
        pub id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetSinkResult {
        /// Options that affect sinks exporting data to BigQuery. Structure is documented below.
        pub bigquery_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::logging::GetSinkBigqueryOption>,
        >,
        /// A description of this exclusion.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The destination of the sink (or, in other words, where logs are written to).
        pub destination: pulumi_wasm_rust::Output<String>,
        /// Whether this exclusion is disabled and it does not exclude any log entries.
        pub disabled: pulumi_wasm_rust::Output<bool>,
        /// Log entries that match any of the exclusion filters are not exported. Structure is documented below.
        pub exclusions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::logging::GetSinkExclusion>,
        >,
        /// An advanced logs filter that matches the log entries to be excluded.
        pub filter: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        /// A client-assigned identifier, such as `load-balancer-exclusion`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The identity associated with this sink. This identity must be granted write access to the configured `destination`.
        pub writer_identity: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSinkArgs) -> GetSinkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:logging/getSink:getSink".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bigqueryOptions".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "destination".into(),
                },
                register_interface::ResultField {
                    name: "disabled".into(),
                },
                register_interface::ResultField {
                    name: "exclusions".into(),
                },
                register_interface::ResultField {
                    name: "filter".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "writerIdentity".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSinkResult {
            bigquery_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bigqueryOptions").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            destination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destination").unwrap(),
            ),
            disabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disabled").unwrap(),
            ),
            exclusions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exclusions").unwrap(),
            ),
            filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filter").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            writer_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("writerIdentity").unwrap(),
            ),
        }
    }
}
