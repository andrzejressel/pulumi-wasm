/// Resource for managing an AWS DataZone Asset Type.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = asset_type::create(
///         "test",
///         AssetTypeArgs::builder()
///             .description("example")
///             .domain_identifier("${testAwsDatazoneDomain.id}")
///             .name("example")
///             .owning_project_identifier("${testAwsDatazoneProject.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DataZone Asset Type using the `domain_identifier,name`. For example:
///
/// ```sh
/// $ pulumi import aws:datazone/assetType:AssetType example domain-id-12345678,example
/// ```
pub mod asset_type {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssetTypeArgs {
        /// The description of the custom asset type.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique identifier of the Amazon DataZone domain where the custom asset type is being created.
        #[builder(into)]
        pub domain_identifier: pulumi_wasm_rust::Output<String>,
        /// The metadata forms that are to be attached to the custom asset type.
        #[builder(into, default)]
        pub forms_inputs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::datazone::AssetTypeFormsInput>>,
        >,
        /// The name of the custom asset type.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique identifier of the Amazon DataZone project that owns the custom asset type.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub owning_project_identifier: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::datazone::AssetTypeTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct AssetTypeResult {
        /// The timestamp when the custom asset type was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// The user who created the custom asset type.
        pub created_by: pulumi_wasm_rust::Output<String>,
        /// The description of the custom asset type.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique identifier of the Amazon DataZone domain where the custom asset type is being created.
        pub domain_identifier: pulumi_wasm_rust::Output<String>,
        /// The metadata forms that are to be attached to the custom asset type.
        pub forms_inputs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::datazone::AssetTypeFormsInput>>,
        >,
        /// The name of the custom asset type.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The unique identifier of the Amazon DataZone project that owns the custom asset type.
        ///
        /// The following arguments are optional:
        pub owning_project_identifier: pulumi_wasm_rust::Output<String>,
        /// The revision of the asset type.
        pub revision: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::datazone::AssetTypeTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AssetTypeArgs) -> AssetTypeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let domain_identifier_binding = args.domain_identifier.get_inner();
        let forms_inputs_binding = args.forms_inputs.get_inner();
        let name_binding = args.name.get_inner();
        let owning_project_identifier_binding = args
            .owning_project_identifier
            .get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datazone/assetType:AssetType".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "domainIdentifier".into(),
                    value: &domain_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "formsInputs".into(),
                    value: &forms_inputs_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "owningProjectIdentifier".into(),
                    value: &owning_project_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "createdBy".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "domainIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "formsInputs".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "owningProjectIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "revision".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AssetTypeResult {
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            created_by: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdBy").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            domain_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainIdentifier").unwrap(),
            ),
            forms_inputs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("formsInputs").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owning_project_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owningProjectIdentifier").unwrap(),
            ),
            revision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revision").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}