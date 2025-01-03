/// Provides an Amazon Lex Slot Type resource. For more information see
/// [Amazon Lex: How It Works](https://docs.aws.amazon.com/lex/latest/dg/how-it-works.html)
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let flowerTypes = slot_type::create(
///         "flowerTypes",
///         SlotTypeArgs::builder()
///             .create_version(true)
///             .description("Types of flowers to order")
///             .enumeration_values(
///                 vec![
///                     SlotTypeEnumerationValue::builder().synonyms(vec!["Lirium",
///                     "Martagon",]).value("lilies").build_struct(),
///                     SlotTypeEnumerationValue::builder().synonyms(vec!["Eduardoregelia",
///                     "Podonix",]).value("tulips").build_struct(),
///                 ],
///             )
///             .name("FlowerTypes")
///             .value_selection_strategy("ORIGINAL_VALUE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import slot types using their name. For example:
///
/// ```sh
/// $ pulumi import aws:lex/slotType:SlotType flower_types FlowerTypes
/// ```
pub mod slot_type {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SlotTypeArgs {
        /// Determines if a new slot type version is created when the initial resource is created and on each
        /// update. Defaults to `false`.
        #[builder(into, default)]
        pub create_version: pulumi_wasm_rust::Output<Option<bool>>,
        /// A description of the slot type. Must be less than or equal to 200 characters in length.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of EnumerationValue objects that defines the values that
        /// the slot type can take. Each value can have a list of synonyms, which are additional values that help
        /// train the machine learning model about the values that it resolves for a slot. Attributes are
        /// documented under enumeration_value.
        #[builder(into)]
        pub enumeration_values: pulumi_wasm_rust::Output<
            Vec<super::super::types::lex::SlotTypeEnumerationValue>,
        >,
        /// The name of the slot type. The name is not case sensitive. Must be less than or equal to 100 characters in length.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Determines the slot resolution strategy that Amazon Lex
        /// uses to return slot type values. `ORIGINAL_VALUE` returns the value entered by the user if the user
        /// value is similar to the slot value. `TOP_RESOLUTION` returns the first value in the resolution list
        /// if there is a resolution list for the slot, otherwise null is returned. Defaults to `ORIGINAL_VALUE`.
        #[builder(into, default)]
        pub value_selection_strategy: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SlotTypeResult {
        /// Checksum identifying the version of the slot type that was created. The checksum is
        /// not included as an argument because the resource will add it automatically when updating the slot type.
        pub checksum: pulumi_wasm_rust::Output<String>,
        /// Determines if a new slot type version is created when the initial resource is created and on each
        /// update. Defaults to `false`.
        pub create_version: pulumi_wasm_rust::Output<Option<bool>>,
        /// The date when the slot type version was created.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// A description of the slot type. Must be less than or equal to 200 characters in length.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of EnumerationValue objects that defines the values that
        /// the slot type can take. Each value can have a list of synonyms, which are additional values that help
        /// train the machine learning model about the values that it resolves for a slot. Attributes are
        /// documented under enumeration_value.
        pub enumeration_values: pulumi_wasm_rust::Output<
            Vec<super::super::types::lex::SlotTypeEnumerationValue>,
        >,
        /// The date when the `$LATEST` version of this slot type was updated.
        pub last_updated_date: pulumi_wasm_rust::Output<String>,
        /// The name of the slot type. The name is not case sensitive. Must be less than or equal to 100 characters in length.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Determines the slot resolution strategy that Amazon Lex
        /// uses to return slot type values. `ORIGINAL_VALUE` returns the value entered by the user if the user
        /// value is similar to the slot value. `TOP_RESOLUTION` returns the first value in the resolution list
        /// if there is a resolution list for the slot, otherwise null is returned. Defaults to `ORIGINAL_VALUE`.
        pub value_selection_strategy: pulumi_wasm_rust::Output<Option<String>>,
        /// The version of the slot type.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SlotTypeArgs) -> SlotTypeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let create_version_binding = args.create_version.get_inner();
        let description_binding = args.description.get_inner();
        let enumeration_values_binding = args.enumeration_values.get_inner();
        let name_binding = args.name.get_inner();
        let value_selection_strategy_binding = args.value_selection_strategy.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lex/slotType:SlotType".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "createVersion".into(),
                    value: &create_version_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enumerationValues".into(),
                    value: &enumeration_values_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "valueSelectionStrategy".into(),
                    value: &value_selection_strategy_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "checksum".into(),
                },
                register_interface::ResultField {
                    name: "createVersion".into(),
                },
                register_interface::ResultField {
                    name: "createdDate".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "enumerationValues".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedDate".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "valueSelectionStrategy".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SlotTypeResult {
            checksum: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("checksum").unwrap(),
            ),
            create_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createVersion").unwrap(),
            ),
            created_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdDate").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            enumeration_values: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enumerationValues").unwrap(),
            ),
            last_updated_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedDate").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            value_selection_strategy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("valueSelectionStrategy").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
