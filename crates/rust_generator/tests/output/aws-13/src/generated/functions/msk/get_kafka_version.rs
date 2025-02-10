#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_kafka_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKafkaVersionArgs {
        /// Ordered list of preferred Kafka versions. The first match in this list will be returned. Either `preferred_versions` or `version` must be set.
        #[builder(into, default)]
        pub preferred_versions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Version of MSK Kafka. For example 2.4.1.1 or "2.2.1" etc. Either `preferred_versions` or `version` must be set.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetKafkaVersionResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub preferred_versions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Status of the MSK Kafka version eg. `ACTIVE` or `DEPRECATED`.
        pub status: pulumi_gestalt_rust::Output<String>,
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetKafkaVersionArgs,
    ) -> GetKafkaVersionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let preferred_versions_binding = args.preferred_versions.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:msk/getKafkaVersion:getKafkaVersion".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredVersions".into(),
                    value: preferred_versions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetKafkaVersionResult {
            id: o.get_field("id"),
            preferred_versions: o.get_field("preferredVersions"),
            status: o.get_field("status"),
            version: o.get_field("version"),
        }
    }
}
