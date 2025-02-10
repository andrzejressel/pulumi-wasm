#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_finding_ids {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFindingIdsArgs {
        /// ID of the GuardDuty detector.
        #[builder(into)]
        pub detector_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFindingIdsResult {
        pub detector_id: pulumi_gestalt_rust::Output<String>,
        /// A list of finding IDs for the specified detector.
        pub finding_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Indicates whether findings are present for the specified detector.
        pub has_findings: pulumi_gestalt_rust::Output<bool>,
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetFindingIdsArgs,
    ) -> GetFindingIdsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let detector_id_binding = args.detector_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:guardduty/getFindingIds:getFindingIds".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "detectorId".into(),
                    value: detector_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetFindingIdsResult {
            detector_id: o.get_field("detectorId"),
            finding_ids: o.get_field("findingIds"),
            has_findings: o.get_field("hasFindings"),
            id: o.get_field("id"),
        }
    }
}
