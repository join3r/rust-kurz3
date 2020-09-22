// vytvor funkciu evie, ktorá
// nastaví premennú x, ktorá bude číslo
// nastaví premennú c, ktorá bude písmeno
// nastaví premennú v, ktorá bude veta
// vypíše na obrazovku Eviena veta je <v> a moje písmeno je <c>
// kde <v> a <c> vymení za hodnoty v a c
pub fn evie() {
    let c = 'A';
    let v = "ahoj som veta";
    println!("Eviena veta je {} a moje pismeno je {}", v, c);
}
// vytvor funkciu evie_if, ktorá
// bude mať vstup bool
// pomocou if alebo match
// ak bude vstup true, tak vypíše písmeno 'T'
// ak bude vstup false, tak vypíše písmeno 'F'
pub fn evie_if(x: bool) {
    if x {
        println!("T");
    } else {
        println!("F");
    };
}

// vytvor funkciu minus, ktorá má ako vstup 2x i64 a výstup i64
// funkcia má odčítať druhý vstup od prvého -

// vytvor funkciu evie_for, ktorá
// vypíše pomocou for alebo while čísla od 1 do i32 (vrátane), ktorý bude vstupom funkcie

// final task
// sprav funkciu evie_all, ktorá spustí
// evie, evie_if, minus a evie_for. Koľko vstupov musí mať aby mala všetko pre všetky funkcie?
// aký musí mať výstup?
