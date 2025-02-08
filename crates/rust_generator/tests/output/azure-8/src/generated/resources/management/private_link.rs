/// Manages a Resource Management Private Link to restrict access for managing resources in the tenant.
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
///     let examplePrivateLink = private_link::create(
///         "examplePrivateLink",
///         PrivateLinkArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// An existing Resource Management Private Link can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:management/privateLink:PrivateLink example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.Authorization/resourceManagementPrivateLinks/link1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod private_link {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrivateLinkArgs {
        /// The Azure Region where the Resource Management Private Link should exist. Changing this forces a new Resource Management Private Link to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of this Resource Management Private Link. Changing this forces a new Resource Management Private Link to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group within which this Resource Management Private Link should exist. Changing this forces a new Resource Management Private Link to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PrivateLinkResult {
        /// The Azure Region where the Resource Management Private Link should exist. Changing this forces a new Resource Management Private Link to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of this Resource Management Private Link. Changing this forces a new Resource Management Private Link to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group within which this Resource Management Private Link should exist. Changing this forces a new Resource Management Private Link to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PrivateLinkArgs,
    ) -> PrivateLinkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:management/privateLink:PrivateLink".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PrivateLinkResult {
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}
