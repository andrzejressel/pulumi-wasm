#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_slot_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSlotTypeArgs {
        /// Name of the slot type. The name is case sensitive.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version of the slot type.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSlotTypeResult {
        /// Checksum identifying the version of the slot type that was created. The checksum is
        /// not included as an argument because the resource will add it automatically when updating the slot type.
        pub checksum: pulumi_gestalt_rust::Output<String>,
        /// Date when the slot type version was created.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Description of the slot type.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Set of EnumerationValue objects that defines the values that
        /// the slot type can take. Each value can have a set of synonyms, which are additional values that help
        /// train the machine learning model about the values that it resolves for a slot.
        pub enumeration_values: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::lex::GetSlotTypeEnumerationValue>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Date when the $LATEST version of this slot type was updated.
        pub last_updated_date: pulumi_gestalt_rust::Output<String>,
        /// Name of the slot type. The name is not case sensitive.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Determines the slot resolution strategy that Amazon Lex
        /// uses to return slot type values. `ORIGINAL_VALUE` returns the value entered by the user if the user
        /// value is similar to the slot value. `TOP_RESOLUTION` returns the first value in the resolution list
        /// if there is a resolution list for the slot, otherwise null is returned.
        pub value_selection_strategy: pulumi_gestalt_rust::Output<String>,
        /// Version of the slot type.
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSlotTypeArgs,
    ) -> GetSlotTypeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:lex/getSlotType:getSlotType".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSlotTypeResult {
            checksum: o.get_field("checksum"),
            created_date: o.get_field("createdDate"),
            description: o.get_field("description"),
            enumeration_values: o.get_field("enumerationValues"),
            id: o.get_field("id"),
            last_updated_date: o.get_field("lastUpdatedDate"),
            name: o.get_field("name"),
            value_selection_strategy: o.get_field("valueSelectionStrategy"),
            version: o.get_field("version"),
        }
    }
}
