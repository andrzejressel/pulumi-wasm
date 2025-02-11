#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_db_system_shapes {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDbSystemShapesArgs {
        /// The Azure Region to query for the system shapes in.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDbSystemShapesResult {
        /// A `db_system_shapes` block as defined below.
        pub db_system_shapes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::oracle::GetDbSystemShapesDbSystemShape>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDbSystemShapesArgs,
    ) -> GetDbSystemShapesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:oracle/getDbSystemShapes:getDbSystemShapes".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDbSystemShapesResult {
            db_system_shapes: o.get_field("dbSystemShapes"),
            id: o.get_field("id"),
            location: o.get_field("location"),
        }
    }
}
