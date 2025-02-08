/// Manages a Container App.
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
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("acctest-01")
///             .resource_group_name("${example.name}")
///             .retention_in_days(30)
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleApp = app::create(
///         "exampleApp",
///         AppArgs::builder()
///             .container_app_environment_id("${exampleEnvironment.id}")
///             .name("example-app")
///             .resource_group_name("${example.name}")
///             .revision_mode("Single")
///             .template(
///                 AppTemplate::builder()
///                     .containers(
///                         vec![
///                             AppTemplateContainer::builder().cpu(0.25)
///                             .image("mcr.microsoft.com/k8se/quickstart:latest")
///                             .memory("0.5Gi").name("examplecontainerapp").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleEnvironment = environment::create(
///         "exampleEnvironment",
///         EnvironmentArgs::builder()
///             .location("${example.location}")
///             .log_analytics_workspace_id("${exampleAnalyticsWorkspace.id}")
///             .name("Example-Environment")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// A Container App can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerapp/app:App example "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.App/containerApps/myContainerApp"
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod app {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppArgs {
        /// The ID of the Container App Environment within which this Container App should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_app_environment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `dapr` block as detailed below.
        #[builder(into, default)]
        pub dapr: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerapp::AppDapr>,
        >,
        /// An `identity` block as detailed below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerapp::AppIdentity>,
        >,
        /// An `ingress` block as detailed below.
        #[builder(into, default)]
        pub ingress: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerapp::AppIngress>,
        >,
        /// The maximum of inactive revisions allowed for this Container App.
        #[builder(into, default)]
        pub max_inactive_revisions: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name for this Container App. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `registry` block as detailed below.
        #[builder(into, default)]
        pub registries: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::containerapp::AppRegistry>>,
        >,
        /// The name of the resource group in which the Container App Environment is to be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The revisions operational mode for the Container App. Possible values include `Single` and `Multiple`. In `Single` mode, a single revision is in operation at any given time. In `Multiple` mode, more than one revision can be active at a time and can be configured with load distribution via the `traffic_weight` block in the `ingress` configuration.
        #[builder(into)]
        pub revision_mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `secret` block as detailed below.
        #[builder(into, default)]
        pub secrets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::containerapp::AppSecret>>,
        >,
        /// A mapping of tags to assign to the Container App.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `template` block as detailed below.
        #[builder(into)]
        pub template: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::containerapp::AppTemplate,
        >,
        /// The name of the Workload Profile in the Container App Environment to place this Container App.
        ///
        /// > **Note:** Omit this value to use the default `Consumption` Workload Profile.
        #[builder(into, default)]
        pub workload_profile_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AppResult {
        /// The ID of the Container App Environment within which this Container App should exist. Changing this forces a new resource to be created.
        pub container_app_environment_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Custom Domain Verification for this Container App.
        pub custom_domain_verification_id: pulumi_gestalt_rust::Output<String>,
        /// A `dapr` block as detailed below.
        pub dapr: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerapp::AppDapr>,
        >,
        /// An `identity` block as detailed below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerapp::AppIdentity>,
        >,
        /// An `ingress` block as detailed below.
        pub ingress: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerapp::AppIngress>,
        >,
        /// The FQDN of the Latest Revision of the Container App.
        pub latest_revision_fqdn: pulumi_gestalt_rust::Output<String>,
        /// The name of the latest Container Revision.
        pub latest_revision_name: pulumi_gestalt_rust::Output<String>,
        /// The location this Container App is deployed in. This is the same as the Environment in which it is deployed.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The maximum of inactive revisions allowed for this Container App.
        pub max_inactive_revisions: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name for this Container App. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of the Public IP Addresses which the Container App uses for outbound network access.
        pub outbound_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A `registry` block as detailed below.
        pub registries: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::containerapp::AppRegistry>>,
        >,
        /// The name of the resource group in which the Container App Environment is to be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The revisions operational mode for the Container App. Possible values include `Single` and `Multiple`. In `Single` mode, a single revision is in operation at any given time. In `Multiple` mode, more than one revision can be active at a time and can be configured with load distribution via the `traffic_weight` block in the `ingress` configuration.
        pub revision_mode: pulumi_gestalt_rust::Output<String>,
        /// One or more `secret` block as detailed below.
        pub secrets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::containerapp::AppSecret>>,
        >,
        /// A mapping of tags to assign to the Container App.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `template` block as detailed below.
        pub template: pulumi_gestalt_rust::Output<
            super::super::types::containerapp::AppTemplate,
        >,
        /// The name of the Workload Profile in the Container App Environment to place this Container App.
        ///
        /// > **Note:** Omit this value to use the default `Consumption` Workload Profile.
        pub workload_profile_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AppArgs,
    ) -> AppResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let container_app_environment_id_binding = args
            .container_app_environment_id
            .get_output(context)
            .get_inner();
        let dapr_binding = args.dapr.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let ingress_binding = args.ingress.get_output(context).get_inner();
        let max_inactive_revisions_binding = args
            .max_inactive_revisions
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let registries_binding = args.registries.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let revision_mode_binding = args.revision_mode.get_output(context).get_inner();
        let secrets_binding = args.secrets.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let template_binding = args.template.get_output(context).get_inner();
        let workload_profile_name_binding = args
            .workload_profile_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerapp/app:App".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerAppEnvironmentId".into(),
                    value: &container_app_environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "dapr".into(),
                    value: &dapr_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "ingress".into(),
                    value: &ingress_binding,
                },
                register_interface::ObjectField {
                    name: "maxInactiveRevisions".into(),
                    value: &max_inactive_revisions_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "registries".into(),
                    value: &registries_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "revisionMode".into(),
                    value: &revision_mode_binding,
                },
                register_interface::ObjectField {
                    name: "secrets".into(),
                    value: &secrets_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "template".into(),
                    value: &template_binding,
                },
                register_interface::ObjectField {
                    name: "workloadProfileName".into(),
                    value: &workload_profile_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AppResult {
            container_app_environment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("containerAppEnvironmentId"),
            ),
            custom_domain_verification_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customDomainVerificationId"),
            ),
            dapr: pulumi_gestalt_rust::__private::into_domain(o.extract_field("dapr")),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            ingress: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ingress"),
            ),
            latest_revision_fqdn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("latestRevisionFqdn"),
            ),
            latest_revision_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("latestRevisionName"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            max_inactive_revisions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxInactiveRevisions"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            outbound_ip_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outboundIpAddresses"),
            ),
            registries: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("registries"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            revision_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("revisionMode"),
            ),
            secrets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secrets"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            template: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("template"),
            ),
            workload_profile_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workloadProfileName"),
            ),
        }
    }
}
