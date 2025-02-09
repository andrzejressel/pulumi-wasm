#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_instance_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceTypeArgs {
        /// Instance
        #[builder(into)]
        pub instance_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceTypeResult {
        /// `true` if auto recovery is supported.
        pub auto_recovery_supported: pulumi_gestalt_rust::Output<bool>,
        /// `true` if it is a bare metal instance type.
        pub bare_metal: pulumi_gestalt_rust::Output<bool>,
        /// `true` if the instance type is a burstable performance instance type.
        pub burstable_performance_supported: pulumi_gestalt_rust::Output<bool>,
        /// `true`  if the instance type is a current generation.
        pub current_generation: pulumi_gestalt_rust::Output<bool>,
        /// `true` if Dedicated Hosts are supported on the instance type.
        pub dedicated_hosts_supported: pulumi_gestalt_rust::Output<bool>,
        /// Default number of cores for the instance type.
        pub default_cores: pulumi_gestalt_rust::Output<i32>,
        /// The  default  number of threads per core for the instance type.
        pub default_threads_per_core: pulumi_gestalt_rust::Output<i32>,
        /// Default number of vCPUs for the instance type.
        pub default_vcpus: pulumi_gestalt_rust::Output<i32>,
        /// Indicates whether Amazon EBS encryption is supported.
        pub ebs_encryption_support: pulumi_gestalt_rust::Output<String>,
        /// Whether non-volatile memory express (NVMe) is supported.
        pub ebs_nvme_support: pulumi_gestalt_rust::Output<String>,
        /// Indicates that the instance type is Amazon EBS-optimized.
        pub ebs_optimized_support: pulumi_gestalt_rust::Output<String>,
        /// The baseline bandwidth performance for an EBS-optimized instance type, in Mbps.
        pub ebs_performance_baseline_bandwidth: pulumi_gestalt_rust::Output<i32>,
        /// The baseline input/output storage operations per seconds for an EBS-optimized instance type.
        pub ebs_performance_baseline_iops: pulumi_gestalt_rust::Output<i32>,
        /// The baseline throughput performance for an EBS-optimized instance type, in MBps.
        pub ebs_performance_baseline_throughput: pulumi_gestalt_rust::Output<f64>,
        /// The maximum bandwidth performance for an EBS-optimized instance type, in Mbps.
        pub ebs_performance_maximum_bandwidth: pulumi_gestalt_rust::Output<i32>,
        /// The maximum input/output storage operations per second for an EBS-optimized instance type.
        pub ebs_performance_maximum_iops: pulumi_gestalt_rust::Output<i32>,
        /// The maximum throughput performance for an EBS-optimized instance type, in MBps.
        pub ebs_performance_maximum_throughput: pulumi_gestalt_rust::Output<f64>,
        /// Whether Elastic Fabric Adapter (EFA) is supported.
        pub efa_supported: pulumi_gestalt_rust::Output<bool>,
        /// Whether Elastic Network Adapter (ENA) is supported.
        pub ena_support: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether encryption in-transit between instances is supported.
        pub encryption_in_transit_supported: pulumi_gestalt_rust::Output<bool>,
        /// Describes the FPGA accelerator settings for the instance type.
        /// * `fpgas.#.count` - The count of FPGA accelerators for the instance type.
        /// * `fpgas.#.manufacturer` - The manufacturer of the FPGA accelerator.
        /// * `fpgas.#.memory_size` - The size (in MiB) for the memory available to the FPGA accelerator.
        /// * `fpgas.#.name` - The name of the FPGA accelerator.
        pub fpgas: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceTypeFpga>,
        >,
        /// `true` if the instance type is eligible for the free tier.
        pub free_tier_eligible: pulumi_gestalt_rust::Output<bool>,
        /// Describes the GPU accelerators for the instance type.
        /// * `gpus.#.count` - The number of GPUs for the instance type.
        /// * `gpus.#.manufacturer` - The manufacturer of the GPU accelerator.
        /// * `gpus.#.memory_size` - The size (in MiB) for the memory available to the GPU accelerator.
        /// * `gpus.#.name` - The name of the GPU accelerator.
        pub gpuses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceTypeGpus>,
        >,
        /// `true` if On-Demand hibernation is supported.
        pub hibernation_supported: pulumi_gestalt_rust::Output<bool>,
        /// Hypervisor used for the instance type.
        pub hypervisor: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Describes the Inference accelerators for the instance type.
        /// * `inference_accelerators.#.count` - The number of Inference accelerators for the instance type.
        /// * `inference_accelerators.#.manufacturer` - The manufacturer of the Inference accelerator.
        /// * `inference_accelerators.#.name` - The name of the Inference accelerator.
        pub inference_accelerators: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceTypeInferenceAccelerator>,
        >,
        /// Describes the disks for the instance type.
        /// * `instance_disks.#.count` - The number of disks with this configuration.
        /// * `instance_disks.#.size` - The size of the disk in GB.
        /// * `instance_disks.#.type` - The type of disk.
        pub instance_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceTypeInstanceDisk>,
        >,
        /// `true` if instance storage is supported.
        pub instance_storage_supported: pulumi_gestalt_rust::Output<bool>,
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        /// `true` if IPv6 is supported.
        pub ipv6_supported: pulumi_gestalt_rust::Output<bool>,
        /// The maximum number of IPv4 addresses per network interface.
        pub maximum_ipv4_addresses_per_interface: pulumi_gestalt_rust::Output<i32>,
        /// The maximum number of IPv6 addresses per network interface.
        pub maximum_ipv6_addresses_per_interface: pulumi_gestalt_rust::Output<i32>,
        /// The maximum number of physical network cards that can be allocated to the instance.
        pub maximum_network_cards: pulumi_gestalt_rust::Output<i32>,
        /// The maximum number of network interfaces for the instance type.
        pub maximum_network_interfaces: pulumi_gestalt_rust::Output<i32>,
        /// Size of the instance memory, in MiB.
        pub memory_size: pulumi_gestalt_rust::Output<i32>,
        /// Describes the network performance.
        pub network_performance: pulumi_gestalt_rust::Output<String>,
        /// A list of architectures supported by the instance type.
        pub supported_architectures: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of supported placement groups types.
        pub supported_placement_strategies: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Indicates the supported root device types.
        pub supported_root_device_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Indicates whether the instance type is offered for spot or On-Demand.
        pub supported_usages_classes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The supported virtualization types.
        pub supported_virtualization_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The speed of the processor, in GHz.
        pub sustained_clock_speed: pulumi_gestalt_rust::Output<f64>,
        /// Total memory of all FPGA accelerators for the instance type (in MiB).
        pub total_fpga_memory: pulumi_gestalt_rust::Output<i32>,
        /// Total size of the memory for the GPU accelerators for the instance type (in MiB).
        pub total_gpu_memory: pulumi_gestalt_rust::Output<i32>,
        /// The total size of the instance disks, in GB.
        pub total_instance_storage: pulumi_gestalt_rust::Output<i32>,
        /// List of the valid number of cores that can be configured for the instance type.
        pub valid_cores: pulumi_gestalt_rust::Output<Vec<i32>>,
        /// List of the valid number of threads per core that can be configured for the instance type.
        pub valid_threads_per_cores: pulumi_gestalt_rust::Output<Vec<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetInstanceTypeArgs,
    ) -> GetInstanceTypeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_type_binding = args.instance_type.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getInstanceType:getInstanceType".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceType".into(),
                    value: instance_type_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetInstanceTypeResult {
            auto_recovery_supported: o.get_field("autoRecoverySupported"),
            bare_metal: o.get_field("bareMetal"),
            burstable_performance_supported: o
                .get_field("burstablePerformanceSupported"),
            current_generation: o.get_field("currentGeneration"),
            dedicated_hosts_supported: o.get_field("dedicatedHostsSupported"),
            default_cores: o.get_field("defaultCores"),
            default_threads_per_core: o.get_field("defaultThreadsPerCore"),
            default_vcpus: o.get_field("defaultVcpus"),
            ebs_encryption_support: o.get_field("ebsEncryptionSupport"),
            ebs_nvme_support: o.get_field("ebsNvmeSupport"),
            ebs_optimized_support: o.get_field("ebsOptimizedSupport"),
            ebs_performance_baseline_bandwidth: o
                .get_field("ebsPerformanceBaselineBandwidth"),
            ebs_performance_baseline_iops: o.get_field("ebsPerformanceBaselineIops"),
            ebs_performance_baseline_throughput: o
                .get_field("ebsPerformanceBaselineThroughput"),
            ebs_performance_maximum_bandwidth: o
                .get_field("ebsPerformanceMaximumBandwidth"),
            ebs_performance_maximum_iops: o.get_field("ebsPerformanceMaximumIops"),
            ebs_performance_maximum_throughput: o
                .get_field("ebsPerformanceMaximumThroughput"),
            efa_supported: o.get_field("efaSupported"),
            ena_support: o.get_field("enaSupport"),
            encryption_in_transit_supported: o.get_field("encryptionInTransitSupported"),
            fpgas: o.get_field("fpgas"),
            free_tier_eligible: o.get_field("freeTierEligible"),
            gpuses: o.get_field("gpuses"),
            hibernation_supported: o.get_field("hibernationSupported"),
            hypervisor: o.get_field("hypervisor"),
            id: o.get_field("id"),
            inference_accelerators: o.get_field("inferenceAccelerators"),
            instance_disks: o.get_field("instanceDisks"),
            instance_storage_supported: o.get_field("instanceStorageSupported"),
            instance_type: o.get_field("instanceType"),
            ipv6_supported: o.get_field("ipv6Supported"),
            maximum_ipv4_addresses_per_interface: o
                .get_field("maximumIpv4AddressesPerInterface"),
            maximum_ipv6_addresses_per_interface: o
                .get_field("maximumIpv6AddressesPerInterface"),
            maximum_network_cards: o.get_field("maximumNetworkCards"),
            maximum_network_interfaces: o.get_field("maximumNetworkInterfaces"),
            memory_size: o.get_field("memorySize"),
            network_performance: o.get_field("networkPerformance"),
            supported_architectures: o.get_field("supportedArchitectures"),
            supported_placement_strategies: o.get_field("supportedPlacementStrategies"),
            supported_root_device_types: o.get_field("supportedRootDeviceTypes"),
            supported_usages_classes: o.get_field("supportedUsagesClasses"),
            supported_virtualization_types: o.get_field("supportedVirtualizationTypes"),
            sustained_clock_speed: o.get_field("sustainedClockSpeed"),
            total_fpga_memory: o.get_field("totalFpgaMemory"),
            total_gpu_memory: o.get_field("totalGpuMemory"),
            total_instance_storage: o.get_field("totalInstanceStorage"),
            valid_cores: o.get_field("validCores"),
            valid_threads_per_cores: o.get_field("validThreadsPerCores"),
        }
    }
}
