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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAdbsCharacterSetsArgs,
    ) -> GetAdbsCharacterSetsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:oracle/getAdbsCharacterSets:getAdbsCharacterSets".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAdbsCharacterSetsResult {
            character_sets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("characterSets"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
        }
    }
}
