/// Allows creation and management of a Google Cloud Platform project.
///
/// Projects created with this resource must be associated with an Organization.
/// See the [Organization documentation](https://cloud.google.com/resource-manager/docs/quickstarts) for more details.
///
/// The user or service account that is running this provider when creating a `gcp.organizations.Project`
/// resource must have `roles/resourcemanager.projectCreator` on the specified organization. See the
/// [Access Control for Organizations Using IAM](https://cloud.google.com/resource-manager/docs/access-control-org)
/// doc for more information.
///
/// > This resource reads the specified billing account on every pulumi up and plan operation so you must have permissions on the specified billing account.
///
/// > It is recommended to use the `constraints/compute.skipDefaultNetworkCreation` [constraint](https://www.terraform.io/docs/providers/google/r/google_organization_policy.html) to remove the default network instead of setting `auto_create_network` to false, when possible.
///
/// > It may take a while for the attached tag bindings to be deleted after the project is scheduled to be deleted.
///
/// To get more information about projects, see:
///
/// * [API documentation](https://cloud.google.com/resource-manager/reference/rest/v1/projects)
/// * How-to Guides
///     * [Creating and managing projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects)
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myProject = project::create(
///         "myProject",
///         ProjectArgs::builder()
///             .name("My Project")
///             .org_id("1234567")
///             .project_id("your-project-id")
///             .build_struct(),
///     );
/// }
/// ```
///
/// To create a project under a specific folder
///
/// ```yaml
/// resources:
///   myProject-in-a-folder:
///     type: gcp:organizations:Project
///     name: my_project-in-a-folder
///     properties:
///       name: My Project
///       projectId: your-project-id
///       folderId: ${department1.name}
///   department1:
///     type: gcp:organizations:Folder
///     properties:
///       displayName: Department 1
///       parent: organizations/1234567
/// ```
///
/// To create a project with a tag
///
/// ```yaml
/// resources:
///   myProject:
///     type: gcp:organizations:Project
///     name: my_project
///     properties:
///       name: My Project
///       projectId: your-project-id
///       orgId: '1234567'
///       tags:
///         1234567/env: staging
/// ```
///
/// ## Import
///
/// Projects can be imported using the `project_id`, e.g.
///
/// * `{{project_id}}`
///
/// When using the `pulumi import` command, Projects can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:projects/usageExportBucket:UsageExportBucket default {{project_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod usage_export_bucket {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UsageExportBucketArgs {
        /// The bucket to store reports in.
        #[builder(into)]
        pub bucket_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A prefix for the reports, for instance, the project name.
        #[builder(into, default)]
        pub prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project to set the export bucket on. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct UsageExportBucketResult {
        /// The bucket to store reports in.
        pub bucket_name: pulumi_gestalt_rust::Output<String>,
        /// A prefix for the reports, for instance, the project name.
        pub prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// The project to set the export bucket on. If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UsageExportBucketArgs,
    ) -> UsageExportBucketResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_name_binding = args.bucket_name.get_output(context);
        let prefix_binding = args.prefix.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:projects/usageExportBucket:UsageExportBucket".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucketName".into(),
                    value: bucket_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prefix".into(),
                    value: prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UsageExportBucketResult {
            bucket_name: o.get_field("bucketName"),
            prefix: o.get_field("prefix"),
            project: o.get_field("project"),
        }
    }
}
