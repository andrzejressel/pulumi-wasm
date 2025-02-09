/// Provides connectivity for VMware Engine private clouds.
///
///
/// To get more information about Network, see:
///
/// * [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.vmwareEngineNetworks)
///
/// ## Example Usage
///
/// ### Vmware Engine Network Standard
///
///
/// ```yaml
/// resources:
///   vmw-engine-network:
///     type: gcp:vmwareengine:Network
///     properties:
///       name: standard-nw
///       location: global
///       type: STANDARD
///       description: VMwareEngine standard network sample
/// ```
/// ### Vmware Engine Network Legacy
///
///
/// ```yaml
/// resources:
///   vmw-engine-network:
///     type: gcp:vmwareengine:Network
///     properties:
///       project: ${acceptance.project}
///       name: us-west1-default
///       location: us-west1
///       type: LEGACY
///       description: VMwareEngine legacy network sample
///   acceptance:
///     type: gcp:projects:Service
///     properties:
///       project: ${acceptanceProject.projectId}
///       service: vmwareengine.googleapis.com
///     options:
///       dependsOn:
///         - ${wait60Seconds}
///   # there can be only 1 Legacy network per region for a given project,
///   # so creating new project for isolation in CI.
///   acceptanceProject:
///     type: gcp:organizations:Project
///     name: acceptance
///     properties:
///       name: vmw-proj
///       projectId: vmw-proj
///       orgId: '123456789'
///       billingAccount: 000000-0000000-0000000-000000
///       deletionPolicy: DELETE
///   wait60Seconds:
///     type: time:sleep
///     name: wait_60_seconds
///     properties:
///       createDuration: 60s
///     options:
///       dependsOn:
///         - ${acceptanceProject}
/// ```
///
/// ## Import
///
/// Network can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/vmwareEngineNetworks/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Network can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/network:Network default projects/{{project}}/locations/{{location}}/vmwareEngineNetworks/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/network:Network default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/network:Network default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkArgs {
        /// User-provided description for this VMware Engine network.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location where the VMwareEngineNetwork should reside.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the VMwareEngineNetwork.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// VMware Engine network type.
        /// Possible values are: `LEGACY`, `STANDARD`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkResult {
        /// User-provided description for this VMware Engine network.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The location where the VMwareEngineNetwork should reside.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The ID of the VMwareEngineNetwork.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// State of the VMware Engine network.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// VMware Engine network type.
        /// Possible values are: `LEGACY`, `STANDARD`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// System-generated unique identifier for the resource.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// VMware Engine service VPC networks that provide connectivity from a private cloud to customer projects,
        /// the internet, and other Google Cloud services.
        /// Structure is documented below.
        pub vpc_networks: pulumi_gestalt_rust::Output<
            Vec<super::super::types::vmwareengine::NetworkVpcNetwork>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkArgs,
    ) -> NetworkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:vmwareengine/network:Network".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkResult {
            description: o.get_field("description"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            state: o.get_field("state"),
            type_: o.get_field("type"),
            uid: o.get_field("uid"),
            vpc_networks: o.get_field("vpcNetworks"),
        }
    }
}
