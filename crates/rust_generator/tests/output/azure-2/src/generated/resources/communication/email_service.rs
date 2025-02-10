/// Manages an Email Communication Service.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleEmailService = email_service::create(
///         "exampleEmailService",
///         EmailServiceArgs::builder()
///             .data_location("United States")
///             .name("example-emailcommunicationservice")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Communication Services can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:communication/emailService:EmailService example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Communication/emailServices/emailCommunicationService1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod email_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailServiceArgs {
        /// The location where the Email Communication service stores its data at rest. Possible values are `Africa`, `Asia Pacific`, `Australia`, `Brazil`, `Canada`, `Europe`, `France`, `Germany`, `India`, `Japan`, `Korea`, `Norway`, `Switzerland`, `UAE`, `UK` `usgov` and `United States`. Changing this forces a new Email Communication Service to be created.
        #[builder(into)]
        pub data_location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Email Communication Service resource. Changing this forces a new Email Communication Service to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Email Communication Service should exist. Changing this forces a new Email Communication Service to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Email Communication Service.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EmailServiceResult {
        /// The location where the Email Communication service stores its data at rest. Possible values are `Africa`, `Asia Pacific`, `Australia`, `Brazil`, `Canada`, `Europe`, `France`, `Germany`, `India`, `Japan`, `Korea`, `Norway`, `Switzerland`, `UAE`, `UK` `usgov` and `United States`. Changing this forces a new Email Communication Service to be created.
        pub data_location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Email Communication Service resource. Changing this forces a new Email Communication Service to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Email Communication Service should exist. Changing this forces a new Email Communication Service to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Email Communication Service.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EmailServiceArgs,
    ) -> EmailServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_location_binding = args.data_location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:communication/emailService:EmailService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataLocation".into(),
                    value: data_location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EmailServiceResult {
            data_location: o.get_field("dataLocation"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
