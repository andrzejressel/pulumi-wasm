use anyhow::Error;

use pulumi_wasm_docker::resource::container;
use pulumi_wasm_docker::resource::container::container;
use pulumi_wasm_random::resource::random_string::{random_string, RandomStringArgs};
use pulumi_wasm_rust::Output;
use pulumi_wasm_rust::{add_export, pulumi_main};

#[pulumi_main]
fn test_main() -> Result<(), Error> {
    let length: Output<i32> = Output::new(&12).map(|i: i32| i * 3);
    let random_string = random_string(
        "test",
        RandomStringArgs {
            keepers: None.into(),
            length,
            lower: None.into(),
            min_lower: None.into(),
            min_numeric: None.into(),
            min_special: None.into(),
            min_upper: None.into(),
            number: None.into(),
            numeric: None.into(),
            override_special: None.into(),
            special: None.into(),
            upper: None.into(),
        },
    );

    // Tests preview behaviour for unknown fields
    let t = random_string.result.map(|s| format!("Result: {s}"));

    // Tests number mapping
    let number = random_string.min_upper.map(|i| i * 2);

    let cont = container(
        "container",
        container::ContainerArgs {
            attach: true.into(),
            capabilities: None.into(),
            cgroupns_mode: None.into(),
            command: vec!["echo".to_string(), "Hello World!".to_string()].into(),
            container_read_refresh_timeout_milliseconds: None.into(),
            cpu_set: None.into(),
            cpu_shares: None.into(),
            destroy_grace_seconds: None.into(),
            devices: None.into(),
            dns: None.into(),
            dns_opts: None.into(),
            dns_searches: None.into(),
            domainname: None.into(),
            entrypoints: None.into(),
            envs: None.into(),
            gpus: None.into(),
            group_adds: None.into(),
            healthcheck: None.into(),
            hostname: None.into(),
            hosts: None.into(),
            image: "ubuntu".to_string().into(),
            init: None.into(),
            ipc_mode: None.into(),
            labels: None.into(),
            log_driver: None.into(),
            log_opts: None.into(),
            logs: true.into(),
            max_retry_count: None.into(),
            memory: None.into(),
            memory_swap: None.into(),
            mounts: None.into(),
            must_run: false.into(),
            name: None.into(),
            network_mode: None.into(),
            networks_advanced: None.into(),
            pid_mode: None.into(),
            ports: None.into(),
            privileged: None.into(),
            publish_all_ports: None.into(),
            read_only: None.into(),
            remove_volumes: None.into(),
            restart: None.into(),
            rm: None.into(),
            runtime: None.into(),
            security_opts: None.into(),
            shm_size: None.into(),
            start: None.into(),
            stdin_open: None.into(),
            stop_signal: None.into(),
            stop_timeout: None.into(),
            storage_opts: None.into(),
            sysctls: None.into(),
            tmpfs: None.into(),
            tty: None.into(),
            ulimits: None.into(),
            uploads: None.into(),
            user: None.into(),
            userns_mode: None.into(),
            volumes: None.into(),
            wait: None.into(),
            wait_timeout: None.into(),
            working_dir: None.into(),
        },
    );

    add_export("logs", &cont.container_logs);
    add_export("result", &random_string.result);
    add_export("transformed_result", &t);
    add_export("number", &number);
    Ok(())
}
