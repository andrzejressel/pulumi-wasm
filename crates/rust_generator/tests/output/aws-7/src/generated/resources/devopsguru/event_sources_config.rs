/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_sources_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventSourcesConfigArgs {
        /// Configuration information about the integration of DevOps Guru as the Consumer via EventBridge with another AWS Service. See `event_sources` below.
        #[builder(into, default)]
        pub event_sources: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::devopsguru::EventSourcesConfigEventSource>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EventSourcesConfigResult {
        /// Configuration information about the integration of DevOps Guru as the Consumer via EventBridge with another AWS Service. See `event_sources` below.
        pub event_sources: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::devopsguru::EventSourcesConfigEventSource>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventSourcesConfigArgs,
    ) -> EventSourcesConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let event_sources_binding = args.event_sources.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:devopsguru/eventSourcesConfig:EventSourcesConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventSources".into(),
                    value: &event_sources_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventSourcesConfigResult {
            event_sources: o.get_field("eventSources"),
        }
    }
}
