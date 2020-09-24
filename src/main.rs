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
    baci();
    evie();
    firestorm();
    hydrant();
    osiris();
    bachi_if(true);
    evie_if(false);
    osiris_if(true);
    firestorm_if(true);
    hydrant_if(false);
    assert_eq!(minus(10, 5), 5);
    assert_eq!(delene(10, 2), 5);
    assert_eq!(plus(1, 1), 2);
    assert_eq!(krat(1, 1), 1);
    assert_eq!(vacsie_ako(2, 3), false);
}

