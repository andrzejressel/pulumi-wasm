/// Mute Findings is a volume management feature in Security Command Center
/// that lets you manually or programmatically hide irrelevant findings,
/// and create filters to automatically silence existing and future
/// findings based on criteria you specify.
///
///
/// To get more information about OrganizationMuteConfig, see:
///
/// * [API documentation](https://cloud.google.com/security-command-center/docs/reference/rest/v2/organizations.muteConfigs)
///
/// ## Example Usage
///
/// ### Scc V2 Organization Mute Config Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = v_2_organization_mute_config::create(
///         "default",
///         V2OrganizationMuteConfigArgs::builder()
///             .description(
///                 "My custom Cloud Security Command Center Finding Organization mute Configuration",
///             )
///             .filter("severity = \"HIGH\"")
///             .location("global")
///             .mute_config_id("my-config")
///             .organization("123456789")
///             .type_("STATIC")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// OrganizationMuteConfig can be imported using any of these accepted formats:
///
/// * `organizations/{{organization}}/locations/{{location}}/muteConfigs/{{mute_config_id}}`
///
/// * `{{organization}}/{{location}}/{{mute_config_id}}`
///
/// When using the `pulumi import` command, OrganizationMuteConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2OrganizationMuteConfig:V2OrganizationMuteConfig default organizations/{{organization}}/locations/{{location}}/muteConfigs/{{mute_config_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2OrganizationMuteConfig:V2OrganizationMuteConfig default {{organization}}/{{location}}/{{mute_config_id}}
/// ```
///
pub mod v_2_organization_mute_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2OrganizationMuteConfigArgs {
        /// A description of the mute config.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// An expression that defines the filter to apply across create/update
        /// events of findings. While creating a filter string, be mindful of
        /// the scope in which the mute configuration is being created. E.g.,
        /// If a filter contains project = X but is created under the
        /// project = Y scope, it might not match any findings.
        #[builder(into)]
        pub filter: pulumi_wasm_rust::InputOrOutput<String>,
        /// location Id is provided by organization. If not provided, Use global as default.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Unique identifier provided by the client within the parent scope.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub mute_config_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The organization whose Cloud Security Command Center the Mute
        /// Config lives in.
        #[builder(into)]
        pub organization: pulumi_wasm_rust::InputOrOutput<String>,
        /// The type of the mute config.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct V2OrganizationMuteConfigResult {
        /// The time at which the mute config was created. This field is set by
        /// the server and will be ignored if provided on config creation.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// A description of the mute config.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// An expression that defines the filter to apply across create/update
        /// events of findings. While creating a filter string, be mindful of
        /// the scope in which the mute configuration is being created. E.g.,
        /// If a filter contains project = X but is created under the
        /// project = Y scope, it might not match any findings.
        pub filter: pulumi_wasm_rust::Output<String>,
        /// location Id is provided by organization. If not provided, Use global as default.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Email address of the user who last edited the mute config. This
        /// field is set by the server and will be ignored if provided on
        /// config creation or update.
        pub most_recent_editor: pulumi_wasm_rust::Output<String>,
        /// Unique identifier provided by the client within the parent scope.
        ///
        ///
        /// - - -
        pub mute_config_id: pulumi_wasm_rust::Output<String>,
        /// Name of the mute config. Its format is
        /// organizations/{organization}/locations/global/muteConfigs/{configId},
        /// folders/{folder}/locations/global/muteConfigs/{configId},
        /// or projects/{project}/locations/global/muteConfigs/{configId}
        pub name: pulumi_wasm_rust::Output<String>,
        /// The organization whose Cloud Security Command Center the Mute
        /// Config lives in.
        pub organization: pulumi_wasm_rust::Output<String>,
        /// The type of the mute config.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Output only. The most recent time at which the mute config was
        /// updated. This field is set by the server and will be ignored if
        /// provided on config creation or update.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: V2OrganizationMuteConfigArgs,
    ) -> V2OrganizationMuteConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let filter_binding = args.filter.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let mute_config_id_binding = args.mute_config_id.get_output(context).get_inner();
        let organization_binding = args.organization.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securitycenter/v2OrganizationMuteConfig:V2OrganizationMuteConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "muteConfigId".into(),
                    value: &mute_config_id_binding,
                },
                register_interface::ObjectField {
                    name: "organization".into(),
                    value: &organization_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "filter".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "mostRecentEditor".into(),
                },
                register_interface::ResultField {
                    name: "muteConfigId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "organization".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        V2OrganizationMuteConfigResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filter").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            most_recent_editor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mostRecentEditor").unwrap(),
            ),
            mute_config_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("muteConfigId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            organization: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organization").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
