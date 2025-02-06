/// Manages an IoT Central Organization
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resource
///       location: West Europe
///   exampleApplication:
///     type: azure:iotcentral:Application
///     name: example
///     properties:
///       name: example-iotcentral-app
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       subDomain: example-iotcentral-app-subdomain
///       displayName: example-iotcentral-app-display-name
///       sku: ST1
///       template: iotc-default@1.0.0
///       tags:
///         Foo: Bar
///   exampleParent:
///     type: azure:iotcentral:Organization
///     name: example_parent
///     properties:
///       iotcentralApplicationId: ${exampleApplication.id}
///       organizationId: example-parent-organization-id
///       displayName: Org example parent
///   exampleOrganization:
///     type: azure:iotcentral:Organization
///     name: example
///     properties:
///       iotcentralApplicationId: ${exampleApplication.id}
///       organizationId: example-child-organization-id
///       displayName: Org example
///       parentOrganizationId: ${exampleParent.organizationId}
/// ```
///
/// ## Import
///
/// The IoT Central Organization can be imported using the `id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iotcentral/organization:Organization example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.IoTCentral/iotApps/example/organizations/example
/// ```
///
pub mod organization {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationArgs {
        /// Custom `display_name` for the organization.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The application `id`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iotcentral_application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the organization. Changing this forces a new resource to be created.
        #[builder(into)]
        pub organization_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The `organization_id` of the parent organization. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub parent_organization_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationResult {
        /// Custom `display_name` for the organization.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The application `id`. Changing this forces a new resource to be created.
        pub iotcentral_application_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the organization. Changing this forces a new resource to be created.
        pub organization_id: pulumi_gestalt_rust::Output<String>,
        /// The `organization_id` of the parent organization. Changing this forces a new resource to be created.
        pub parent_organization_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: OrganizationArgs,
    ) -> OrganizationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let iotcentral_application_id_binding = args
            .iotcentral_application_id
            .get_output(context)
            .get_inner();
        let organization_id_binding = args
            .organization_id
            .get_output(context)
            .get_inner();
        let parent_organization_id_binding = args
            .parent_organization_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:iotcentral/organization:Organization".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "iotcentralApplicationId".into(),
                    value: &iotcentral_application_id_binding,
                },
                register_interface::ObjectField {
                    name: "organizationId".into(),
                    value: &organization_id_binding,
                },
                register_interface::ObjectField {
                    name: "parentOrganizationId".into(),
                    value: &parent_organization_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        OrganizationResult {
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            iotcentral_application_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iotcentralApplicationId"),
            ),
            organization_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("organizationId"),
            ),
            parent_organization_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parentOrganizationId"),
            ),
        }
    }
}
