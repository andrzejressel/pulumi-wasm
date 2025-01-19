/// An OS Config resource representing a guest configuration policy. These policies represent
/// the desired state for VM instance guest environments including packages to install or remove,
/// package repository configurations, and software to install.
///
/// To get more information about GuestPolicies, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/osconfig/rest)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/compute/docs/os-config-management)
///
/// ## Example Usage
///
/// ### Os Config Guest Policies Basic
///
///
/// ```yaml
/// resources:
///   foobar:
///     type: gcp:compute:Instance
///     properties:
///       name: guest-policy-inst
///       machineType: e2-medium
///       zone: us-central1-a
///       canIpForward: false
///       tags:
///         - foo
///         - bar
///       bootDisk:
///         initializeParams:
///           image: ${myImage.selfLink}
///       networkInterfaces:
///         - network: default
///       metadata:
///         foo: bar
///   guestPolicies:
///     type: gcp:osconfig:GuestPolicies
///     name: guest_policies
///     properties:
///       guestPolicyId: guest-policy
///       assignment:
///         instances:
///           - ${foobar.id}
///       packages:
///         - name: my-package
///           desiredState: UPDATED
/// variables:
///   myImage:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-11
///         project: debian-cloud
/// ```
/// ### Os Config Guest Policies Packages
///
///
/// ```yaml
/// resources:
///   guestPolicies:
///     type: gcp:osconfig:GuestPolicies
///     name: guest_policies
///     properties:
///       guestPolicyId: guest-policy
///       assignment:
///         groupLabels:
///           - labels:
///               color: red
///               env: test
///           - labels:
///               color: blue
///               env: test
///       packages:
///         - name: my-package
///           desiredState: INSTALLED
///         - name: bad-package-1
///           desiredState: REMOVED
///         - name: bad-package-2
///           desiredState: REMOVED
///           manager: APT
///       packageRepositories:
///         - apt:
///             uri: https://packages.cloud.google.com/apt
///             archiveType: DEB
///             distribution: cloud-sdk-stretch
///             components:
///               - main
///         - yum:
///             id: google-cloud-sdk
///             displayName: Google Cloud SDK
///             baseUrl: https://packages.cloud.google.com/yum/repos/cloud-sdk-el7-x86_64
///             gpgKeys:
///               - https://packages.cloud.google.com/yum/doc/yum-key.gpg
///               - https://packages.cloud.google.com/yum/doc/rpm-package-key.gpg
/// ```
/// ### Os Config Guest Policies Recipes
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let guestPolicies = guest_policies::create(
///         "guestPolicies",
///         GuestPoliciesArgs::builder()
///             .assignment(
///                 GuestPoliciesAssignment::builder()
///                     .zones(vec!["us-east1-b", "us-east1-d",])
///                     .build_struct(),
///             )
///             .guest_policy_id("guest-policy")
///             .recipes(
///                 vec![
///                     GuestPoliciesRecipe::builder()
///                     .artifacts(vec![GuestPoliciesRecipeArtifact::builder()
///                     .gcs(GuestPoliciesRecipeArtifactGcs::builder().bucket("my-bucket")
///                     .generation(1546030865175603).object("executable.msi")
///                     .build_struct()).id("guest-policy-artifact-id").build_struct(),])
///                     .desiredState("INSTALLED")
///                     .installSteps(vec![GuestPoliciesRecipeInstallStep::builder()
///                     .msiInstallation(GuestPoliciesRecipeInstallStepMsiInstallation::builder()
///                     .artifactId("guest-policy-artifact-id").build_struct())
///                     .build_struct(),]).name("guest-policy-recipe").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// GuestPolicies can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/guestPolicies/{{guest_policy_id}}`
///
/// * `{{project}}/{{guest_policy_id}}`
///
/// * `{{guest_policy_id}}`
///
/// When using the `pulumi import` command, GuestPolicies can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:osconfig/guestPolicies:GuestPolicies default projects/{{project}}/guestPolicies/{{guest_policy_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:osconfig/guestPolicies:GuestPolicies default {{project}}/{{guest_policy_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:osconfig/guestPolicies:GuestPolicies default {{guest_policy_id}}
/// ```
///
pub mod guest_policies {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GuestPoliciesArgs {
        /// Specifies the VM instances that are assigned to this policy. This allows you to target sets
        /// or groups of VM instances by different parameters such as labels, names, OS, or zones.
        /// If left empty, all VM instances underneath this policy are targeted.
        /// At the same level in the resource hierarchy (that is within a project), the service prevents
        /// the creation of multiple policies that conflict with each other.
        /// For more information, see how the service
        /// [handles assignment conflicts](https://cloud.google.com/compute/docs/os-config-management/create-guest-policy#handle-conflicts).
        /// Structure is documented below.
        #[builder(into)]
        pub assignment: pulumi_wasm_rust::Output<
            super::super::types::osconfig::GuestPoliciesAssignment,
        >,
        /// Description of the guest policy. Length of the description is limited to 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The etag for this guest policy. If this is provided on update, it must match the server's etag.
        #[builder(into, default)]
        pub etag: pulumi_wasm_rust::Output<Option<String>>,
        /// The logical name of the guest policy in the project with the following restrictions:
        /// * Must contain only lowercase letters, numbers, and hyphens.
        /// * Must start with a letter.
        /// * Must be between 1-63 characters.
        /// * Must end with a number or a letter.
        /// * Must be unique within the project.
        #[builder(into)]
        pub guest_policy_id: pulumi_wasm_rust::Output<String>,
        /// A list of package repositories to configure on the VM instance. This is done before any other configs are applied so
        /// they can use these repos. Package repositories are only configured if the corresponding package manager(s) are
        /// available.
        #[builder(into, default)]
        pub package_repositories: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::osconfig::GuestPoliciesPackageRepository>>,
        >,
        /// The software packages to be managed by this policy.
        #[builder(into, default)]
        pub packages: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::osconfig::GuestPoliciesPackage>>,
        >,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of Recipes to install on the VM instance.
        #[builder(into, default)]
        pub recipes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::osconfig::GuestPoliciesRecipe>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GuestPoliciesResult {
        /// Specifies the VM instances that are assigned to this policy. This allows you to target sets
        /// or groups of VM instances by different parameters such as labels, names, OS, or zones.
        /// If left empty, all VM instances underneath this policy are targeted.
        /// At the same level in the resource hierarchy (that is within a project), the service prevents
        /// the creation of multiple policies that conflict with each other.
        /// For more information, see how the service
        /// [handles assignment conflicts](https://cloud.google.com/compute/docs/os-config-management/create-guest-policy#handle-conflicts).
        /// Structure is documented below.
        pub assignment: pulumi_wasm_rust::Output<
            super::super::types::osconfig::GuestPoliciesAssignment,
        >,
        /// Time this guest policy was created. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds.
        /// Example: "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Description of the guest policy. Length of the description is limited to 1024 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The etag for this guest policy. If this is provided on update, it must match the server's etag.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The logical name of the guest policy in the project with the following restrictions:
        /// * Must contain only lowercase letters, numbers, and hyphens.
        /// * Must start with a letter.
        /// * Must be between 1-63 characters.
        /// * Must end with a number or a letter.
        /// * Must be unique within the project.
        pub guest_policy_id: pulumi_wasm_rust::Output<String>,
        /// Unique name of the resource in this project using one of the following forms: projects/{project_number}/guestPolicies/{guestPolicyId}.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of package repositories to configure on the VM instance. This is done before any other configs are applied so
        /// they can use these repos. Package repositories are only configured if the corresponding package manager(s) are
        /// available.
        pub package_repositories: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::osconfig::GuestPoliciesPackageRepository>>,
        >,
        /// The software packages to be managed by this policy.
        pub packages: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::osconfig::GuestPoliciesPackage>>,
        >,
        pub project: pulumi_wasm_rust::Output<String>,
        /// A list of Recipes to install on the VM instance.
        pub recipes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::osconfig::GuestPoliciesRecipe>>,
        >,
        /// Last time this guest policy was updated. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds.
        /// Example: "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: GuestPoliciesArgs) -> GuestPoliciesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let assignment_binding = args.assignment.get_inner();
        let description_binding = args.description.get_inner();
        let etag_binding = args.etag.get_inner();
        let guest_policy_id_binding = args.guest_policy_id.get_inner();
        let package_repositories_binding = args.package_repositories.get_inner();
        let packages_binding = args.packages.get_inner();
        let project_binding = args.project.get_inner();
        let recipes_binding = args.recipes.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:osconfig/guestPolicies:GuestPolicies".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "assignment".into(),
                    value: &assignment_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "etag".into(),
                    value: &etag_binding,
                },
                register_interface::ObjectField {
                    name: "guestPolicyId".into(),
                    value: &guest_policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "packageRepositories".into(),
                    value: &package_repositories_binding,
                },
                register_interface::ObjectField {
                    name: "packages".into(),
                    value: &packages_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "recipes".into(),
                    value: &recipes_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "assignment".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "guestPolicyId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "packageRepositories".into(),
                },
                register_interface::ResultField {
                    name: "packages".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "recipes".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GuestPoliciesResult {
            assignment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assignment").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            guest_policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("guestPolicyId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            package_repositories: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("packageRepositories").unwrap(),
            ),
            packages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("packages").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            recipes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recipes").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
