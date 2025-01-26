pub mod get_bootstrap_brokers {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBootstrapBrokersArgs {
        /// ARN of the cluster the nodes belong to.
        #[builder(into)]
        pub cluster_arn: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetBootstrapBrokersResult {
        /// Comma separated list of one or more hostname:port pairs of kafka brokers suitable to bootstrap connectivity to the kafka cluster.
        pub bootstrap_brokers: pulumi_wasm_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and SASL IAM port pairs.
        pub bootstrap_brokers_public_sasl_iam: pulumi_wasm_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and SASL SCRAM port pairs.
        pub bootstrap_brokers_public_sasl_scram: pulumi_wasm_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and TLS port pairs.
        pub bootstrap_brokers_public_tls: pulumi_wasm_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and SASL IAM port pairs.
        pub bootstrap_brokers_sasl_iam: pulumi_wasm_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and SASL SCRAM port pairs.
        pub bootstrap_brokers_sasl_scram: pulumi_wasm_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and TLS port pairs.
        pub bootstrap_brokers_tls: pulumi_wasm_rust::Output<String>,
        /// A string containing one or more DNS names (or IP addresses) and SASL IAM port pairs for VPC connectivity.
        pub bootstrap_brokers_vpc_connectivity_sasl_iam: pulumi_wasm_rust::Output<
            String,
        >,
        /// A string containing one or more DNS names (or IP addresses) and SASL SCRAM port pairs for VPC connectivity.
        pub bootstrap_brokers_vpc_connectivity_sasl_scram: pulumi_wasm_rust::Output<
            String,
        >,
        /// A string containing one or more DNS names (or IP addresses) and TLS port pairs for VPC connectivity.
        pub bootstrap_brokers_vpc_connectivity_tls: pulumi_wasm_rust::Output<String>,
        pub cluster_arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetBootstrapBrokersArgs,
    ) -> GetBootstrapBrokersResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_arn_binding = args.cluster_arn.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:msk/getBootstrapBrokers:getBootstrapBrokers".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterArn".into(),
                    value: &cluster_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bootstrapBrokers".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapBrokersPublicSaslIam".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapBrokersPublicSaslScram".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapBrokersPublicTls".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapBrokersSaslIam".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapBrokersSaslScram".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapBrokersTls".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapBrokersVpcConnectivitySaslIam".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapBrokersVpcConnectivitySaslScram".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapBrokersVpcConnectivityTls".into(),
                },
                register_interface::ResultField {
                    name: "clusterArn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBootstrapBrokersResult {
            bootstrap_brokers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokers").unwrap(),
            ),
            bootstrap_brokers_public_sasl_iam: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokersPublicSaslIam").unwrap(),
            ),
            bootstrap_brokers_public_sasl_scram: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokersPublicSaslScram").unwrap(),
            ),
            bootstrap_brokers_public_tls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokersPublicTls").unwrap(),
            ),
            bootstrap_brokers_sasl_iam: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokersSaslIam").unwrap(),
            ),
            bootstrap_brokers_sasl_scram: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokersSaslScram").unwrap(),
            ),
            bootstrap_brokers_tls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokersTls").unwrap(),
            ),
            bootstrap_brokers_vpc_connectivity_sasl_iam: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokersVpcConnectivitySaslIam").unwrap(),
            ),
            bootstrap_brokers_vpc_connectivity_sasl_scram: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokersVpcConnectivitySaslScram").unwrap(),
            ),
            bootstrap_brokers_vpc_connectivity_tls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokersVpcConnectivityTls").unwrap(),
            ),
            cluster_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterArn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
        }
    }
}
