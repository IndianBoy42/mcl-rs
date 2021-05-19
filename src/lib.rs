#![feature(trait_alias)]
#![feature(extended_key_value_attributes)]
#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

pub mod filters;
pub mod odometry;
pub mod sensor_models;

mod datatypes;

mod utils {
    pub mod collect_vec;
    pub mod fmap;
    pub mod gaussian;
    pub mod searcher_util;
}

pub use datatypes::*;

pub mod notes;
