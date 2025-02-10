#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_adbs_character_sets {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAdbsCharacterSetsArgs {
        /// The Azure Region to query for the character sets in.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAdbsCharacterSetsResult {
        /// A `character_sets` block as defined below.
        pub character_sets: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::oracle::GetAdbsCharacterSetsCharacterSet>,
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
        args: GetAdbsCharacterSetsArgs,
    ) -> GetAdbsCharacterSetsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:oracle/getAdbsCharacterSets:getAdbsCharacterSets".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAdbsCharacterSetsResult {
            character_sets: o.get_field("characterSets"),
            id: o.get_field("id"),
            location: o.get_field("location"),
        }
    }
}
