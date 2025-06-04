use super::cli::{Args, get_config_from_args, utils::write_file_with_dirs};
use super::core::{
    Context,
    config::SimulationConfig,
    log,
    options::{
        ArrivalTimeCallback, ArrivalTimeRegistry, ExperimentOptions, Topology, TopologyRegistry,
    },
    simulation::{Simulation, SimulationRegistry},
};
use chrono::Local;
use clap::Parser;

#[derive(Default)]
pub struct Simulator {
    pub simulation_registry: SimulationRegistry,
    pub topology_registry: TopologyRegistry,
    pub arrival_time_registry: ArrivalTimeRegistry,
}

impl Simulator {
    #[must_use]
    pub fn new() -> Self {
        Self {
            simulation_registry: SimulationRegistry::new(),
            topology_registry: TopologyRegistry::new(),
            arrival_time_registry: ArrivalTimeRegistry::new(),
        }
    }

    pub fn add_simulation<S: Simulation>(&mut self) -> &mut Self {
        self.simulation_registry.register::<S>();
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
            // if n_peers isn't specified it will be the default value of 5
            experiment.n_peers = Some(experiment.n_peers.unwrap_or(5));

            let opts = ExperimentOptions {
                n_peers: experiment.n_peers.unwrap_or(5),
                topology: experiment.topology.clone(),
                arrival_time: experiment.arrival_time.clone(),
            };

            if let Err(err) = self.simulation_registry.run_simulation(
                &experiment.simulation,
                &mut exp_ctx,
                self,
                opts,
            ) {
                log::global_error(format!("Simulation not run: {err:?}"));
            }
        }

        // TODO: Deal with this as it may panic
        let toml_str = toml::to_string(&config).expect("Failed to serialized configuration");

        if config.should_write_config {
            if let Some(dir) = config.dir {
                let outfile = format!("{dir}/config.toml");
                match write_file_with_dirs(&outfile, &toml_str) {
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
                println!("\nConfiguration file:\n{toml_str}");
            }
        } else {
            println!("\nThe parsed configuration file is:\n{toml_str}");
        }
    }
}
