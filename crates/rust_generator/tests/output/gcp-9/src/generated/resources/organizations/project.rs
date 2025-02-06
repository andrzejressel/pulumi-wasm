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
/// $ pulumi import gcp:organizations/project:Project default {{project_id}}
/// ```
///
pub mod project {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectArgs {
        /// Create the 'default' network automatically. Default true. If set to false, the default network will be deleted. Note
        /// that, for quota purposes, you will still need to have 1 network slot available to create the project successfully, even
        /// if you set auto_create_network to false, since the network will exist momentarily.
        #[builder(into, default)]
        pub auto_create_network: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The alphanumeric ID of the billing account this project
        /// belongs to. The user or service account performing this operation with the provider
        /// must have at mininum Billing Account User privileges (`roles/billing.user`) on the billing account.
        /// See [Google Cloud Billing API Access Control](https://cloud.google.com/billing/docs/how-to/billing-access)
        /// for more details.
        #[builder(into, default)]
        pub billing_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub deletion_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The numeric ID of the folder this project should be
        /// created under. Only one of `org_id` or `folder_id` may be
        /// specified. If the `folder_id` is specified, then the project is
        /// created under the specified folder. Changing this forces the
        /// project to be migrated to the newly specified folder.
        #[builder(into, default)]
        pub folder_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A set of key/value label pairs to assign to the project.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The display name of the project.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The numeric ID of the organization this project belongs to.
        /// Changing this forces a new project to be created.  Only one of
        /// `org_id` or `folder_id` may be specified. If the `org_id` is
        /// specified then the project is created at the top level. Changing
        /// this forces the project to be migrated to the newly specified
        /// organization.
        #[builder(into, default)]
        pub org_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project ID. Changing this forces a new project to be created.
        #[builder(into, default)]
        pub project_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored when empty. The field is immutable and causes resource replacement when mutated. This field is only set at create time and modifying this field after creation will trigger recreation. To apply tags to an existing resource, see the `gcp.tags.TagValue` resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProjectResult {
        /// Create the 'default' network automatically. Default true. If set to false, the default network will be deleted. Note
        /// that, for quota purposes, you will still need to have 1 network slot available to create the project successfully, even
        /// if you set auto_create_network to false, since the network will exist momentarily.
        pub auto_create_network: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The alphanumeric ID of the billing account this project
        /// belongs to. The user or service account performing this operation with the provider
        /// must have at mininum Billing Account User privileges (`roles/billing.user`) on the billing account.
        /// See [Google Cloud Billing API Access Control](https://cloud.google.com/billing/docs/how-to/billing-access)
        /// for more details.
        pub billing_account: pulumi_gestalt_rust::Output<Option<String>>,
        pub deletion_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The numeric ID of the folder this project should be
        /// created under. Only one of `org_id` or `folder_id` may be
        /// specified. If the `folder_id` is specified, then the project is
        /// created under the specified folder. Changing this forces the
        /// project to be migrated to the newly specified folder.
        pub folder_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A set of key/value label pairs to assign to the project.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The display name of the project.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The numeric identifier of the project.
        pub number: pulumi_gestalt_rust::Output<String>,
        /// The numeric ID of the organization this project belongs to.
        /// Changing this forces a new project to be created.  Only one of
        /// `org_id` or `folder_id` may be specified. If the `org_id` is
        /// specified then the project is created at the top level. Changing
        /// this forces the project to be migrated to the newly specified
        /// organization.
        pub org_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The project ID. Changing this forces a new project to be created.
        pub project_id: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored when empty. The field is immutable and causes resource replacement when mutated. This field is only set at create time and modifying this field after creation will trigger recreation. To apply tags to an existing resource, see the `gcp.tags.TagValue` resource.
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
        args: ProjectArgs,
    ) -> ProjectResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let auto_create_network_binding = args
            .auto_create_network
            .get_output(context)
            .get_inner();
        let billing_account_binding = args
            .billing_account
            .get_output(context)
            .get_inner();
        let deletion_policy_binding = args
            .deletion_policy
            .get_output(context)
            .get_inner();
        let folder_id_binding = args.folder_id.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let org_id_binding = args.org_id.get_output(context).get_inner();
        let project_id_binding = args.project_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:organizations/project:Project".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoCreateNetwork".into(),
                    value: &auto_create_network_binding,
                },
                register_interface::ObjectField {
                    name: "billingAccount".into(),
                    value: &billing_account_binding,
                },
                register_interface::ObjectField {
                    name: "deletionPolicy".into(),
                    value: &deletion_policy_binding,
                },
                register_interface::ObjectField {
                    name: "folderId".into(),
                    value: &folder_id_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding,
                },
                register_interface::ObjectField {
                    name: "projectId".into(),
                    value: &project_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProjectResult {
            auto_create_network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoCreateNetwork"),
            ),
            billing_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("billingAccount"),
            ),
            deletion_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionPolicy"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            folder_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("folderId"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            number: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("number"),
            ),
            org_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("orgId"),
            ),
            project_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("projectId"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
