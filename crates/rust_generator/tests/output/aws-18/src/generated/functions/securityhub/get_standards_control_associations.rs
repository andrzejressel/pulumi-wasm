#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetStandardsControlAssociationsArgs,
    ) -> GetStandardsControlAssociationsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let security_control_id_binding = args.security_control_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:securityhub/getStandardsControlAssociations:getStandardsControlAssociations"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityControlId".into(),
                    value: security_control_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetStandardsControlAssociationsResult {
            id: o.get_field("id"),
            security_control_id: o.get_field("securityControlId"),
            standards_control_associations: o.get_field("standardsControlAssociations"),
        }
    }
}
