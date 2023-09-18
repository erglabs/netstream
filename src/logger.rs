/*
Copyright (C) 2023 ErgLabs <dev@erglabs.org>.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter};
pub fn init_subscriber() -> anyhow::Result<()> {
    // let file_appender = tracing_appender::rolling::daily("logs",
    // "netstream.log");
    // todo:esavier switch for file logging
    // let subscriber = tracing_subscriber::registry()
    //     .with(EnvFilter::from_default_env())
    //     .with(
    //         fmt::Layer::new()
    //             .with_file(false)
    //             .with_line_number(true)
    //             .with_ansi(false)
    //             .with_writer(file_appender),
    //     );
    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(
            fmt::Layer::default()
                .with_target(true)
                .with_thread_names(true)
                .with_ansi(true)
                .with_line_number(true)
                .with_file(true)
                .with_thread_ids(true),
        );
    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to set a global logger instance");
    tracing::info!("logger initialized, new application run");
    tracing::info!("===========================================================");
    tracing::info!("ComponentInfo:");
    tracing::info!(">> BUILDER -----------------------------------------------------------");
    tracing::info!(
        "BUILD_TIMESTAMP       {}",
        std::env!("VERGEN_BUILD_TIMESTAMP")
    );
    tracing::info!(">> CARGO -------------------------------------------------------------");
    tracing::info!("CARGO_DEBUG           {}", std::env!("VERGEN_CARGO_DEBUG"));
    tracing::info!(
        "CARGO_FEATURES        {}",
        std::env!("VERGEN_CARGO_FEATURES")
    );
    tracing::info!(
        "CARGO_OPTLVL          {}",
        std::env!("VERGEN_CARGO_OPT_LEVEL")
    );
    tracing::info!(
        "CARGO_TARGET          {}",
        std::env!("VERGEN_CARGO_TARGET_TRIPLE")
    );
    tracing::info!(">> GIT ----------------------------------------------------------------");
    tracing::info!("GIT_BRANCH            {}", std::env!("VERGEN_GIT_BRANCH"));
    tracing::info!(
        "GIT_COMMIT_COUNT      {}",
        std::env!("VERGEN_GIT_COMMIT_COUNT")
    );
    tracing::info!(
        "GIT_COMMIT_TIEMESTAMP {}",
        std::env!("VERGEN_GIT_COMMIT_TIMESTAMP")
    );
    tracing::info!("GIT_DESCRIBE          {}", std::env!("VERGEN_GIT_DESCRIBE"));
    tracing::info!("GIT_SHA               {}", std::env!("VERGEN_GIT_SHA"));
    tracing::info!(">> RUSTC --------------------------------------------------------------");
    tracing::info!(
        "RUSTC_CHANNEL         {}",
        std::env!("VERGEN_RUSTC_CHANNEL")
    );
    tracing::info!(
        "RUSTC_TIMESTAMP       {}",
        std::env!("VERGEN_RUSTC_COMMIT_DATE")
    );
    tracing::info!(
        "RUSTC_HASAH           {}",
        std::env!("VERGEN_RUSTC_COMMIT_HASH")
    );
    tracing::info!(
        "RUSTC_TRIPLET         {}",
        std::env!("VERGEN_RUSTC_HOST_TRIPLE")
    );
    tracing::info!(
        "RUSTC_LLVM_V          {}",
        std::env!("VERGEN_RUSTC_LLVM_VERSION")
    );
    tracing::info!("RUSTC_RUSTC_SV        {}", std::env!("VERGEN_RUSTC_SEMVER"));
    tracing::info!(">> SYSTEM -------------------------------------------------------------");
    tracing::info!("SYSTEM_OSNAME         {}", std::env!("VERGEN_SYSINFO_NAME"));
    tracing::info!(
        "SYSTEM_OSVERSION      {}",
        std::env!("VERGEN_SYSINFO_OS_VERSION")
    );
    tracing::info!(
        "SYSTEM_MEMORY         {}",
        std::env!("VERGEN_SYSINFO_TOTAL_MEMORY")
    );
    tracing::info!(
        "SYSTEM_CPU_VENDOR     {}",
        std::env!("VERGEN_SYSINFO_CPU_VENDOR")
    );
    tracing::info!(
        "SYSTEM_CPU_BRAND      {}",
        std::env!("VERGEN_SYSINFO_CPU_BRAND")
    );
    tracing::info!("===========================================================");
    Ok(())
}
