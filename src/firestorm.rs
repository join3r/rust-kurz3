// vytvor funkciu firestorm, ktorá
// nastaví premennú x, ktorá bude číslo
// nastaví premennú c, ktorá bude písmeno
// nastaví premennú v, ktorá bude veta
// vypíše na obrazovku Firestormova veta je <v> a moje písmeno je <c>
// kde <v> a <c> vymení za hodnoty v a c

pub fn firestorm() {
    let x = 1;
    let c = 'a';
    let v = "Firestormova veta je Firestormova veta je Firestormova veta je Firestormova veta je ";
    println!("Firestormova veta je {} a moje písmeno je {}", v, c);
}

// vytvor funkciu firestorm_if, ktorá
// bude mať vstup bool
// pomocou if alebo match
// ak bude vstup true, tak vypíše písmeno 'T'
// ak bude vstup false, tak vypíše písmeno 'F'
pub fn firestorm_if(b: bool) {
    if b {
        println!("T");
    } else {
        println!("F");
    };
}

// vytvor funkciu krat, ktorá má ako vstup 2x i64 a výstup i64
// funkcia má vynásobiť oba vstupy *
pub fn krat(a: i64, b: i64) -> i64 {
    a * b
}

// vytvor funkciu firestorm_for, ktorá
// vypíše pomocou for alebo while čísla od 1 do i32 (vrátane), ktorý bude vstupom funkcie
pub fn firestorm_for(input: i32) {
    if input < 1 {
        println!("Cislo je mimo");
    } else {
        for i in 1..=input {
            println!("{}", i);
        }
    };
}

// Oprav funckiu firestorm_for, tak aby ak bude vstup menší ako 1 tak vypíše, že číslo je mimo

// final task
// sprav funkciu firestorm_all, ktorá spustí
// firestorm, firestorm_if, krat a firestorm_for. Koľko vstupov musí mať aby mala všetko pre všetky funkcie?
// aký musí mať výstup?

pub fn firestorm_all(be: bool, cislo1: i64, cislo2: i64, horny_interval: i32) -> i64 {
    firestorm();
    firestorm_if(be);
    firestorm_for(horny_interval);
    krat(cislo1, cislo2)
}
