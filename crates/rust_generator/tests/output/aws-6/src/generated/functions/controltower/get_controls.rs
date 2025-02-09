#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_controls {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetControlsArgs {
        /// The ARN of the organizational unit.
        #[builder(into)]
        pub target_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetControlsResult {
        /// List of all the ARNs for the controls applied to the `target_identifier`.
        pub enabled_controls: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub target_identifier: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetControlsArgs,
    ) -> GetControlsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let target_identifier_binding = args.target_identifier.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:controltower/getControls:getControls".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetIdentifier".into(),
                    value: target_identifier_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetControlsResult {
            enabled_controls: o.get_field("enabledControls"),
            id: o.get_field("id"),
            target_identifier: o.get_field("targetIdentifier"),
        }
    }
}
