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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetFindingIdsArgs,
    ) -> GetFindingIdsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let detector_id_binding = args.detector_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:guardduty/getFindingIds:getFindingIds".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "detectorId".into(),
                    value: &detector_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFindingIdsResult {
            detector_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("detectorId"),
            ),
            finding_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("findingIds"),
            ),
            has_findings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hasFindings"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
