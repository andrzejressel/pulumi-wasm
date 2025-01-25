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
pub mod management_server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagementServerArgs {
        /// The location for the management server (management console)
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of management server (management console)
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Network details to create management server (management console).
        /// Structure is documented below.
        #[builder(into, default)]
        pub networks: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::types::backupdisasterrecovery::ManagementServerNetwork>,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The type of management server (management console).
        /// Default value is `BACKUP_RESTORE`.
        /// Possible values are: `BACKUP_RESTORE`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ManagementServerResult {
        /// The location for the management server (management console)
        pub location: pulumi_wasm_rust::Output<String>,
        /// The management console URI
        /// Structure is documented below.
        pub management_uris: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::backupdisasterrecovery::ManagementServerManagementUri,
            >,
        >,
        /// The name of management server (management console)
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// Network details to create management server (management console).
        /// Structure is documented below.
        pub networks: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::backupdisasterrecovery::ManagementServerNetwork>,
            >,
        >,
        /// The oauth2ClientId of management console.
        pub oauth2_client_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The type of management server (management console).
        /// Default value is `BACKUP_RESTORE`.
        /// Possible values are: `BACKUP_RESTORE`.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ManagementServerArgs,
    ) -> ManagementServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managementUris".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networks".into(),
                },
                register_interface::ResultField {
                    name: "oauth2ClientId".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ManagementServerResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            management_uris: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managementUris").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            networks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networks").unwrap(),
            ),
            oauth2_client_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("oauth2ClientId").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
