use crate::internal::core::options::Scenario;
use crate::internal::core::options::ScenarioRegistry;

use super::cli::{Args, SimulationConfig, get_config_from_args, utils::write_file_with_dirs};
use super::core::{
    Context, log,
    options::{
        ArrivalTimeCallback, ArrivalTimeRegistry, ExperimentOptions, Topology, TopologyRegistry,
    },
};
use chrono::Local;
use clap::Parser;

#[derive(Default)]
pub struct Simulator {
    pub scenario_registry: ScenarioRegistry,
    pub topology_registry: TopologyRegistry,
    pub arrival_time_registry: ArrivalTimeRegistry,
}

impl Simulator {
    #[must_use]
    pub fn new() -> Self {
        Self {
            scenario_registry: ScenarioRegistry::new(),
            topology_registry: TopologyRegistry::new(),
            arrival_time_registry: ArrivalTimeRegistry::new(),
        }
    }

    pub fn add_scenario<S: Scenario>(&mut self) -> &mut Self {
        self.scenario_registry.register::<S>();
        self
    }

    pub fn add_topology<T: Topology>(&mut self) -> &mut Self {
        self.topology_registry.register::<T>();
        self
    }

    pub fn add_arrival_time_cb<A: ArrivalTimeCallback>(&mut self) -> &mut Self {
        self.arrival_time_registry.register::<A>();
        self
    }

    pub fn start(&mut self) {
        let args = Args::parse();

        let mut config: SimulationConfig = match get_config_from_args(args.clone(), self) {
            Ok(c_option) => match c_option {
                Some(c) => c,
                None => return,
            },
            Err(e) => {
                log::global_error(format!("Failed to load configuration file: {e}"));
                return;
            }
        };

        for experiment in &mut config.experiments {
            // Print new line before each experiment
            println!();
            log::global_internal(format!("EXPERIMENT '{}'", experiment.name));

            let mut exp_ctx = Context::new(experiment.seed, args.logger_level);
            if let Some(rate) = experiment.drop_rate {
                exp_ctx.set_drop_rate(rate);
            }
            if let Some(jitter) = experiment.jitter {
                exp_ctx.set_jitter(jitter);
            }

            if let Some(directory) = &config.dir {
                let log_file_path = format!(
                    "{}/{}/{}/{}.log",
                    directory,
                    experiment.name,
                    Local::now().timestamp(),
                    experiment.name,
                );
                let metrics_file_path = format!(
                    "{}/{}/{}/{}.jsonl",
                    directory,
                    experiment.name,
                    Local::now().timestamp(),
                    experiment.name,
                );
                if let Err(e) = exp_ctx.logger.set_log_file(&log_file_path) {
                    log::global_error(format!("Failed to set log file to {log_file_path}: {e}"));
                }
                if let Err(e) = exp_ctx.logger.set_metrics_file(&metrics_file_path) {
                    log::global_error(format!(
                        "Failed to set metrics file to {metrics_file_path}: {e}"
                    ));
                }
            }

            exp_ctx.logger.set_flush_threshold(args.flush_threshold);

            // add generated seed to config
            experiment.seed = Some(exp_ctx.seed);

            let opts = ExperimentOptions {
                topology: experiment.topology.clone(),
                arrival_time: experiment.arrival_time.clone(),
                deadline: experiment.deadline,
                extra_args: experiment.extra_args.clone(),
            };

            if let Err(err) =
                self.scenario_registry
                    .run_scenario(&experiment.scenario, &mut exp_ctx, self, opts)
            {
                log::global_error(format!("Scenario not run: {err:?}"));
            }
        }

        // TODO: Deal with this as it may panic
        let yaml_str = serde_yaml::to_string(&config).expect("Failed to serialized configuration");

        if config.should_write_config {
            if let Some(dir) = config.dir {
                let outfile = format!("{dir}/config.yaml");
                match write_file_with_dirs(&outfile, &yaml_str) {
                    Ok(()) => {
                        log::global_info(format!("Wrote configuration file to: {outfile}"));
                    }
                    Err(e) => {
                        log::global_warn(format!(
                            "Failed to write configuration file: {e}\nto: {outfile}"
                        ));
                    }
                }
            } else {
                println!("\nConfiguration file:\n{yaml_str}");
            }
        }
    }
}
