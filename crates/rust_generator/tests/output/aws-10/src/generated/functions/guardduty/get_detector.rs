#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_detector {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDetectorArgs {
        /// ID of the detector.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDetectorResult {
        /// Current configuration of the detector features.
        pub features: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::guardduty::GetDetectorFeature>,
        >,
        /// The frequency of notifications sent about subsequent finding occurrences.
        pub finding_publishing_frequency: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Service-linked role that grants GuardDuty access to the resources in the AWS account.
        pub service_role_arn: pulumi_gestalt_rust::Output<String>,
        /// Current status of the detector.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDetectorArgs,
    ) -> GetDetectorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let id_binding = args.id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:guardduty/getDetector:getDetector".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDetectorResult {
            features: o.get_field("features"),
            finding_publishing_frequency: o.get_field("findingPublishingFrequency"),
            id: o.get_field("id"),
            service_role_arn: o.get_field("serviceRoleArn"),
            status: o.get_field("status"),
        }
    }
}
