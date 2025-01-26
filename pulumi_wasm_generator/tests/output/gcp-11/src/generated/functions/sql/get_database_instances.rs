pub mod get_database_instances {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatabaseInstancesArgs {
        /// To filter out the Cloud SQL instances which are of the specified database version.
        #[builder(into, default)]
        pub database_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resources belong. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// To filter out the Cloud SQL instances which are located in the specified region.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// To filter out the Cloud SQL instances based on the current serving state of the database instance. Supported values include `SQL_INSTANCE_STATE_UNSPECIFIED`, `RUNNABLE`, `SUSPENDED`, `PENDING_DELETE`, `PENDING_CREATE`, `MAINTENANCE`, `FAILED`.
        #[builder(into, default)]
        pub state: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// To filter out the Cloud SQL instances based on the tier(or machine type) of the database instances.
        #[builder(into, default)]
        pub tier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// To filter out the Cloud SQL instances which are located in the specified zone. This zone refers to the Compute Engine zone that the instance is currently serving from.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDatabaseInstancesResult {
        pub database_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instances: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::sql::GetDatabaseInstancesInstance>,
        >,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        pub state: pulumi_wasm_rust::Output<Option<String>>,
        pub tier: pulumi_wasm_rust::Output<Option<String>>,
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDatabaseInstancesArgs,
    ) -> GetDatabaseInstancesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let database_version_binding = args
            .database_version
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let state_binding = args.state.get_output(context).get_inner();
        let tier_binding = args.tier.get_output(context).get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:sql/getDatabaseInstances:getDatabaseInstances".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "databaseVersion".into(),
                    value: &database_version_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
                register_interface::ObjectField {
                    name: "tier".into(),
                    value: &tier_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDatabaseInstancesResult {
            database_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("databaseVersion"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            instances: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instances"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            tier: pulumi_wasm_rust::__private::into_domain(o.extract_field("tier")),
            zone: pulumi_wasm_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
