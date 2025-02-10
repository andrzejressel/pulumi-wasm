/// Provides a Cloudflare Observatory Scheduled Test resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = observatory_scheduled_test::create(
///         "example",
///         ObservatoryScheduledTestArgs::builder()
///             .frequency("WEEKLY")
///             .region("us-central1")
///             .url("example.com")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/observatoryScheduledTest:ObservatoryScheduledTest example <zone_id>:<url>:<region>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod observatory_scheduled_test {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ObservatoryScheduledTestArgs {
        /// The frequency to run the test. Available values: `DAILY`, `WEEKLY`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub frequency: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The region to run the test in. Available values: `us-central1`, `us-east1`, `us-east4`, `us-south1`, `us-west1`, `southamerica-east1`, `europe-north1`, `europe-southwest1`, `europe-west1`, `europe-west2`, `europe-west3`, `europe-west4`, `europe-west8`, `europe-west9`, `asia-east1`, `asia-south1`, `asia-southeast1`, `me-west1`, `australia-southeast1`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub region: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The page to run the test on. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub url: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ObservatoryScheduledTestResult {
        /// The frequency to run the test. Available values: `DAILY`, `WEEKLY`. **Modifying this attribute will force creation of a new resource.**
        pub frequency: pulumi_gestalt_rust::Output<String>,
        /// The region to run the test in. Available values: `us-central1`, `us-east1`, `us-east4`, `us-south1`, `us-west1`, `southamerica-east1`, `europe-north1`, `europe-southwest1`, `europe-west1`, `europe-west2`, `europe-west3`, `europe-west4`, `europe-west8`, `europe-west9`, `asia-east1`, `asia-south1`, `asia-southeast1`, `me-west1`, `australia-southeast1`. **Modifying this attribute will force creation of a new resource.**
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The page to run the test on. **Modifying this attribute will force creation of a new resource.**
        pub url: pulumi_gestalt_rust::Output<String>,
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
        args: ObservatoryScheduledTestArgs,
    ) -> ObservatoryScheduledTestResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let frequency_binding = args.frequency.get_output(context);
        let region_binding = args.region.get_output(context);
        let url_binding = args.url.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/observatoryScheduledTest:ObservatoryScheduledTest"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frequency".into(),
                    value: frequency_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "url".into(),
                    value: url_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ObservatoryScheduledTestResult {
            frequency: o.get_field("frequency"),
            region: o.get_field("region"),
            url: o.get_field("url"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
