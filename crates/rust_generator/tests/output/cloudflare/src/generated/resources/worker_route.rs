/// Provides a Cloudflare worker route resource. A route will also require a `cloudflare.WorkerScript`.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myRoute = worker_route::create(
///         "myRoute",
///         WorkerRouteArgs::builder()
///             .pattern("example.com/*")
///             .script_name("${myScript.name}")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
///     let myScript = workers_script::create(
///         "myScript",
///         WorkersScriptArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/workerRoute:WorkerRoute example <zone_id>/<route_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod worker_route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkerRouteArgs {
        /// The [route pattern](https://developers.cloudflare.com/workers/about/routes/) to associate the Worker with.
        #[builder(into)]
        pub pattern: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Worker script name to invoke for requests that match the route pattern.
        #[builder(into, default)]
        pub script_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkerRouteResult {
        /// The [route pattern](https://developers.cloudflare.com/workers/about/routes/) to associate the Worker with.
        pub pattern: pulumi_gestalt_rust::Output<String>,
        /// Worker script name to invoke for requests that match the route pattern.
        pub script_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkerRouteArgs,
    ) -> WorkerRouteResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let pattern_binding = args.pattern.get_output(context);
        let script_name_binding = args.script_name.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/workerRoute:WorkerRoute".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pattern".into(),
                    value: pattern_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scriptName".into(),
                    value: script_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkerRouteResult {
            pattern: o.get_field("pattern"),
            script_name: o.get_field("scriptName"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
