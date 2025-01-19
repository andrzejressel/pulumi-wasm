pub mod get_instance_type {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceTypeArgs {
        /// Instance
        #[builder(into)]
        pub instance_type: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceTypeResult {
        /// `true` if auto recovery is supported.
        pub auto_recovery_supported: pulumi_wasm_rust::Output<bool>,
        /// `true` if it is a bare metal instance type.
        pub bare_metal: pulumi_wasm_rust::Output<bool>,
        /// `true` if the instance type is a burstable performance instance type.
        pub burstable_performance_supported: pulumi_wasm_rust::Output<bool>,
        /// `true`  if the instance type is a current generation.
        pub current_generation: pulumi_wasm_rust::Output<bool>,
        /// `true` if Dedicated Hosts are supported on the instance type.
        pub dedicated_hosts_supported: pulumi_wasm_rust::Output<bool>,
        /// Default number of cores for the instance type.
        pub default_cores: pulumi_wasm_rust::Output<i32>,
        /// The  default  number of threads per core for the instance type.
        pub default_threads_per_core: pulumi_wasm_rust::Output<i32>,
        /// Default number of vCPUs for the instance type.
        pub default_vcpus: pulumi_wasm_rust::Output<i32>,
        /// Indicates whether Amazon EBS encryption is supported.
        pub ebs_encryption_support: pulumi_wasm_rust::Output<String>,
        /// Whether non-volatile memory express (NVMe) is supported.
        pub ebs_nvme_support: pulumi_wasm_rust::Output<String>,
        /// Indicates that the instance type is Amazon EBS-optimized.
        pub ebs_optimized_support: pulumi_wasm_rust::Output<String>,
        /// The baseline bandwidth performance for an EBS-optimized instance type, in Mbps.
        pub ebs_performance_baseline_bandwidth: pulumi_wasm_rust::Output<i32>,
        /// The baseline input/output storage operations per seconds for an EBS-optimized instance type.
        pub ebs_performance_baseline_iops: pulumi_wasm_rust::Output<i32>,
        /// The baseline throughput performance for an EBS-optimized instance type, in MBps.
        pub ebs_performance_baseline_throughput: pulumi_wasm_rust::Output<f64>,
        /// The maximum bandwidth performance for an EBS-optimized instance type, in Mbps.
        pub ebs_performance_maximum_bandwidth: pulumi_wasm_rust::Output<i32>,
        /// The maximum input/output storage operations per second for an EBS-optimized instance type.
        pub ebs_performance_maximum_iops: pulumi_wasm_rust::Output<i32>,
        /// The maximum throughput performance for an EBS-optimized instance type, in MBps.
        pub ebs_performance_maximum_throughput: pulumi_wasm_rust::Output<f64>,
        /// Whether Elastic Fabric Adapter (EFA) is supported.
        pub efa_supported: pulumi_wasm_rust::Output<bool>,
        /// Whether Elastic Network Adapter (ENA) is supported.
        pub ena_support: pulumi_wasm_rust::Output<String>,
        /// Indicates whether encryption in-transit between instances is supported.
        pub encryption_in_transit_supported: pulumi_wasm_rust::Output<bool>,
        /// Describes the FPGA accelerator settings for the instance type.
        /// * `fpgas.#.count` - The count of FPGA accelerators for the instance type.
        /// * `fpgas.#.manufacturer` - The manufacturer of the FPGA accelerator.
        /// * `fpgas.#.memory_size` - The size (in MiB) for the memory available to the FPGA accelerator.
        /// * `fpgas.#.name` - The name of the FPGA accelerator.
        pub fpgas: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceTypeFpga>,
        >,
        /// `true` if the instance type is eligible for the free tier.
        pub free_tier_eligible: pulumi_wasm_rust::Output<bool>,
        /// Describes the GPU accelerators for the instance type.
        /// * `gpus.#.count` - The number of GPUs for the instance type.
        /// * `gpus.#.manufacturer` - The manufacturer of the GPU accelerator.
        /// * `gpus.#.memory_size` - The size (in MiB) for the memory available to the GPU accelerator.
        /// * `gpus.#.name` - The name of the GPU accelerator.
        pub gpuses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceTypeGpus>,
        >,
        /// `true` if On-Demand hibernation is supported.
        pub hibernation_supported: pulumi_wasm_rust::Output<bool>,
        /// Hypervisor used for the instance type.
        pub hypervisor: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Describes the Inference accelerators for the instance type.
        /// * `inference_accelerators.#.count` - The number of Inference accelerators for the instance type.
        /// * `inference_accelerators.#.manufacturer` - The manufacturer of the Inference accelerator.
        /// * `inference_accelerators.#.name` - The name of the Inference accelerator.
        pub inference_accelerators: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceTypeInferenceAccelerator>,
        >,
        /// Describes the disks for the instance type.
        /// * `instance_disks.#.count` - The number of disks with this configuration.
        /// * `instance_disks.#.size` - The size of the disk in GB.
        /// * `instance_disks.#.type` - The type of disk.
        pub instance_disks: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceTypeInstanceDisk>,
        >,
        /// `true` if instance storage is supported.
        pub instance_storage_supported: pulumi_wasm_rust::Output<bool>,
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// `true` if IPv6 is supported.
        pub ipv6_supported: pulumi_wasm_rust::Output<bool>,
        /// The maximum number of IPv4 addresses per network interface.
        pub maximum_ipv4_addresses_per_interface: pulumi_wasm_rust::Output<i32>,
        /// The maximum number of IPv6 addresses per network interface.
        pub maximum_ipv6_addresses_per_interface: pulumi_wasm_rust::Output<i32>,
        /// The maximum number of physical network cards that can be allocated to the instance.
        pub maximum_network_cards: pulumi_wasm_rust::Output<i32>,
        /// The maximum number of network interfaces for the instance type.
        pub maximum_network_interfaces: pulumi_wasm_rust::Output<i32>,
        /// Size of the instance memory, in MiB.
        pub memory_size: pulumi_wasm_rust::Output<i32>,
        /// Describes the network performance.
        pub network_performance: pulumi_wasm_rust::Output<String>,
        /// A list of architectures supported by the instance type.
        pub supported_architectures: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of supported placement groups types.
        pub supported_placement_strategies: pulumi_wasm_rust::Output<Vec<String>>,
        /// Indicates the supported root device types.
        pub supported_root_device_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// Indicates whether the instance type is offered for spot or On-Demand.
        pub supported_usages_classes: pulumi_wasm_rust::Output<Vec<String>>,
        /// The supported virtualization types.
        pub supported_virtualization_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// The speed of the processor, in GHz.
        pub sustained_clock_speed: pulumi_wasm_rust::Output<f64>,
        /// Total memory of all FPGA accelerators for the instance type (in MiB).
        pub total_fpga_memory: pulumi_wasm_rust::Output<i32>,
        /// Total size of the memory for the GPU accelerators for the instance type (in MiB).
        pub total_gpu_memory: pulumi_wasm_rust::Output<i32>,
        /// The total size of the instance disks, in GB.
        pub total_instance_storage: pulumi_wasm_rust::Output<i32>,
        /// List of the valid number of cores that can be configured for the instance type.
        pub valid_cores: pulumi_wasm_rust::Output<Vec<i32>>,
        /// List of the valid number of threads per core that can be configured for the instance type.
        pub valid_threads_per_cores: pulumi_wasm_rust::Output<Vec<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetInstanceTypeArgs) -> GetInstanceTypeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_type_binding = args.instance_type.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getInstanceType:getInstanceType".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoRecoverySupported".into(),
                },
                register_interface::ResultField {
                    name: "bareMetal".into(),
                },
                register_interface::ResultField {
                    name: "burstablePerformanceSupported".into(),
                },
                register_interface::ResultField {
                    name: "currentGeneration".into(),
                },
                register_interface::ResultField {
                    name: "dedicatedHostsSupported".into(),
                },
                register_interface::ResultField {
                    name: "defaultCores".into(),
                },
                register_interface::ResultField {
                    name: "defaultThreadsPerCore".into(),
                },
                register_interface::ResultField {
                    name: "defaultVcpus".into(),
                },
                register_interface::ResultField {
                    name: "ebsEncryptionSupport".into(),
                },
                register_interface::ResultField {
                    name: "ebsNvmeSupport".into(),
                },
                register_interface::ResultField {
                    name: "ebsOptimizedSupport".into(),
                },
                register_interface::ResultField {
                    name: "ebsPerformanceBaselineBandwidth".into(),
                },
                register_interface::ResultField {
                    name: "ebsPerformanceBaselineIops".into(),
                },
                register_interface::ResultField {
                    name: "ebsPerformanceBaselineThroughput".into(),
                },
                register_interface::ResultField {
                    name: "ebsPerformanceMaximumBandwidth".into(),
                },
                register_interface::ResultField {
                    name: "ebsPerformanceMaximumIops".into(),
                },
                register_interface::ResultField {
                    name: "ebsPerformanceMaximumThroughput".into(),
                },
                register_interface::ResultField {
                    name: "efaSupported".into(),
                },
                register_interface::ResultField {
                    name: "enaSupport".into(),
                },
                register_interface::ResultField {
                    name: "encryptionInTransitSupported".into(),
                },
                register_interface::ResultField {
                    name: "fpgas".into(),
                },
                register_interface::ResultField {
                    name: "freeTierEligible".into(),
                },
                register_interface::ResultField {
                    name: "gpuses".into(),
                },
                register_interface::ResultField {
                    name: "hibernationSupported".into(),
                },
                register_interface::ResultField {
                    name: "hypervisor".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "inferenceAccelerators".into(),
                },
                register_interface::ResultField {
                    name: "instanceDisks".into(),
                },
                register_interface::ResultField {
                    name: "instanceStorageSupported".into(),
                },
                register_interface::ResultField {
                    name: "instanceType".into(),
                },
                register_interface::ResultField {
                    name: "ipv6Supported".into(),
                },
                register_interface::ResultField {
                    name: "maximumIpv4AddressesPerInterface".into(),
                },
                register_interface::ResultField {
                    name: "maximumIpv6AddressesPerInterface".into(),
                },
                register_interface::ResultField {
                    name: "maximumNetworkCards".into(),
                },
                register_interface::ResultField {
                    name: "maximumNetworkInterfaces".into(),
                },
                register_interface::ResultField {
                    name: "memorySize".into(),
                },
                register_interface::ResultField {
                    name: "networkPerformance".into(),
                },
                register_interface::ResultField {
                    name: "supportedArchitectures".into(),
                },
                register_interface::ResultField {
                    name: "supportedPlacementStrategies".into(),
                },
                register_interface::ResultField {
                    name: "supportedRootDeviceTypes".into(),
                },
                register_interface::ResultField {
                    name: "supportedUsagesClasses".into(),
                },
                register_interface::ResultField {
                    name: "supportedVirtualizationTypes".into(),
                },
                register_interface::ResultField {
                    name: "sustainedClockSpeed".into(),
                },
                register_interface::ResultField {
                    name: "totalFpgaMemory".into(),
                },
                register_interface::ResultField {
                    name: "totalGpuMemory".into(),
                },
                register_interface::ResultField {
                    name: "totalInstanceStorage".into(),
                },
                register_interface::ResultField {
                    name: "validCores".into(),
                },
                register_interface::ResultField {
                    name: "validThreadsPerCores".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetInstanceTypeResult {
            auto_recovery_supported: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoRecoverySupported").unwrap(),
            ),
            bare_metal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bareMetal").unwrap(),
            ),
            burstable_performance_supported: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("burstablePerformanceSupported").unwrap(),
            ),
            current_generation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("currentGeneration").unwrap(),
            ),
            dedicated_hosts_supported: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dedicatedHostsSupported").unwrap(),
            ),
            default_cores: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultCores").unwrap(),
            ),
            default_threads_per_core: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultThreadsPerCore").unwrap(),
            ),
            default_vcpus: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultVcpus").unwrap(),
            ),
            ebs_encryption_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsEncryptionSupport").unwrap(),
            ),
            ebs_nvme_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsNvmeSupport").unwrap(),
            ),
            ebs_optimized_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsOptimizedSupport").unwrap(),
            ),
            ebs_performance_baseline_bandwidth: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsPerformanceBaselineBandwidth").unwrap(),
            ),
            ebs_performance_baseline_iops: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsPerformanceBaselineIops").unwrap(),
            ),
            ebs_performance_baseline_throughput: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsPerformanceBaselineThroughput").unwrap(),
            ),
            ebs_performance_maximum_bandwidth: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsPerformanceMaximumBandwidth").unwrap(),
            ),
            ebs_performance_maximum_iops: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsPerformanceMaximumIops").unwrap(),
            ),
            ebs_performance_maximum_throughput: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsPerformanceMaximumThroughput").unwrap(),
            ),
            efa_supported: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("efaSupported").unwrap(),
            ),
            ena_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enaSupport").unwrap(),
            ),
            encryption_in_transit_supported: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionInTransitSupported").unwrap(),
            ),
            fpgas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fpgas").unwrap(),
            ),
            free_tier_eligible: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("freeTierEligible").unwrap(),
            ),
            gpuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gpuses").unwrap(),
            ),
            hibernation_supported: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hibernationSupported").unwrap(),
            ),
            hypervisor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hypervisor").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            inference_accelerators: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inferenceAccelerators").unwrap(),
            ),
            instance_disks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceDisks").unwrap(),
            ),
            instance_storage_supported: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceStorageSupported").unwrap(),
            ),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceType").unwrap(),
            ),
            ipv6_supported: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6Supported").unwrap(),
            ),
            maximum_ipv4_addresses_per_interface: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maximumIpv4AddressesPerInterface").unwrap(),
            ),
            maximum_ipv6_addresses_per_interface: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maximumIpv6AddressesPerInterface").unwrap(),
            ),
            maximum_network_cards: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maximumNetworkCards").unwrap(),
            ),
            maximum_network_interfaces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maximumNetworkInterfaces").unwrap(),
            ),
            memory_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memorySize").unwrap(),
            ),
            network_performance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkPerformance").unwrap(),
            ),
            supported_architectures: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedArchitectures").unwrap(),
            ),
            supported_placement_strategies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedPlacementStrategies").unwrap(),
            ),
            supported_root_device_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedRootDeviceTypes").unwrap(),
            ),
            supported_usages_classes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedUsagesClasses").unwrap(),
            ),
            supported_virtualization_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedVirtualizationTypes").unwrap(),
            ),
            sustained_clock_speed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sustainedClockSpeed").unwrap(),
            ),
            total_fpga_memory: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("totalFpgaMemory").unwrap(),
            ),
            total_gpu_memory: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("totalGpuMemory").unwrap(),
            ),
            total_instance_storage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("totalInstanceStorage").unwrap(),
            ),
            valid_cores: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validCores").unwrap(),
            ),
            valid_threads_per_cores: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validThreadsPerCores").unwrap(),
            ),
        }
    }
}
