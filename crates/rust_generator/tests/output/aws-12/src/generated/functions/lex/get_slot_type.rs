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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSlotTypeArgs,
    ) -> GetSlotTypeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lex/getSlotType:getSlotType".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSlotTypeResult {
            checksum: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("checksum"),
            ),
            created_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            enumeration_values: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enumerationValues"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            last_updated_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastUpdatedDate"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            value_selection_strategy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("valueSelectionStrategy"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
