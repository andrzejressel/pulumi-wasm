/// Manages a Communication Service.
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
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .data_location("United States")
///             .name("example-communicationservice")
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
/// $ pulumi import azure:communication/service:Service example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Communication/communicationServices/communicationService1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// The location where the Communication service stores its data at rest. Possible values are `Africa`, `Asia Pacific`, `Australia`, `Brazil`, `Canada`, `Europe`, `France`, `Germany`, `India`, `Japan`, `Korea`, `Norway`, `Switzerland`, `UAE`, `UK`, `usgov` and `United States`. Defaults to `United States`. Changing this forces a new Communication Service to be created.
        #[builder(into, default)]
        pub data_location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Communication Service resource. Changing this forces a new Communication Service to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Communication Service should exist. Changing this forces a new Communication Service to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Communication Service.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// The location where the Communication service stores its data at rest. Possible values are `Africa`, `Asia Pacific`, `Australia`, `Brazil`, `Canada`, `Europe`, `France`, `Germany`, `India`, `Japan`, `Korea`, `Norway`, `Switzerland`, `UAE`, `UK`, `usgov` and `United States`. Defaults to `United States`. Changing this forces a new Communication Service to be created.
        pub data_location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Communication Service resource. Changing this forces a new Communication Service to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The primary connection string of the Communication Service.
        pub primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The primary key of the Communication Service.
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Communication Service should exist. Changing this forces a new Communication Service to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The secondary connection string of the Communication Service.
        pub secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The secondary key of the Communication Service.
        pub secondary_key: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Communication Service.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ServiceArgs,
    ) -> ServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let data_location_binding = args.data_location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:communication/service:Service".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataLocation".into(),
                    value: &data_location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServiceResult {
            data_location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataLocation"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            primary_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryConnectionString"),
            ),
            primary_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryKey"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryConnectionString"),
            ),
            secondary_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryKey"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
