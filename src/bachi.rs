// vytvor funkciu bachi, ktorá
// nastaví premennú x, ktorá bude číslo
// nastaví premennú c, ktorá bude písmeno
// nastaví premennú v, ktorá bude veta
// vypíše na obrazovku Bachiho veta je <v> a moje písmeno je <c>
// kde <v> a <c> vymení za hodnoty v a c
pub fn baci() {
    let c = 'p';
    let v = "ahoj ako";
    println!("Baciho veta je {} a moje pismeno je {}", v, c);
}
// vytvor funkciu bachi_if, ktorá
// bude mať vstup bool
// pomocou if alebo match
// ak bude vstup true, tak vypíše písmeno 'T'
// ak bude vstup false, tak vypíše písmeno 'F'

pub fn bachi_if(baci: bool) {
    if baci {
        println!("T");
    } else {
        println!("F");
    };
}

// vytvor funkciu delene, ktorá má ako vstup 2x i64 a výstup i64
// funkcia má vydeliť oba vstupy

// vytvor funkciu bachi_for, ktorá
// vypíše pomocou for čísla od 1 do i32 (vrátane), ktorý bude vstupom funkcie

// final task
// sprav funkciu bachi_all, ktorá spustí
// bachi, bachi_if, delene a bachi_for. Koľko vstupov musí mať aby mala všetko pre všetky funkcie?
// aký musí mať výstup?
