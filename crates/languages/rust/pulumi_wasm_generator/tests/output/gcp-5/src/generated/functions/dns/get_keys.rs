pub mod get_keys {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKeysArgs {
        /// The name or id of the Cloud DNS managed zone.
        #[builder(into)]
        pub managed_zone: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If `project` is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetKeysResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A list of Key-signing key (KSK) records. Structure is documented below. Additionally, the DS record is provided:
        pub key_signing_keys: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dns::GetKeysKeySigningKey>,
        >,
        pub managed_zone: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// A list of Zone-signing key (ZSK) records. Structure is documented below.
        pub zone_signing_keys: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dns::GetKeysZoneSigningKey>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetKeysArgs,
    ) -> GetKeysResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let managed_zone_binding = args.managed_zone.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:dns/getKeys:getKeys".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "managedZone".into(),
                    value: &managed_zone_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetKeysResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            key_signing_keys: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keySigningKeys"),
            ),
            managed_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("managedZone"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            zone_signing_keys: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("zoneSigningKeys"),
            ),
        }
    }
}
