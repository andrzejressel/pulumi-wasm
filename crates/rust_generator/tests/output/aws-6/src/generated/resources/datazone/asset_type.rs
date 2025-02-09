/// Resource for managing an AWS DataZone Asset Type.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod asset_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssetTypeArgs {
        /// The description of the custom asset type.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The unique identifier of the Amazon DataZone domain where the custom asset type is being created.
        #[builder(into)]
        pub domain_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The metadata forms that are to be attached to the custom asset type.
        #[builder(into, default)]
        pub forms_inputs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::datazone::AssetTypeFormsInput>>,
        >,
        /// The name of the custom asset type.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The unique identifier of the Amazon DataZone project that owns the custom asset type.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub owning_project_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datazone::AssetTypeTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct AssetTypeResult {
        /// The timestamp when the custom asset type was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The user who created the custom asset type.
        pub created_by: pulumi_gestalt_rust::Output<String>,
        /// The description of the custom asset type.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The unique identifier of the Amazon DataZone domain where the custom asset type is being created.
        pub domain_identifier: pulumi_gestalt_rust::Output<String>,
        /// The metadata forms that are to be attached to the custom asset type.
        pub forms_inputs: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::datazone::AssetTypeFormsInput>>,
        >,
        /// The name of the custom asset type.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier of the Amazon DataZone project that owns the custom asset type.
        ///
        /// The following arguments are optional:
        pub owning_project_identifier: pulumi_gestalt_rust::Output<String>,
        /// The revision of the asset type.
        pub revision: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::datazone::AssetTypeTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AssetTypeArgs,
    ) -> AssetTypeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let domain_identifier_binding = args.domain_identifier.get_output(context);
        let forms_inputs_binding = args.forms_inputs.get_output(context);
        let name_binding = args.name.get_output(context);
        let owning_project_identifier_binding = args
            .owning_project_identifier
            .get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:datazone/assetType:AssetType".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainIdentifier".into(),
                    value: domain_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "formsInputs".into(),
                    value: forms_inputs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "owningProjectIdentifier".into(),
                    value: owning_project_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AssetTypeResult {
            created_at: o.get_field("createdAt"),
            created_by: o.get_field("createdBy"),
            description: o.get_field("description"),
            domain_identifier: o.get_field("domainIdentifier"),
            forms_inputs: o.get_field("formsInputs"),
            name: o.get_field("name"),
            owning_project_identifier: o.get_field("owningProjectIdentifier"),
            revision: o.get_field("revision"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
