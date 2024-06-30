use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use bindings::exports::pulumi::docker::container;
use bindings::exports::pulumi::docker::image;
use bindings::exports::pulumi::docker::network;
use bindings::exports::pulumi::docker::plugin;
use bindings::exports::pulumi::docker::registry_image;
use bindings::exports::pulumi::docker::remote_image;
use bindings::exports::pulumi::docker::secret;
use bindings::exports::pulumi::docker::service;
use bindings::exports::pulumi::docker::service_config;
use bindings::exports::pulumi::docker::tag;
use bindings::exports::pulumi::docker::volume;

mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}

impl container::Guest for Component {
    fn invoke(name: String, args: container::Args) -> container::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/container:Container".into(),
            name,
            object: vec![
                ObjectField { name: "attach".into(), value: args.attach },
                ObjectField { name: "capabilities".into(), value: args.capabilities },
                ObjectField { name: "cgroupnsMode".into(), value: args.cgroupns_mode },
                ObjectField { name: "command".into(), value: args.command },
                ObjectField { name: "containerReadRefreshTimeoutMilliseconds".into(), value: args.container_read_refresh_timeout_milliseconds },
                ObjectField { name: "cpuSet".into(), value: args.cpu_set },
                ObjectField { name: "cpuShares".into(), value: args.cpu_shares },
                ObjectField { name: "destroyGraceSeconds".into(), value: args.destroy_grace_seconds },
                ObjectField { name: "devices".into(), value: args.devices },
                ObjectField { name: "dns".into(), value: args.dns },
                ObjectField { name: "dnsOpts".into(), value: args.dns_opts },
                ObjectField { name: "dnsSearches".into(), value: args.dns_searches },
                ObjectField { name: "domainname".into(), value: args.domainname },
                ObjectField { name: "entrypoints".into(), value: args.entrypoints },
                ObjectField { name: "envs".into(), value: args.envs },
                ObjectField { name: "gpus".into(), value: args.gpus },
                ObjectField { name: "groupAdds".into(), value: args.group_adds },
                ObjectField { name: "healthcheck".into(), value: args.healthcheck },
                ObjectField { name: "hostname".into(), value: args.hostname },
                ObjectField { name: "hosts".into(), value: args.hosts },
                ObjectField { name: "image".into(), value: args.image },
                ObjectField { name: "init".into(), value: args.init },
                ObjectField { name: "ipcMode".into(), value: args.ipc_mode },
                ObjectField { name: "labels".into(), value: args.labels },
                ObjectField { name: "logDriver".into(), value: args.log_driver },
                ObjectField { name: "logOpts".into(), value: args.log_opts },
                ObjectField { name: "logs".into(), value: args.logs },
                ObjectField { name: "maxRetryCount".into(), value: args.max_retry_count },
                ObjectField { name: "memory".into(), value: args.memory },
                ObjectField { name: "memorySwap".into(), value: args.memory_swap },
                ObjectField { name: "mounts".into(), value: args.mounts },
                ObjectField { name: "mustRun".into(), value: args.must_run },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "networkMode".into(), value: args.network_mode },
                ObjectField { name: "networksAdvanced".into(), value: args.networks_advanced },
                ObjectField { name: "pidMode".into(), value: args.pid_mode },
                ObjectField { name: "ports".into(), value: args.ports },
                ObjectField { name: "privileged".into(), value: args.privileged },
                ObjectField { name: "publishAllPorts".into(), value: args.publish_all_ports },
                ObjectField { name: "readOnly".into(), value: args.read_only },
                ObjectField { name: "removeVolumes".into(), value: args.remove_volumes },
                ObjectField { name: "restart".into(), value: args.restart },
                ObjectField { name: "rm".into(), value: args.rm },
                ObjectField { name: "runtime".into(), value: args.runtime },
                ObjectField { name: "securityOpts".into(), value: args.security_opts },
                ObjectField { name: "shmSize".into(), value: args.shm_size },
                ObjectField { name: "start".into(), value: args.start },
                ObjectField { name: "stdinOpen".into(), value: args.stdin_open },
                ObjectField { name: "stopSignal".into(), value: args.stop_signal },
                ObjectField { name: "stopTimeout".into(), value: args.stop_timeout },
                ObjectField { name: "storageOpts".into(), value: args.storage_opts },
                ObjectField { name: "sysctls".into(), value: args.sysctls },
                ObjectField { name: "tmpfs".into(), value: args.tmpfs },
                ObjectField { name: "tty".into(), value: args.tty },
                ObjectField { name: "ulimits".into(), value: args.ulimits },
                ObjectField { name: "uploads".into(), value: args.uploads },
                ObjectField { name: "user".into(), value: args.user },
                ObjectField { name: "usernsMode".into(), value: args.userns_mode },
                ObjectField { name: "volumes".into(), value: args.volumes },
                ObjectField { name: "wait".into(), value: args.wait },
                ObjectField { name: "waitTimeout".into(), value: args.wait_timeout },
                ObjectField { name: "workingDir".into(), value: args.working_dir },
            ],
            results: vec![
                ResultField { name: "attach".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "bridge".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "capabilities".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 97, 100, 100, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 165, 100, 114, 111, 112, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "cgroupnsMode".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "command".into(), schema: vec![129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "containerLogs".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "containerReadRefreshTimeoutMilliseconds".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116] },
                ResultField { name: "cpuSet".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "cpuShares".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116] },
                ResultField { name: "destroyGraceSeconds".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116] },
                ResultField { name: "devices".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 131, 173, 99, 111, 110, 116, 97, 105, 110, 101, 114, 80, 97, 116, 104, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 104, 111, 115, 116, 80, 97, 116, 104, 166, 83, 116, 114, 105, 110, 103, 171, 112, 101, 114, 109, 105, 115, 115, 105, 111, 110, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "dns".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "dnsOpts".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "dnsSearches".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "domainname".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "entrypoints".into(), schema: vec![129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "envs".into(), schema: vec![129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "exitCode".into(), schema: vec![163, 73, 110, 116] },
                ResultField { name: "gpus".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "groupAdds".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "healthcheck".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 133, 168, 105, 110, 116, 101, 114, 118, 97, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 114, 101, 116, 114, 105, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 171, 115, 116, 97, 114, 116, 80, 101, 114, 105, 111, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 116, 101, 115, 116, 115, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 167, 116, 105, 109, 101, 111, 117, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "hostname".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "hosts".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 104, 111, 115, 116, 166, 83, 116, 114, 105, 110, 103, 162, 105, 112, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "image".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "init".into(), schema: vec![164, 66, 111, 111, 108] },
                ResultField { name: "ipcMode".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "labels".into(), schema: vec![129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 165, 108, 97, 98, 101, 108, 166, 83, 116, 114, 105, 110, 103, 165, 118, 97, 108, 117, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "logDriver".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "logOpts".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "logs".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "maxRetryCount".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116] },
                ResultField { name: "memory".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116] },
                ResultField { name: "memorySwap".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116] },
                ResultField { name: "mounts".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 135, 171, 98, 105, 110, 100, 79, 112, 116, 105, 111, 110, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 129, 171, 112, 114, 111, 112, 97, 103, 97, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 114, 101, 97, 100, 79, 110, 108, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 166, 115, 111, 117, 114, 99, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166, 116, 97, 114, 103, 101, 116, 166, 83, 116, 114, 105, 110, 103, 172, 116, 109, 112, 102, 115, 79, 112, 116, 105, 111, 110, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 109, 111, 100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 169, 115, 105, 122, 101, 66, 121, 116, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 164, 116, 121, 112, 101, 166, 83, 116, 114, 105, 110, 103, 173, 118, 111, 108, 117, 109, 101, 79, 112, 116, 105, 111, 110, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 132, 170, 100, 114, 105, 118, 101, 114, 78, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173, 100, 114, 105, 118, 101, 114, 79, 112, 116, 105, 111, 110, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103, 166, 108, 97, 98, 101, 108, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 165, 108, 97, 98, 101, 108, 166, 83, 116, 114, 105, 110, 103, 165, 118, 97, 108, 117, 101, 166, 83, 116, 114, 105, 110, 103, 166, 110, 111, 67, 111, 112, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "mustRun".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "name".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "networkDatas".into(), schema: vec![129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 136, 167, 103, 97, 116, 101, 119, 97, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 177, 103, 108, 111, 98, 97, 108, 73, 112, 118, 54, 65, 100, 100, 114, 101, 115, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 182, 103, 108, 111, 98, 97, 108, 73, 112, 118, 54, 80, 114, 101, 102, 105, 120, 76, 101, 110, 103, 116, 104, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 169, 105, 112, 65, 100, 100, 114, 101, 115, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 105, 112, 80, 114, 101, 102, 105, 120, 76, 101, 110, 103, 116, 104, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 171, 105, 112, 118, 54, 71, 97, 116, 101, 119, 97, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 109, 97, 99, 65, 100, 100, 114, 101, 115, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 171, 110, 101, 116, 119, 111, 114, 107, 78, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "networkMode".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "networksAdvanced".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 132, 167, 97, 108, 105, 97, 115, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 171, 105, 112, 118, 52, 65, 100, 100, 114, 101, 115, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 171, 105, 112, 118, 54, 65, 100, 100, 114, 101, 115, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 110, 97, 109, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "pidMode".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "ports".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 132, 168, 101, 120, 116, 101, 114, 110, 97, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 168, 105, 110, 116, 101, 114, 110, 97, 108, 163, 73, 110, 116, 162, 105, 112, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 112, 114, 111, 116, 111, 99, 111, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "privileged".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "publishAllPorts".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "readOnly".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "removeVolumes".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "restart".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "rm".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "runtime".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "securityOpts".into(), schema: vec![129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "shmSize".into(), schema: vec![163, 73, 110, 116] },
                ResultField { name: "start".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "stdinOpen".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "stopSignal".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "stopTimeout".into(), schema: vec![163, 73, 110, 116] },
                ResultField { name: "storageOpts".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "sysctls".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "tmpfs".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "tty".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "ulimits".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 131, 164, 104, 97, 114, 100, 163, 73, 110, 116, 164, 110, 97, 109, 101, 166, 83, 116, 114, 105, 110, 103, 164, 115, 111, 102, 116, 163, 73, 110, 116] },
                ResultField { name: "uploads".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 134, 167, 99, 111, 110, 116, 101, 110, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173, 99, 111, 110, 116, 101, 110, 116, 66, 97, 115, 101, 54, 52, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 164, 102, 105, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166, 115, 111, 117, 114, 99, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 115, 111, 117, 114, 99, 101, 72, 97, 115, 104, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "user".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "usernsMode".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "volumes".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 133, 173, 99, 111, 110, 116, 97, 105, 110, 101, 114, 80, 97, 116, 104, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173, 102, 114, 111, 109, 67, 111, 110, 116, 97, 105, 110, 101, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 104, 111, 115, 116, 80, 97, 116, 104, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 114, 101, 97, 100, 79, 110, 108, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 170, 118, 111, 108, 117, 109, 101, 78, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "wait".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "waitTimeout".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116] },
                ResultField { name: "workingDir".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        container::Res {
            attach: o.fields.iter().find(|o| o.name == "attach").unwrap().output.duplicate(),
            bridge: o.fields.iter().find(|o| o.name == "bridge").unwrap().output.duplicate(),
            capabilities: o.fields.iter().find(|o| o.name == "capabilities").unwrap().output.duplicate(),
            cgroupns_mode: o.fields.iter().find(|o| o.name == "cgroupnsMode").unwrap().output.duplicate(),
            command: o.fields.iter().find(|o| o.name == "command").unwrap().output.duplicate(),
            container_logs: o.fields.iter().find(|o| o.name == "containerLogs").unwrap().output.duplicate(),
            container_read_refresh_timeout_milliseconds: o.fields.iter().find(|o| o.name == "containerReadRefreshTimeoutMilliseconds").unwrap().output.duplicate(),
            cpu_set: o.fields.iter().find(|o| o.name == "cpuSet").unwrap().output.duplicate(),
            cpu_shares: o.fields.iter().find(|o| o.name == "cpuShares").unwrap().output.duplicate(),
            destroy_grace_seconds: o.fields.iter().find(|o| o.name == "destroyGraceSeconds").unwrap().output.duplicate(),
            devices: o.fields.iter().find(|o| o.name == "devices").unwrap().output.duplicate(),
            dns: o.fields.iter().find(|o| o.name == "dns").unwrap().output.duplicate(),
            dns_opts: o.fields.iter().find(|o| o.name == "dnsOpts").unwrap().output.duplicate(),
            dns_searches: o.fields.iter().find(|o| o.name == "dnsSearches").unwrap().output.duplicate(),
            domainname: o.fields.iter().find(|o| o.name == "domainname").unwrap().output.duplicate(),
            entrypoints: o.fields.iter().find(|o| o.name == "entrypoints").unwrap().output.duplicate(),
            envs: o.fields.iter().find(|o| o.name == "envs").unwrap().output.duplicate(),
            exit_code: o.fields.iter().find(|o| o.name == "exitCode").unwrap().output.duplicate(),
            gpus: o.fields.iter().find(|o| o.name == "gpus").unwrap().output.duplicate(),
            group_adds: o.fields.iter().find(|o| o.name == "groupAdds").unwrap().output.duplicate(),
            healthcheck: o.fields.iter().find(|o| o.name == "healthcheck").unwrap().output.duplicate(),
            hostname: o.fields.iter().find(|o| o.name == "hostname").unwrap().output.duplicate(),
            hosts: o.fields.iter().find(|o| o.name == "hosts").unwrap().output.duplicate(),
            image: o.fields.iter().find(|o| o.name == "image").unwrap().output.duplicate(),
            init: o.fields.iter().find(|o| o.name == "init").unwrap().output.duplicate(),
            ipc_mode: o.fields.iter().find(|o| o.name == "ipcMode").unwrap().output.duplicate(),
            labels: o.fields.iter().find(|o| o.name == "labels").unwrap().output.duplicate(),
            log_driver: o.fields.iter().find(|o| o.name == "logDriver").unwrap().output.duplicate(),
            log_opts: o.fields.iter().find(|o| o.name == "logOpts").unwrap().output.duplicate(),
            logs: o.fields.iter().find(|o| o.name == "logs").unwrap().output.duplicate(),
            max_retry_count: o.fields.iter().find(|o| o.name == "maxRetryCount").unwrap().output.duplicate(),
            memory: o.fields.iter().find(|o| o.name == "memory").unwrap().output.duplicate(),
            memory_swap: o.fields.iter().find(|o| o.name == "memorySwap").unwrap().output.duplicate(),
            mounts: o.fields.iter().find(|o| o.name == "mounts").unwrap().output.duplicate(),
            must_run: o.fields.iter().find(|o| o.name == "mustRun").unwrap().output.duplicate(),
            name: o.fields.iter().find(|o| o.name == "name").unwrap().output.duplicate(),
            network_datas: o.fields.iter().find(|o| o.name == "networkDatas").unwrap().output.duplicate(),
            network_mode: o.fields.iter().find(|o| o.name == "networkMode").unwrap().output.duplicate(),
            networks_advanced: o.fields.iter().find(|o| o.name == "networksAdvanced").unwrap().output.duplicate(),
            pid_mode: o.fields.iter().find(|o| o.name == "pidMode").unwrap().output.duplicate(),
            ports: o.fields.iter().find(|o| o.name == "ports").unwrap().output.duplicate(),
            privileged: o.fields.iter().find(|o| o.name == "privileged").unwrap().output.duplicate(),
            publish_all_ports: o.fields.iter().find(|o| o.name == "publishAllPorts").unwrap().output.duplicate(),
            read_only: o.fields.iter().find(|o| o.name == "readOnly").unwrap().output.duplicate(),
            remove_volumes: o.fields.iter().find(|o| o.name == "removeVolumes").unwrap().output.duplicate(),
            restart: o.fields.iter().find(|o| o.name == "restart").unwrap().output.duplicate(),
            rm: o.fields.iter().find(|o| o.name == "rm").unwrap().output.duplicate(),
            runtime: o.fields.iter().find(|o| o.name == "runtime").unwrap().output.duplicate(),
            security_opts: o.fields.iter().find(|o| o.name == "securityOpts").unwrap().output.duplicate(),
            shm_size: o.fields.iter().find(|o| o.name == "shmSize").unwrap().output.duplicate(),
            start: o.fields.iter().find(|o| o.name == "start").unwrap().output.duplicate(),
            stdin_open: o.fields.iter().find(|o| o.name == "stdinOpen").unwrap().output.duplicate(),
            stop_signal: o.fields.iter().find(|o| o.name == "stopSignal").unwrap().output.duplicate(),
            stop_timeout: o.fields.iter().find(|o| o.name == "stopTimeout").unwrap().output.duplicate(),
            storage_opts: o.fields.iter().find(|o| o.name == "storageOpts").unwrap().output.duplicate(),
            sysctls: o.fields.iter().find(|o| o.name == "sysctls").unwrap().output.duplicate(),
            tmpfs: o.fields.iter().find(|o| o.name == "tmpfs").unwrap().output.duplicate(),
            tty: o.fields.iter().find(|o| o.name == "tty").unwrap().output.duplicate(),
            ulimits: o.fields.iter().find(|o| o.name == "ulimits").unwrap().output.duplicate(),
            uploads: o.fields.iter().find(|o| o.name == "uploads").unwrap().output.duplicate(),
            user: o.fields.iter().find(|o| o.name == "user").unwrap().output.duplicate(),
            userns_mode: o.fields.iter().find(|o| o.name == "usernsMode").unwrap().output.duplicate(),
            volumes: o.fields.iter().find(|o| o.name == "volumes").unwrap().output.duplicate(),
            wait: o.fields.iter().find(|o| o.name == "wait").unwrap().output.duplicate(),
            wait_timeout: o.fields.iter().find(|o| o.name == "waitTimeout").unwrap().output.duplicate(),
            working_dir: o.fields.iter().find(|o| o.name == "workingDir").unwrap().output.duplicate(),
        }

    }
}
impl image::Guest for Component {
    fn invoke(name: String, args: image::Args) -> image::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/image:Image".into(),
            name,
            object: vec![
                ObjectField { name: "build".into(), value: args.build },
                ObjectField { name: "buildOnPreview".into(), value: args.build_on_preview },
                ObjectField { name: "imageName".into(), value: args.image_name },
                ObjectField { name: "registry".into(), value: args.registry },
                ObjectField { name: "skipPush".into(), value: args.skip_push },
            ],
            results: vec![
                ResultField { name: "baseImageName".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "context".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "dockerfile".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "imageName".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "platform".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "registryServer".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "repoDigest".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        image::Res {
            base_image_name: o.fields.iter().find(|o| o.name == "baseImageName").unwrap().output.duplicate(),
            context: o.fields.iter().find(|o| o.name == "context").unwrap().output.duplicate(),
            dockerfile: o.fields.iter().find(|o| o.name == "dockerfile").unwrap().output.duplicate(),
            image_name: o.fields.iter().find(|o| o.name == "imageName").unwrap().output.duplicate(),
            platform: o.fields.iter().find(|o| o.name == "platform").unwrap().output.duplicate(),
            registry_server: o.fields.iter().find(|o| o.name == "registryServer").unwrap().output.duplicate(),
            repo_digest: o.fields.iter().find(|o| o.name == "repoDigest").unwrap().output.duplicate(),
        }

    }
}
impl network::Guest for Component {
    fn invoke(name: String, args: network::Args) -> network::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/network:Network".into(),
            name,
            object: vec![
                ObjectField { name: "attachable".into(), value: args.attachable },
                ObjectField { name: "checkDuplicate".into(), value: args.check_duplicate },
                ObjectField { name: "driver".into(), value: args.driver },
                ObjectField { name: "ingress".into(), value: args.ingress },
                ObjectField { name: "internal".into(), value: args.internal },
                ObjectField { name: "ipamConfigs".into(), value: args.ipam_configs },
                ObjectField { name: "ipamDriver".into(), value: args.ipam_driver },
                ObjectField { name: "ipamOptions".into(), value: args.ipam_options },
                ObjectField { name: "ipv6".into(), value: args.ipv6 },
                ObjectField { name: "labels".into(), value: args.labels },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "options".into(), value: args.options },
            ],
            results: vec![
                ResultField { name: "attachable".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "checkDuplicate".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "driver".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "ingress".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "internal".into(), schema: vec![164, 66, 111, 111, 108] },
                ResultField { name: "ipamConfigs".into(), schema: vec![129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 132, 170, 97, 117, 120, 65, 100, 100, 114, 101, 115, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103, 167, 103, 97, 116, 101, 119, 97, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 105, 112, 82, 97, 110, 103, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166, 115, 117, 98, 110, 101, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "ipamDriver".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "ipamOptions".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "ipv6".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "labels".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 165, 108, 97, 98, 101, 108, 166, 83, 116, 114, 105, 110, 103, 165, 118, 97, 108, 117, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "name".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "options".into(), schema: vec![129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "scope".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        network::Res {
            attachable: o.fields.iter().find(|o| o.name == "attachable").unwrap().output.duplicate(),
            check_duplicate: o.fields.iter().find(|o| o.name == "checkDuplicate").unwrap().output.duplicate(),
            driver: o.fields.iter().find(|o| o.name == "driver").unwrap().output.duplicate(),
            ingress: o.fields.iter().find(|o| o.name == "ingress").unwrap().output.duplicate(),
            internal: o.fields.iter().find(|o| o.name == "internal").unwrap().output.duplicate(),
            ipam_configs: o.fields.iter().find(|o| o.name == "ipamConfigs").unwrap().output.duplicate(),
            ipam_driver: o.fields.iter().find(|o| o.name == "ipamDriver").unwrap().output.duplicate(),
            ipam_options: o.fields.iter().find(|o| o.name == "ipamOptions").unwrap().output.duplicate(),
            ipv6: o.fields.iter().find(|o| o.name == "ipv6").unwrap().output.duplicate(),
            labels: o.fields.iter().find(|o| o.name == "labels").unwrap().output.duplicate(),
            name: o.fields.iter().find(|o| o.name == "name").unwrap().output.duplicate(),
            options: o.fields.iter().find(|o| o.name == "options").unwrap().output.duplicate(),
            scope: o.fields.iter().find(|o| o.name == "scope").unwrap().output.duplicate(),
        }

    }
}
impl plugin::Guest for Component {
    fn invoke(name: String, args: plugin::Args) -> plugin::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/plugin:Plugin".into(),
            name,
            object: vec![
                ObjectField { name: "alias".into(), value: args.alias },
                ObjectField { name: "enableTimeout".into(), value: args.enable_timeout },
                ObjectField { name: "enabled".into(), value: args.enabled },
                ObjectField { name: "envs".into(), value: args.envs },
                ObjectField { name: "forceDestroy".into(), value: args.force_destroy },
                ObjectField { name: "forceDisable".into(), value: args.force_disable },
                ObjectField { name: "grantAllPermissions".into(), value: args.grant_all_permissions },
                ObjectField { name: "grantPermissions".into(), value: args.grant_permissions },
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "alias".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "enableTimeout".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116] },
                ResultField { name: "enabled".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "envs".into(), schema: vec![129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "forceDestroy".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "forceDisable".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "grantAllPermissions".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "grantPermissions".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 110, 97, 109, 101, 166, 83, 116, 114, 105, 110, 103, 166, 118, 97, 108, 117, 101, 115, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "name".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "pluginReference".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        plugin::Res {
            alias: o.fields.iter().find(|o| o.name == "alias").unwrap().output.duplicate(),
            enable_timeout: o.fields.iter().find(|o| o.name == "enableTimeout").unwrap().output.duplicate(),
            enabled: o.fields.iter().find(|o| o.name == "enabled").unwrap().output.duplicate(),
            envs: o.fields.iter().find(|o| o.name == "envs").unwrap().output.duplicate(),
            force_destroy: o.fields.iter().find(|o| o.name == "forceDestroy").unwrap().output.duplicate(),
            force_disable: o.fields.iter().find(|o| o.name == "forceDisable").unwrap().output.duplicate(),
            grant_all_permissions: o.fields.iter().find(|o| o.name == "grantAllPermissions").unwrap().output.duplicate(),
            grant_permissions: o.fields.iter().find(|o| o.name == "grantPermissions").unwrap().output.duplicate(),
            name: o.fields.iter().find(|o| o.name == "name").unwrap().output.duplicate(),
            plugin_reference: o.fields.iter().find(|o| o.name == "pluginReference").unwrap().output.duplicate(),
        }

    }
}
impl registry_image::Guest for Component {
    fn invoke(name: String, args: registry_image::Args) -> registry_image::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/registryImage:RegistryImage".into(),
            name,
            object: vec![
                ObjectField { name: "insecureSkipVerify".into(), value: args.insecure_skip_verify },
                ObjectField { name: "keepRemotely".into(), value: args.keep_remotely },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "triggers".into(), value: args.triggers },
            ],
            results: vec![
                ResultField { name: "insecureSkipVerify".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "keepRemotely".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "name".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "sha256Digest".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "triggers".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        registry_image::Res {
            insecure_skip_verify: o.fields.iter().find(|o| o.name == "insecureSkipVerify").unwrap().output.duplicate(),
            keep_remotely: o.fields.iter().find(|o| o.name == "keepRemotely").unwrap().output.duplicate(),
            name: o.fields.iter().find(|o| o.name == "name").unwrap().output.duplicate(),
            sha256_digest: o.fields.iter().find(|o| o.name == "sha256Digest").unwrap().output.duplicate(),
            triggers: o.fields.iter().find(|o| o.name == "triggers").unwrap().output.duplicate(),
        }

    }
}
impl remote_image::Guest for Component {
    fn invoke(name: String, args: remote_image::Args) -> remote_image::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/remoteImage:RemoteImage".into(),
            name,
            object: vec![
                ObjectField { name: "build".into(), value: args.build },
                ObjectField { name: "forceRemove".into(), value: args.force_remove },
                ObjectField { name: "keepLocally".into(), value: args.keep_locally },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "platform".into(), value: args.platform },
                ObjectField { name: "pullTriggers".into(), value: args.pull_triggers },
                ObjectField { name: "triggers".into(), value: args.triggers },
            ],
            results: vec![
                ResultField { name: "build".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 222, 0, 35, 171, 97, 117, 116, 104, 67, 111, 110, 102, 105, 103, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 136, 164, 97, 117, 116, 104, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 101, 109, 97, 105, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 104, 111, 115, 116, 78, 97, 109, 101, 166, 83, 116, 114, 105, 110, 103, 173, 105, 100, 101, 110, 116, 105, 116, 121, 84, 111, 107, 101, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 112, 97, 115, 115, 119, 111, 114, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173, 114, 101, 103, 105, 115, 116, 114, 121, 84, 111, 107, 101, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173, 115, 101, 114, 118, 101, 114, 65, 100, 100, 114, 101, 115, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 117, 115, 101, 114, 78, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 98, 117, 105, 108, 100, 65, 114, 103, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103, 169, 98, 117, 105, 108, 100, 65, 114, 103, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103, 167, 98, 117, 105, 108, 100, 73, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 99, 97, 99, 104, 101, 70, 114, 111, 109, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 172, 99, 103, 114, 111, 117, 112, 80, 97, 114, 101, 110, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 99, 111, 110, 116, 101, 120, 116, 166, 83, 116, 114, 105, 110, 103, 169, 99, 112, 117, 80, 101, 114, 105, 111, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 168, 99, 112, 117, 81, 117, 111, 116, 97, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 170, 99, 112, 117, 83, 101, 116, 67, 112, 117, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 99, 112, 117, 83, 101, 116, 77, 101, 109, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 169, 99, 112, 117, 83, 104, 97, 114, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 170, 100, 111, 99, 107, 101, 114, 102, 105, 108, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 101, 120, 116, 114, 97, 72, 111, 115, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 171, 102, 111, 114, 99, 101, 82, 101, 109, 111, 118, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 169, 105, 115, 111, 108, 97, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 108, 97, 98, 101, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103, 166, 108, 97, 98, 101, 108, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103, 166, 109, 101, 109, 111, 114, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 170, 109, 101, 109, 111, 114, 121, 83, 119, 97, 112, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 171, 110, 101, 116, 119, 111, 114, 107, 77, 111, 100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 110, 111, 67, 97, 99, 104, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 168, 112, 108, 97, 116, 102, 111, 114, 109, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 112, 117, 108, 108, 80, 97, 114, 101, 110, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 173, 114, 101, 109, 111, 116, 101, 67, 111, 110, 116, 101, 120, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166, 114, 101, 109, 111, 118, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 172, 115, 101, 99, 117, 114, 105, 116, 121, 79, 112, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 169, 115, 101, 115, 115, 105, 111, 110, 73, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 115, 104, 109, 83, 105, 122, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 166, 115, 113, 117, 97, 115, 104, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 174, 115, 117, 112, 112, 114, 101, 115, 115, 79, 117, 116, 112, 117, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 164, 116, 97, 103, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 166, 116, 97, 114, 103, 101, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 117, 108, 105, 109, 105, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 131, 164, 104, 97, 114, 100, 163, 73, 110, 116, 164, 110, 97, 109, 101, 166, 83, 116, 114, 105, 110, 103, 164, 115, 111, 102, 116, 163, 73, 110, 116, 167, 118, 101, 114, 115, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "forceRemove".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "imageId".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "keepLocally".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108] },
                ResultField { name: "name".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "platform".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "pullTriggers".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "repoDigest".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "triggers".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        remote_image::Res {
            build: o.fields.iter().find(|o| o.name == "build").unwrap().output.duplicate(),
            force_remove: o.fields.iter().find(|o| o.name == "forceRemove").unwrap().output.duplicate(),
            image_id: o.fields.iter().find(|o| o.name == "imageId").unwrap().output.duplicate(),
            keep_locally: o.fields.iter().find(|o| o.name == "keepLocally").unwrap().output.duplicate(),
            name: o.fields.iter().find(|o| o.name == "name").unwrap().output.duplicate(),
            platform: o.fields.iter().find(|o| o.name == "platform").unwrap().output.duplicate(),
            pull_triggers: o.fields.iter().find(|o| o.name == "pullTriggers").unwrap().output.duplicate(),
            repo_digest: o.fields.iter().find(|o| o.name == "repoDigest").unwrap().output.duplicate(),
            triggers: o.fields.iter().find(|o| o.name == "triggers").unwrap().output.duplicate(),
        }

    }
}
impl secret::Guest for Component {
    fn invoke(name: String, args: secret::Args) -> secret::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/secret:Secret".into(),
            name,
            object: vec![
                ObjectField { name: "data".into(), value: args.data },
                ObjectField { name: "labels".into(), value: args.labels },
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "data".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "labels".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 165, 108, 97, 98, 101, 108, 166, 83, 116, 114, 105, 110, 103, 165, 118, 97, 108, 117, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "name".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        secret::Res {
            data: o.fields.iter().find(|o| o.name == "data").unwrap().output.duplicate(),
            labels: o.fields.iter().find(|o| o.name == "labels").unwrap().output.duplicate(),
            name: o.fields.iter().find(|o| o.name == "name").unwrap().output.duplicate(),
        }

    }
}
impl service::Guest for Component {
    fn invoke(name: String, args: service::Args) -> service::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/service:Service".into(),
            name,
            object: vec![
                ObjectField { name: "auth".into(), value: args.auth },
                ObjectField { name: "convergeConfig".into(), value: args.converge_config },
                ObjectField { name: "endpointSpec".into(), value: args.endpoint_spec },
                ObjectField { name: "labels".into(), value: args.labels },
                ObjectField { name: "mode".into(), value: args.mode },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "rollbackConfig".into(), value: args.rollback_config },
                ObjectField { name: "taskSpec".into(), value: args.task_spec },
                ObjectField { name: "updateConfig".into(), value: args.update_config },
            ],
            results: vec![
                ResultField { name: "auth".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 131, 168, 112, 97, 115, 115, 119, 111, 114, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173, 115, 101, 114, 118, 101, 114, 65, 100, 100, 114, 101, 115, 115, 166, 83, 116, 114, 105, 110, 103, 168, 117, 115, 101, 114, 110, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "convergeConfig".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 130, 165, 100, 101, 108, 97, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 116, 105, 109, 101, 111, 117, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "endpointSpec".into(), schema: vec![129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 109, 111, 100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 112, 111, 114, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 133, 164, 110, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 112, 114, 111, 116, 111, 99, 111, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 171, 112, 117, 98, 108, 105, 115, 104, 77, 111, 100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173, 112, 117, 98, 108, 105, 115, 104, 101, 100, 80, 111, 114, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 170, 116, 97, 114, 103, 101, 116, 80, 111, 114, 116, 163, 73, 110, 116] },
                ResultField { name: "labels".into(), schema: vec![129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 165, 108, 97, 98, 101, 108, 166, 83, 116, 114, 105, 110, 103, 165, 118, 97, 108, 117, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "mode".into(), schema: vec![129, 166, 79, 98, 106, 101, 99, 116, 130, 166, 103, 108, 111, 98, 97, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 170, 114, 101, 112, 108, 105, 99, 97, 116, 101, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 129, 168, 114, 101, 112, 108, 105, 99, 97, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116] },
                ResultField { name: "name".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "rollbackConfig".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 134, 165, 100, 101, 108, 97, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173, 102, 97, 105, 108, 117, 114, 101, 65, 99, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 175, 109, 97, 120, 70, 97, 105, 108, 117, 114, 101, 82, 97, 116, 105, 111, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 109, 111, 110, 105, 116, 111, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 111, 114, 100, 101, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 171, 112, 97, 114, 97, 108, 108, 101, 108, 105, 115, 109, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116] },
                ResultField { name: "taskSpec".into(), schema: vec![129, 166, 79, 98, 106, 101, 99, 116, 136, 173, 99, 111, 110, 116, 97, 105, 110, 101, 114, 83, 112, 101, 99, 129, 166, 79, 98, 106, 101, 99, 116, 222, 0, 21, 164, 97, 114, 103, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 168, 99, 111, 109, 109, 97, 110, 100, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 167, 99, 111, 110, 102, 105, 103, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 134, 168, 99, 111, 110, 102, 105, 103, 73, 100, 166, 83, 116, 114, 105, 110, 103, 170, 99, 111, 110, 102, 105, 103, 78, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 102, 105, 108, 101, 71, 105, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 102, 105, 108, 101, 77, 111, 100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 168, 102, 105, 108, 101, 78, 97, 109, 101, 166, 83, 116, 114, 105, 110, 103, 167, 102, 105, 108, 101, 85, 105, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 163, 100, 105, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 169, 100, 110, 115, 67, 111, 110, 102, 105, 103, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 131, 171, 110, 97, 109, 101, 115, 101, 114, 118, 101, 114, 115, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 167, 111, 112, 116, 105, 111, 110, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 168, 115, 101, 97, 114, 99, 104, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 163, 101, 110, 118, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103, 166, 103, 114, 111, 117, 112, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 171, 104, 101, 97, 108, 116, 104, 99, 104, 101, 99, 107, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 133, 168, 105, 110, 116, 101, 114, 118, 97, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 114, 101, 116, 114, 105, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 171, 115, 116, 97, 114, 116, 80, 101, 114, 105, 111, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 116, 101, 115, 116, 115, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 167, 116, 105, 109, 101, 111, 117, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 104, 111, 115, 116, 110, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 104, 111, 115, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 104, 111, 115, 116, 166, 83, 116, 114, 105, 110, 103, 162, 105, 112, 166, 83, 116, 114, 105, 110, 103, 165, 105, 109, 97, 103, 101, 166, 83, 116, 114, 105, 110, 103, 169, 105, 115, 111, 108, 97, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166, 108, 97, 98, 101, 108, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 165, 108, 97, 98, 101, 108, 166, 83, 116, 114, 105, 110, 103, 165, 118, 97, 108, 117, 101, 166, 83, 116, 114, 105, 110, 103, 166, 109, 111, 117, 110, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 135, 171, 98, 105, 110, 100, 79, 112, 116, 105, 111, 110, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 129, 171, 112, 114, 111, 112, 97, 103, 97, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 114, 101, 97, 100, 79, 110, 108, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 166, 115, 111, 117, 114, 99, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166, 116, 97, 114, 103, 101, 116, 166, 83, 116, 114, 105, 110, 103, 172, 116, 109, 112, 102, 115, 79, 112, 116, 105, 111, 110, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 109, 111, 100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 169, 115, 105, 122, 101, 66, 121, 116, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 164, 116, 121, 112, 101, 166, 83, 116, 114, 105, 110, 103, 173, 118, 111, 108, 117, 109, 101, 79, 112, 116, 105, 111, 110, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 132, 170, 100, 114, 105, 118, 101, 114, 78, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173, 100, 114, 105, 118, 101, 114, 79, 112, 116, 105, 111, 110, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103, 166, 108, 97, 98, 101, 108, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 165, 108, 97, 98, 101, 108, 166, 83, 116, 114, 105, 110, 103, 165, 118, 97, 108, 117, 101, 166, 83, 116, 114, 105, 110, 103, 166, 110, 111, 67, 111, 112, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 170, 112, 114, 105, 118, 105, 108, 101, 103, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 130, 174, 99, 114, 101, 100, 101, 110, 116, 105, 97, 108, 83, 112, 101, 99, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 102, 105, 108, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 114, 101, 103, 105, 115, 116, 114, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 115, 101, 76, 105, 110, 117, 120, 67, 111, 110, 116, 101, 120, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 133, 167, 100, 105, 115, 97, 98, 108, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 165, 108, 101, 118, 101, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 114, 111, 108, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 116, 121, 112, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 117, 115, 101, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 114, 101, 97, 100, 79, 110, 108, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 167, 115, 101, 99, 114, 101, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 134, 167, 102, 105, 108, 101, 71, 105, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 102, 105, 108, 101, 77, 111, 100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 168, 102, 105, 108, 101, 78, 97, 109, 101, 166, 83, 116, 114, 105, 110, 103, 167, 102, 105, 108, 101, 85, 105, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 115, 101, 99, 114, 101, 116, 73, 100, 166, 83, 116, 114, 105, 110, 103, 170, 115, 101, 99, 114, 101, 116, 78, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 175, 115, 116, 111, 112, 71, 114, 97, 99, 101, 80, 101, 114, 105, 111, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 115, 116, 111, 112, 83, 105, 103, 110, 97, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166, 115, 121, 115, 99, 116, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103, 164, 117, 115, 101, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 171, 102, 111, 114, 99, 101, 85, 112, 100, 97, 116, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 169, 108, 111, 103, 68, 114, 105, 118, 101, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 110, 97, 109, 101, 166, 83, 116, 114, 105, 110, 103, 167, 111, 112, 116, 105, 111, 110, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103, 177, 110, 101, 116, 119, 111, 114, 107, 115, 65, 100, 118, 97, 110, 99, 101, 100, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 131, 167, 97, 108, 105, 97, 115, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 170, 100, 114, 105, 118, 101, 114, 79, 112, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 164, 110, 97, 109, 101, 166, 83, 116, 114, 105, 110, 103, 169, 112, 108, 97, 99, 101, 109, 101, 110, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 132, 171, 99, 111, 110, 115, 116, 114, 97, 105, 110, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 171, 109, 97, 120, 82, 101, 112, 108, 105, 99, 97, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 169, 112, 108, 97, 116, 102, 111, 114, 109, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 172, 97, 114, 99, 104, 105, 116, 101, 99, 116, 117, 114, 101, 166, 83, 116, 114, 105, 110, 103, 162, 111, 115, 166, 83, 116, 114, 105, 110, 103, 165, 112, 114, 101, 102, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 169, 114, 101, 115, 111, 117, 114, 99, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 130, 166, 108, 105, 109, 105, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 130, 171, 109, 101, 109, 111, 114, 121, 66, 121, 116, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 168, 110, 97, 110, 111, 67, 112, 117, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 171, 114, 101, 115, 101, 114, 118, 97, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 131, 176, 103, 101, 110, 101, 114, 105, 99, 82, 101, 115, 111, 117, 114, 99, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 130, 182, 100, 105, 115, 99, 114, 101, 116, 101, 82, 101, 115, 111, 117, 114, 99, 101, 115, 83, 112, 101, 99, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 179, 110, 97, 109, 101, 100, 82, 101, 115, 111, 117, 114, 99, 101, 115, 83, 112, 101, 99, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 171, 109, 101, 109, 111, 114, 121, 66, 121, 116, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 168, 110, 97, 110, 111, 67, 112, 117, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 173, 114, 101, 115, 116, 97, 114, 116, 80, 111, 108, 105, 99, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 132, 169, 99, 111, 110, 100, 105, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 100, 101, 108, 97, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 171, 109, 97, 120, 65, 116, 116, 101, 109, 112, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 166, 119, 105, 110, 100, 111, 119, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 114, 117, 110, 116, 105, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "updateConfig".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 134, 165, 100, 101, 108, 97, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173, 102, 97, 105, 108, 117, 114, 101, 65, 99, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 175, 109, 97, 120, 70, 97, 105, 108, 117, 114, 101, 82, 97, 116, 105, 111, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 109, 111, 110, 105, 116, 111, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 111, 114, 100, 101, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 171, 112, 97, 114, 97, 108, 108, 101, 108, 105, 115, 109, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116] },
            ],
        };

        let o = register(&request);

        service::Res {
            auth: o.fields.iter().find(|o| o.name == "auth").unwrap().output.duplicate(),
            converge_config: o.fields.iter().find(|o| o.name == "convergeConfig").unwrap().output.duplicate(),
            endpoint_spec: o.fields.iter().find(|o| o.name == "endpointSpec").unwrap().output.duplicate(),
            labels: o.fields.iter().find(|o| o.name == "labels").unwrap().output.duplicate(),
            mode: o.fields.iter().find(|o| o.name == "mode").unwrap().output.duplicate(),
            name: o.fields.iter().find(|o| o.name == "name").unwrap().output.duplicate(),
            rollback_config: o.fields.iter().find(|o| o.name == "rollbackConfig").unwrap().output.duplicate(),
            task_spec: o.fields.iter().find(|o| o.name == "taskSpec").unwrap().output.duplicate(),
            update_config: o.fields.iter().find(|o| o.name == "updateConfig").unwrap().output.duplicate(),
        }

    }
}
impl service_config::Guest for Component {
    fn invoke(name: String, args: service_config::Args) -> service_config::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/serviceConfig:ServiceConfig".into(),
            name,
            object: vec![
                ObjectField { name: "data".into(), value: args.data },
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "data".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "name".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        service_config::Res {
            data: o.fields.iter().find(|o| o.name == "data").unwrap().output.duplicate(),
            name: o.fields.iter().find(|o| o.name == "name").unwrap().output.duplicate(),
        }

    }
}
impl tag::Guest for Component {
    fn invoke(name: String, args: tag::Args) -> tag::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/tag:Tag".into(),
            name,
            object: vec![
                ObjectField { name: "sourceImage".into(), value: args.source_image },
                ObjectField { name: "targetImage".into(), value: args.target_image },
            ],
            results: vec![
                ResultField { name: "sourceImage".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "sourceImageId".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "targetImage".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        tag::Res {
            source_image: o.fields.iter().find(|o| o.name == "sourceImage").unwrap().output.duplicate(),
            source_image_id: o.fields.iter().find(|o| o.name == "sourceImageId").unwrap().output.duplicate(),
            target_image: o.fields.iter().find(|o| o.name == "targetImage").unwrap().output.duplicate(),
        }

    }
}
impl volume::Guest for Component {
    fn invoke(name: String, args: volume::Args) -> volume::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/volume:Volume".into(),
            name,
            object: vec![
                ObjectField { name: "driver".into(), value: args.driver },
                ObjectField { name: "driverOpts".into(), value: args.driver_opts },
                ObjectField { name: "labels".into(), value: args.labels },
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "driver".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "driverOpts".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "labels".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 165, 108, 97, 98, 101, 108, 166, 83, 116, 114, 105, 110, 103, 165, 118, 97, 108, 117, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "mountpoint".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "name".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        volume::Res {
            driver: o.fields.iter().find(|o| o.name == "driver").unwrap().output.duplicate(),
            driver_opts: o.fields.iter().find(|o| o.name == "driverOpts").unwrap().output.duplicate(),
            labels: o.fields.iter().find(|o| o.name == "labels").unwrap().output.duplicate(),
            mountpoint: o.fields.iter().find(|o| o.name == "mountpoint").unwrap().output.duplicate(),
            name: o.fields.iter().find(|o| o.name == "name").unwrap().output.duplicate(),
        }

    }
}
