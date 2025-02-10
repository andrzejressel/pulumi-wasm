/// Enables the Google Compute Engine
/// [Shared VPC](https://cloud.google.com/compute/docs/shared-vpc)
/// feature for a project, assigning it as a Shared VPC service project associated
/// with a given host project.
///
/// For more information, see,
/// [the Project API documentation](https://cloud.google.com/compute/docs/reference/latest/projects),
/// where the Shared VPC feature is referred to by its former name "XPN".
///
/// > **Note:** If Shared VPC Admin role is set at the folder level, use the google-beta provider. The google provider only supports this permission at project or organizational level currently. [[0]](https://cloud.google.com/vpc/docs/provisioning-shared-vpc#enable-shared-vpc-host)
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let service1 = shared_vpc_service_project::create(
///         "service1",
///         SharedVpcServiceProjectArgs::builder()
///             .host_project("host-project-id")
///             .service_project("service-project-id-1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// For a complete Shared VPC example with both host and service projects, see
/// [`gcp.compute.SharedVPCHostProject`](https://www.terraform.io/docs/providers/google/r/compute_shared_vpc_host_project.html).
///
/// ## Import
///
/// Google Compute Engine Shared VPC service project feature can be imported using the `host_project` and `service_project`, e.g.
///
/// * `{{host_project}/{{service_project}}`
///
/// When using the `pulumi import` command, Google Compute Engine Shared VPC service project can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/sharedVPCServiceProject:SharedVPCServiceProject default {{host_project}/{{service_project}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod shared_vpc_service_project {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedVPCServiceProjectArgs {
        /// The deletion policy for the shared VPC service. Setting ABANDON allows the resource to be abandoned rather than deleted. Possible values are: "ABANDON".
        #[builder(into, default)]
        pub deletion_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of a host project to associate.
        #[builder(into)]
        pub host_project: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project that will serve as a Shared VPC service project.
        #[builder(into)]
        pub service_project: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SharedVPCServiceProjectResult {
        /// The deletion policy for the shared VPC service. Setting ABANDON allows the resource to be abandoned rather than deleted. Possible values are: "ABANDON".
        pub deletion_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of a host project to associate.
        pub host_project: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project that will serve as a Shared VPC service project.
        pub service_project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SharedVPCServiceProjectArgs,
    ) -> SharedVPCServiceProjectResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let deletion_policy_binding = args.deletion_policy.get_output(context);
        let host_project_binding = args.host_project.get_output(context);
        let service_project_binding = args.service_project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/sharedVPCServiceProject:SharedVPCServiceProject".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionPolicy".into(),
                    value: deletion_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostProject".into(),
                    value: host_project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceProject".into(),
                    value: service_project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SharedVPCServiceProjectResult {
            deletion_policy: o.get_field("deletionPolicy"),
            host_project: o.get_field("hostProject"),
            service_project: o.get_field("serviceProject"),
        }
    }
}
