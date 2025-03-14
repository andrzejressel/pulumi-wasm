/// ## Example Usage
///
/// ### Compute Network Edge Security Service Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = network_edge_security_service::create(
///         "default",
///         NetworkEdgeSecurityServiceArgs::builder()
///             .description("My basic resource")
///             .name("my-edge-security-service")
///             .region("us-east1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// NetworkEdgeSecurityService can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/networkEdgeSecurityServices/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, NetworkEdgeSecurityService can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/networkEdgeSecurityService:NetworkEdgeSecurityService default projects/{{project}}/regions/{{region}}/networkEdgeSecurityServices/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkEdgeSecurityService:NetworkEdgeSecurityService default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkEdgeSecurityService:NetworkEdgeSecurityService default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkEdgeSecurityService:NetworkEdgeSecurityService default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_edge_security_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkEdgeSecurityServiceArgs {
        /// Free-text description of the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is created.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the gateway security policy.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource URL for the network edge security service associated with this network edge security service.
        #[builder(into, default)]
        pub security_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NetworkEdgeSecurityServiceResult {
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Free-text description of the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Fingerprint of this resource. A hash of the contents stored in this object. This field is used in optimistic locking. This field will be ignored when inserting a NetworkEdgeSecurityService.
        /// An up-to-date fingerprint must be provided in order to update the NetworkEdgeSecurityService, otherwise the request will fail with error 412 conditionNotMet.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Name of the resource. Provided by the client when the resource is created.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The region of the gateway security policy.
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource URL for the network edge security service associated with this network edge security service.
        pub security_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Server-defined URL for the resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Server-defined URL for this resource with the resource id.
        pub self_link_with_service_id: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier for the resource. This identifier is defined by the server.
        pub service_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkEdgeSecurityServiceArgs,
    ) -> NetworkEdgeSecurityServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let security_policy_binding = args.security_policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/networkEdgeSecurityService:NetworkEdgeSecurityService"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityPolicy".into(),
                    value: &security_policy_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkEdgeSecurityServiceResult {
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            fingerprint: o.get_field("fingerprint"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            security_policy: o.get_field("securityPolicy"),
            self_link: o.get_field("selfLink"),
            self_link_with_service_id: o.get_field("selfLinkWithServiceId"),
            service_id: o.get_field("serviceId"),
        }
    }
}
