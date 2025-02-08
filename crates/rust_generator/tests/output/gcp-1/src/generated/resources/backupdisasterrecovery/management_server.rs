/// ## Example Usage
///
/// ### Backup Dr Management Server
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: vpc-network
///   privateIpAddress:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_address
///     properties:
///       name: vpc-network
///       addressType: INTERNAL
///       purpose: VPC_PEERING
///       prefixLength: 20
///       network: ${default.id}
///   defaultConnection:
///     type: gcp:servicenetworking:Connection
///     name: default
///     properties:
///       network: ${default.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${privateIpAddress.name}
///   ms-console:
///     type: gcp:backupdisasterrecovery:ManagementServer
///     properties:
///       location: us-central1
///       name: ms-console
///       type: BACKUP_RESTORE
///     options:
///       dependsOn:
///         - ${defaultConnection}
/// ```
///
/// ## Import
///
/// ManagementServer can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/managementServers/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, ManagementServer can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:backupdisasterrecovery/managementServer:ManagementServer default projects/{{project}}/locations/{{location}}/managementServers/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:backupdisasterrecovery/managementServer:ManagementServer default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:backupdisasterrecovery/managementServer:ManagementServer default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod management_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagementServerArgs {
        /// The location for the management server (management console)
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of management server (management console)
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Network details to create management server (management console).
        /// Structure is documented below.
        #[builder(into, default)]
        pub networks: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::backupdisasterrecovery::ManagementServerNetwork>,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of management server (management console).
        /// Default value is `BACKUP_RESTORE`.
        /// Possible values are: `BACKUP_RESTORE`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ManagementServerResult {
        /// The location for the management server (management console)
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The management console URI
        /// Structure is documented below.
        pub management_uris: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::backupdisasterrecovery::ManagementServerManagementUri,
            >,
        >,
        /// The name of management server (management console)
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Network details to create management server (management console).
        /// Structure is documented below.
        pub networks: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::backupdisasterrecovery::ManagementServerNetwork>,
            >,
        >,
        /// The oauth2ClientId of management console.
        pub oauth2_client_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The type of management server (management console).
        /// Default value is `BACKUP_RESTORE`.
        /// Possible values are: `BACKUP_RESTORE`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ManagementServerArgs,
    ) -> ManagementServerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let networks_binding = args.networks.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:backupdisasterrecovery/managementServer:ManagementServer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networks".into(),
                    value: &networks_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ManagementServerResult {
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            management_uris: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managementUris"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            networks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networks"),
            ),
            oauth2_client_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("oauth2ClientId"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
