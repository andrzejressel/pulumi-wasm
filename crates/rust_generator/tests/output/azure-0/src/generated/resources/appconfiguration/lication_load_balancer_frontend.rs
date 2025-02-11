/// Manages an Application Gateway for Containers Frontend.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = lication_load_balancer::create(
///         "example",
///         LicationLoadBalancerArgs::builder()
///             .location("West Europe")
///             .name("example")
///             .resource_group_name("example")
///             .build_struct(),
///     );
///     let exampleLicationLoadBalancerFrontend = lication_load_balancer_frontend::create(
///         "exampleLicationLoadBalancerFrontend",
///         LicationLoadBalancerFrontendArgs::builder()
///             .application_load_balancer_id("${example.id}")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Application Gateway for Containers Frontend can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appconfiguration/licationLoadBalancerFrontend:LicationLoadBalancerFrontend example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ServiceNetworking/trafficControllers/alb1/frontends/frontend1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod lication_load_balancer_frontend {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LicationLoadBalancerFrontendArgs {
        /// The ID of the Application Gateway for Containers. Changing this forces a new resource to be created.
        #[builder(into)]
        pub application_load_balancer_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Application Gateway for Containers Frontend. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Application Gateway for Containers Frontend.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LicationLoadBalancerFrontendResult {
        /// The ID of the Application Gateway for Containers. Changing this forces a new resource to be created.
        pub application_load_balancer_id: pulumi_gestalt_rust::Output<String>,
        /// The Fully Qualified Domain Name of the DNS record associated to an Application Gateway for Containers Frontend.
        pub fully_qualified_domain_name: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Application Gateway for Containers Frontend. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Application Gateway for Containers Frontend.
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
        args: LicationLoadBalancerFrontendArgs,
    ) -> LicationLoadBalancerFrontendResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_load_balancer_id_binding = args
            .application_load_balancer_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appconfiguration/licationLoadBalancerFrontend:LicationLoadBalancerFrontend"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationLoadBalancerId".into(),
                    value: &application_load_balancer_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LicationLoadBalancerFrontendResult {
            application_load_balancer_id: o.get_field("applicationLoadBalancerId"),
            fully_qualified_domain_name: o.get_field("fullyQualifiedDomainName"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
        }
    }
}
