#[allow(clippy::doc_lazy_continuation)]
pub mod get_standards_control_associations {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetStandardsControlAssociationsArgs {
        /// The identifier of the control (identified with `SecurityControlId`, `SecurityControlArn`, or a mix of both parameters).
        #[builder(into)]
        pub security_control_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetStandardsControlAssociationsResult {
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ID of the security control.
        pub security_control_id: pulumi_gestalt_rust::Output<String>,
        /// A list that provides the status and other details for each security control that applies to each enabled standard.
        /// See `standards_control_associations` below.
        pub standards_control_associations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::securityhub::GetStandardsControlAssociationsStandardsControlAssociation,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetStandardsControlAssociationsArgs,
    ) -> GetStandardsControlAssociationsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let security_control_id_binding = args
            .security_control_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:securityhub/getStandardsControlAssociations:getStandardsControlAssociations"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "securityControlId".into(),
                    value: &security_control_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetStandardsControlAssociationsResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            security_control_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityControlId"),
            ),
            standards_control_associations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("standardsControlAssociations"),
            ),
        }
    }
}
