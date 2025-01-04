/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = event_sources_config::create(
///         "example",
///         EventSourcesConfigArgs::builder()
///             .event_sources(
///                 vec![
///                     EventSourcesConfigEventSource::builder()
///                     .amazonCodeGuruProfilers(vec![EventSourcesConfigEventSourceAmazonCodeGuruProfiler::builder()
///                     .status("ENABLED").build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DevOps Guru Event Sources Config using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:devopsguru/eventSourcesConfig:EventSourcesConfig example us-east-1
/// ```
pub mod event_sources_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventSourcesConfigArgs {
        /// Configuration information about the integration of DevOps Guru as the Consumer via EventBridge with another AWS Service. See `event_sources` below.
        #[builder(into, default)]
        pub event_sources: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::devopsguru::EventSourcesConfigEventSource>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EventSourcesConfigResult {
        /// Configuration information about the integration of DevOps Guru as the Consumer via EventBridge with another AWS Service. See `event_sources` below.
        pub event_sources: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::devopsguru::EventSourcesConfigEventSource>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EventSourcesConfigArgs) -> EventSourcesConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let event_sources_binding = args.event_sources.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:devopsguru/eventSourcesConfig:EventSourcesConfig".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "eventSources".into(),
                    value: &event_sources_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "eventSources".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EventSourcesConfigResult {
            event_sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventSources").unwrap(),
            ),
        }
    }
}
