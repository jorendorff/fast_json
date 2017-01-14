#![allow(dead_code)]
#[macro_use]
extern crate rustler;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;
extern crate json;

use rustler::{NifEnv, NifTerm};
use rustler::schedule::NifScheduleFlags::*;

mod decoder;
mod encoder;
mod errors;
mod parser;
mod sink;
mod util;

use decoder::ParserResource;

rustler_export_nifs! {
    "Elixir.Json",
    [("decode_naive", 2, decoder::decode_naive),
     ("decode_init", 2, decoder::decode_init),
     ("decode_iter", 2, decoder::decode_iter),
     ("decode_dirty", 2, decoder::decode_naive, DirtyCpu),
     ("decode_threaded", 2, decoder::decode_threaded),
     ("encode", 2, encoder::encode, DirtyCpu)],
    Some(load)
}

mod atoms {
    rustler_atoms! {
        atom nil;
        atom ok;
        atom error;
        atom more;
        atom true_atom = "true";
        atom false_atom = "false";
    }
}

fn load(env: NifEnv, _info: NifTerm) -> bool {
    resource_struct_init!(ParserResource, env);
    true
}
