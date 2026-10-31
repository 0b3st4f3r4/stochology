//! Submodules for the `labs` binary: the 7 methodological and empirical laboratories of Stochology.

pub mod lab1_layers;
pub mod lab2_uncertainty;
pub mod lab3_lenses;
pub mod lab4_invariants;
pub mod lab5_siga;
pub mod lab6_governance;
pub mod lab7_real_data;

use std::path::Path;

/// Descriptor for a single laboratory in `labs`.
#[derive(Debug, Clone, Copy)]
pub struct LabDescriptor {
    pub cli_flag: &'static str,
    pub run: fn(&Path),
}

fn run_lab2_wrapper(_root: &Path) {
    lab2_uncertainty::lab_uncertainty();
}

/// Canonical registry of all 7 laboratories in execution order.
pub const ALL_LABS: [LabDescriptor; 7] = [
    LabDescriptor {
        cli_flag: "--layers",
        run: lab1_layers::lab_layers,
    },
    LabDescriptor {
        cli_flag: "--uncertainty",
        run: run_lab2_wrapper,
    },
    LabDescriptor {
        cli_flag: "--lenses",
        run: lab3_lenses::lab_lenses,
    },
    LabDescriptor {
        cli_flag: "--invariants",
        run: lab4_invariants::lab_invariants,
    },
    LabDescriptor {
        cli_flag: "--siga",
        run: lab5_siga::lab_siga,
    },
    LabDescriptor {
        cli_flag: "--governance",
        run: lab6_governance::lab_governance,
    },
    LabDescriptor {
        cli_flag: "--real-data",
        run: lab7_real_data::lab_real_data,
    },
];
