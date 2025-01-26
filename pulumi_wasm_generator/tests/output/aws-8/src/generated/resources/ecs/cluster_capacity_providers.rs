/// Manages the capacity providers of an ECS Cluster.
///
/// More information about capacity providers can be found in the [ECS User Guide](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-capacity-providers.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster::create(
///         "example",
///         ClusterArgs::builder().name("my-cluster").build_struct(),
///     );
///     let exampleClusterCapacityProviders = cluster_capacity_providers::create(
///         "exampleClusterCapacityProviders",
///         ClusterCapacityProvidersArgs::builder()
///             .capacity_providers(vec!["FARGATE",])
///             .cluster_name("${example.name}")
///             .default_capacity_provider_strategies(
///                 vec![
///                     ClusterCapacityProvidersDefaultCapacityProviderStrategy::builder()
///                     .base(1).capacityProvider("FARGATE").weight(100).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ECS cluster capacity providers using the `cluster_name` attribute. For example:
///
/// ```sh
/// $ pulumi import aws:ecs/clusterCapacityProviders:ClusterCapacityProviders example my-cluster
/// ```
pub mod cluster_capacity_providers {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterCapacityProvidersArgs {
        /// Set of names of one or more capacity providers to associate with the cluster. Valid values also include `FARGATE` and `FARGATE_SPOT`.
        #[builder(into, default)]
        pub capacity_providers: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of the ECS cluster to manage capacity providers for.
        #[builder(into)]
        pub cluster_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Set of capacity provider strategies to use by default for the cluster. Detailed below.
        #[builder(into, default)]
        pub default_capacity_provider_strategies: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::ecs::ClusterCapacityProvidersDefaultCapacityProviderStrategy,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterCapacityProvidersResult {
        /// Set of names of one or more capacity providers to associate with the cluster. Valid values also include `FARGATE` and `FARGATE_SPOT`.
        pub capacity_providers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Name of the ECS cluster to manage capacity providers for.
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// Set of capacity provider strategies to use by default for the cluster. Detailed below.
        pub default_capacity_provider_strategies: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::ecs::ClusterCapacityProvidersDefaultCapacityProviderStrategy,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ClusterCapacityProvidersArgs,
    ) -> ClusterCapacityProvidersResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let capacity_providers_binding = args
            .capacity_providers
            .get_output(context)
            .get_inner();
        let cluster_name_binding = args.cluster_name.get_output(context).get_inner();
        let default_capacity_provider_strategies_binding = args
            .default_capacity_provider_strategies
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecs/clusterCapacityProviders:ClusterCapacityProviders".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "capacityProviders".into(),
                    value: &capacity_providers_binding,
                },
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "defaultCapacityProviderStrategies".into(),
                    value: &default_capacity_provider_strategies_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClusterCapacityProvidersResult {
            capacity_providers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("capacityProviders"),
            ),
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterName"),
            ),
            default_capacity_provider_strategies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultCapacityProviderStrategies"),
            ),
        }
    }
}
