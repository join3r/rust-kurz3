// vytvor funkciu osiris, ktorá
// nastaví premennú x, ktorá bude číslo
// nastaví premennú c, ktorá bude písmeno
// nastaví premennú v, ktorá bude veta
// vypíše na obrazovku Osirisova veta je <v> a moje písmeno je <c>
// kde <v> a <c> vymení za hodnoty v a c

pub fn osiris() {
    let x = 5;
    let c = 'Q';
    let v = "Krumple, bandurky, zemiaky.";
    println!("Moja veta je <{}> a moje pismeno je <{}", v, c);
}

// vytvor funkciu osiris_if, ktorá
// bude mať vstup bool
// pomocou if alebo match
// ak bude vstup true, tak vypíše písmeno 'T'
// ak bude vstup false, tak vypíše písmeno 'F'

pub fn osiris_if(vstup: bool) {
    if vstup {
        println!("T");
    } else {
        println!("F");
    };
}

// vytvor funkciu plus, ktorá má ako vstup 2x i64 a výstup i64
// funkcia má spočítať oba vstupy +

pub fn plus(a: i64, b: i64) -> i64 {
    a + b
}

// vytvor funkciu osiris_for, ktorá
// vypíše pomocou for alebo while čísla od 1 do i32 (vrátane), ktorý bude vstupom funkcie

pub fn osiris_for(u: i32) {
    for i in 1..=u {
        println!("{}", i);
    }
}

// Oprav funckiu osiris_for, tak aby ak bude vstup menší ako 1 tak vypíše, že číslo je mimo

pub fn osiris_for_lepsia(u: i32) {
    if u < 1 {
        println!("Co to zadavas za glupoty!");
    } else {
        for i in 1..=u {
            println!("{}", i);
        }
    };
}

// final task
// sprav funkciu osiris_all, ktorá spustí
// osiris, osiris_if, plus a osiris_for. Koľko vstupov musí mať aby mala všetko pre všetky funkcie?
// aký musí mať výstup?

pub fn osiris_all(vstup: bool, a: i64, b: i64, u: i32) -> i64 {
    osiris();
    osiris_if(vstup);
    osiris_for_lepsia(u);
    plus(a, b)
}


