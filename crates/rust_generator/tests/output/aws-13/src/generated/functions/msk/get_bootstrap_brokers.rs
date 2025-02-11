#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_bootstrap_brokers {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBootstrapBrokersArgs {
        /// ARN of the cluster the nodes belong to.
        #[builder(into)]
        pub cluster_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetBootstrapBrokersResult {
        /// Comma separated list of one or more hostname:port pairs of kafka brokers suitable to bootstrap connectivity to the kafka cluster.
        pub bootstrap_brokers: pulumi_gestalt_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and SASL IAM port pairs.
        pub bootstrap_brokers_public_sasl_iam: pulumi_gestalt_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and SASL SCRAM port pairs.
        pub bootstrap_brokers_public_sasl_scram: pulumi_gestalt_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and TLS port pairs.
        pub bootstrap_brokers_public_tls: pulumi_gestalt_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and SASL IAM port pairs.
        pub bootstrap_brokers_sasl_iam: pulumi_gestalt_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and SASL SCRAM port pairs.
        pub bootstrap_brokers_sasl_scram: pulumi_gestalt_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and TLS port pairs.
        pub bootstrap_brokers_tls: pulumi_gestalt_rust::Output<String>,
        /// A string containing one or more DNS names (or IP addresses) and SASL IAM port pairs for VPC connectivity.
        pub bootstrap_brokers_vpc_connectivity_sasl_iam: pulumi_gestalt_rust::Output<
            String,
        >,
        /// A string containing one or more DNS names (or IP addresses) and SASL SCRAM port pairs for VPC connectivity.
        pub bootstrap_brokers_vpc_connectivity_sasl_scram: pulumi_gestalt_rust::Output<
            String,
        >,
        /// A string containing one or more DNS names (or IP addresses) and TLS port pairs for VPC connectivity.
        pub bootstrap_brokers_vpc_connectivity_tls: pulumi_gestalt_rust::Output<String>,
        pub cluster_arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBootstrapBrokersArgs,
    ) -> GetBootstrapBrokersResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_arn_binding = args.cluster_arn.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:msk/getBootstrapBrokers:getBootstrapBrokers".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterArn".into(),
                    value: &cluster_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBootstrapBrokersResult {
            bootstrap_brokers: o.get_field("bootstrapBrokers"),
            bootstrap_brokers_public_sasl_iam: o
                .get_field("bootstrapBrokersPublicSaslIam"),
            bootstrap_brokers_public_sasl_scram: o
                .get_field("bootstrapBrokersPublicSaslScram"),
            bootstrap_brokers_public_tls: o.get_field("bootstrapBrokersPublicTls"),
            bootstrap_brokers_sasl_iam: o.get_field("bootstrapBrokersSaslIam"),
            bootstrap_brokers_sasl_scram: o.get_field("bootstrapBrokersSaslScram"),
            bootstrap_brokers_tls: o.get_field("bootstrapBrokersTls"),
            bootstrap_brokers_vpc_connectivity_sasl_iam: o
                .get_field("bootstrapBrokersVpcConnectivitySaslIam"),
            bootstrap_brokers_vpc_connectivity_sasl_scram: o
                .get_field("bootstrapBrokersVpcConnectivitySaslScram"),
            bootstrap_brokers_vpc_connectivity_tls: o
                .get_field("bootstrapBrokersVpcConnectivityTls"),
            cluster_arn: o.get_field("clusterArn"),
            id: o.get_field("id"),
        }
    }
}
