#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_origin_access_control {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOriginAccessControlArgs {
        /// The identifier for the origin access control settings. For example: `E2T5VTFBZJ3BJB`.
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetOriginAccessControlResult {
        /// A description of the origin access control.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Current version of the origin access control's information. For example: `E2QWRUHAPOMQZL`.
        pub etag: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A name to identify the origin access control.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The type of origin that this origin access control is for.
        pub origin_access_control_origin_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies which requests CloudFront signs.
        pub signing_behavior: pulumi_gestalt_rust::Output<String>,
        /// The signing protocol of the origin access control, which determines how CloudFront signs (authenticates) requests.
        pub signing_protocol: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetOriginAccessControlArgs,
    ) -> GetOriginAccessControlResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let id_binding = args.id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:cloudfront/getOriginAccessControl:getOriginAccessControl".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOriginAccessControlResult {
            description: o.get_field("description"),
            etag: o.get_field("etag"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            origin_access_control_origin_type: o
                .get_field("originAccessControlOriginType"),
            signing_behavior: o.get_field("signingBehavior"),
            signing_protocol: o.get_field("signingProtocol"),
        }
    }
}
