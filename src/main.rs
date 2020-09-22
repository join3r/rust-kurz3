mod bachi;
use bachi::*;
mod evie;
use evie::*;
mod firestorm;
use firestorm::*;
mod hydrant;
use hydrant::*;
mod osiris;
use osiris::*;

fn main() {
    bachi_if(true);
    evie_if(false);
    osiris_if(true);
    firestorm_if(true);
    hydrant_if(false);
}