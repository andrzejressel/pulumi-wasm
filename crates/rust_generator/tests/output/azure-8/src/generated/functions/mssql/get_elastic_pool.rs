#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_elastic_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetElasticPoolArgs {
        /// The name of the elastic pool.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group which contains the elastic pool.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the SQL Server which contains the elastic pool.
        #[builder(into)]
        pub server_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetElasticPoolResult {
        /// The type of enclave being used by the elastic pool.
        pub enclave_type: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The license type to apply for this elastic pool.
        pub license_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The max data size of the elastic pool in bytes.
        pub max_size_bytes: pulumi_gestalt_rust::Output<i32>,
        /// The max data size of the elastic pool in gigabytes.
        pub max_size_gb: pulumi_gestalt_rust::Output<f64>,
        /// Specifies the SKU Name for this Elasticpool.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The maximum capacity any one database can consume.
        pub per_db_max_capacity: pulumi_gestalt_rust::Output<i32>,
        /// The minimum capacity all databases are guaranteed.
        pub per_db_min_capacity: pulumi_gestalt_rust::Output<i32>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub server_name: pulumi_gestalt_rust::Output<String>,
        /// A `sku` block as defined below.
        pub skus: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::mssql::GetElasticPoolSkus>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Whether or not this elastic pool is zone redundant.
        pub zone_redundant: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetElasticPoolArgs,
    ) -> GetElasticPoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let server_name_binding = args.server_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:mssql/getElasticPool:getElasticPool".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverName".into(),
                    value: server_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetElasticPoolResult {
            enclave_type: o.get_field("enclaveType"),
            id: o.get_field("id"),
            license_type: o.get_field("licenseType"),
            location: o.get_field("location"),
            max_size_bytes: o.get_field("maxSizeBytes"),
            max_size_gb: o.get_field("maxSizeGb"),
            name: o.get_field("name"),
            per_db_max_capacity: o.get_field("perDbMaxCapacity"),
            per_db_min_capacity: o.get_field("perDbMinCapacity"),
            resource_group_name: o.get_field("resourceGroupName"),
            server_name: o.get_field("serverName"),
            skus: o.get_field("skus"),
            tags: o.get_field("tags"),
            zone_redundant: o.get_field("zoneRedundant"),
        }
    }
}
