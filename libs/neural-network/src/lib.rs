#![feature(array_methods)]
#![feature(crate_visibility_modifier)]

pub use self::{layer_topology::*, network::*};
use self::{layer::*, neurons::*};
use rand::Rng;

mod layer;
mod layer_topology;
mod neurons;
mod network;
