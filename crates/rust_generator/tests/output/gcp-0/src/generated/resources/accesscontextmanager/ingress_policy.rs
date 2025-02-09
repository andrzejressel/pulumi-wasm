/// This resource has been deprecated, please refer to ServicePerimeterIngressPolicy.
///
///
/// To get more information about IngressPolicy, see:
///
/// * [API documentation](https://cloud.google.com/access-context-manager/docs/reference/rest/v1/accessPolicies.servicePerimeters#ingresspolicy)
///
/// ## Import
///
/// IngressPolicy can be imported using any of these accepted formats:
///
/// * `{{ingress_policy_name}}/{{resource}}`
///
/// When using the `pulumi import` command, IngressPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:accesscontextmanager/ingressPolicy:IngressPolicy default {{ingress_policy_name}}/{{resource}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ingress_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IngressPolicyArgs {
        /// The name of the Service Perimeter to add this resource to.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub ingress_policy_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A GCP resource that is inside of the service perimeter.
        #[builder(into)]
        pub resource: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IngressPolicyResult {
        /// The name of the Service Perimeter to add this resource to.
        ///
        ///
        /// - - -
        pub ingress_policy_name: pulumi_gestalt_rust::Output<String>,
        /// A GCP resource that is inside of the service perimeter.
        pub resource: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IngressPolicyArgs,
    ) -> IngressPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let ingress_policy_name_binding = args.ingress_policy_name.get_output(context);
        let resource_binding = args.resource.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:accesscontextmanager/ingressPolicy:IngressPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ingressPolicyName".into(),
                    value: ingress_policy_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resource".into(),
                    value: resource_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IngressPolicyResult {
            ingress_policy_name: o.get_field("ingressPolicyName"),
            resource: o.get_field("resource"),
        }
    }
}
