// vytvor funkciu firestorm, ktorá
// nastaví premennú x, ktorá bude číslo
// nastaví premennú c, ktorá bude písmeno
// nastaví premennú v, ktorá bude veta
// vypíše na obrazovku Firestormova veta je <v> a moje písmeno je <c>
// kde <v> a <c> vymení za hodnoty v a c

pub fn firestorm() {
    let c = 'a';
    let v = "Lalal";
    println!("Firestormova veta je {} a moje písmeno je {}", v, c);
}

// vytvor funkciu firestorm_if, ktorá
// bude mať vstup bool
// pomocou if alebo match
// ak bude vstup true, tak vypíše písmeno 'T'
// ak bude vstup false, tak vypíše písmeno 'F'
pub fn firestorm_if(b: bool) {
    if b {
        println!("T")
    } else {
        println!("F")
    };
}

// vytvor funkciu krat, ktorá má ako vstup 2x i64 a výstup i64
// funkcia má vynásobiť oba vstupy *

// vytvor funkciu firestorm_for, ktorá
// vypíše pomocou for alebo while čísla od 1 do i32 (vrátane), ktorý bude vstupom funkcie

// final task
// sprav funkciu firestorm_all, ktorá spustí
// firestorm, firestorm_if, krat a firestorm_for. Koľko vstupov musí mať aby mala všetko pre všetky funkcie?
// aký musí mať výstup?
