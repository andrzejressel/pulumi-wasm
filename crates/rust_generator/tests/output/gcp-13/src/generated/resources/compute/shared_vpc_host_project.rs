/// Enables the Google Compute Engine
/// [Shared VPC](https://cloud.google.com/compute/docs/shared-vpc)
/// feature for a project, assigning it as a Shared VPC host project.
///
/// For more information, see,
/// [the Project API documentation](https://cloud.google.com/compute/docs/reference/latest/projects),
/// where the Shared VPC feature is referred to by its former name "XPN".
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let host = shared_vpc_host_project::create(
///         "host",
///         SharedVpcHostProjectArgs::builder().project("host-project-id").build_struct(),
///     );
///     let service1 = shared_vpc_service_project::create(
///         "service1",
///         SharedVpcServiceProjectArgs::builder()
///             .host_project("${host.project}")
///             .service_project("service-project-id-1")
///             .build_struct(),
///     );
///     let service2 = shared_vpc_service_project::create(
///         "service2",
///         SharedVpcServiceProjectArgs::builder()
///             .host_project("${host.project}")
///             .service_project("service-project-id-2")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Google Compute Engine Shared VPC host project feature can be imported using `project`, e.g.
///
/// * `{{project_id}}`
///
/// When using the `pulumi import` command, Google Compute Engine Shared VPC host projects can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/sharedVPCHostProject:SharedVPCHostProject default {{project_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod shared_vpc_host_project {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedVPCHostProjectArgs {
        /// The ID of the project that will serve as a Shared VPC host project
        #[builder(into)]
        pub project: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SharedVPCHostProjectResult {
        /// The ID of the project that will serve as a Shared VPC host project
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SharedVPCHostProjectArgs,
    ) -> SharedVPCHostProjectResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/sharedVPCHostProject:SharedVPCHostProject".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SharedVPCHostProjectResult {
            project: o.get_field("project"),
        }
    }
}
