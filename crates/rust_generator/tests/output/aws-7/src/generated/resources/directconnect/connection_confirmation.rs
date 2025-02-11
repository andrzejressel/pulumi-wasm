/// Provides a confirmation of the creation of the specified hosted connection on an interconnect.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let confirmation = connection_confirmation::create(
///         "confirmation",
///         ConnectionConfirmationArgs::builder()
///             .connection_id("dxcon-ffabc123")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connection_confirmation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionConfirmationArgs {
        /// The ID of the hosted connection.
        #[builder(into)]
        pub connection_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ConnectionConfirmationResult {
        /// The ID of the hosted connection.
        pub connection_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConnectionConfirmationArgs,
    ) -> ConnectionConfirmationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let connection_id_binding = args.connection_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:directconnect/connectionConfirmation:ConnectionConfirmation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionId".into(),
                    value: &connection_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConnectionConfirmationResult {
            connection_id: o.get_field("connectionId"),
        }
    }
}
