// vytvor funkciu osiris, ktorá
// nastaví premennú x, ktorá bude číslo
// nastaví premennú c, ktorá bude písmeno
// nastaví premennú v, ktorá bude veta
// vypíše na obrazovku Osirisova veta je <v> a moje písmeno je <c>
// kde <v> a <c> vymení za hodnoty v a c

pub fn osiris() {
    let c = 'O';
    let v = "Holvan a kibaszott hutotaska?!?!";
    println!("Osirisova veta je <{}> a moje pismeno je <{}>", v, c);
}

// vytvor funkciu osiris_if, ktorá
// bude mať vstup bool
// pomocou if alebo match
// ak bude vstup true, tak vypíše písmeno 'T'
// ak bude vstup false, tak vypíše písmeno 'F'

pub fn osiris_if(gazda: bool) {
    if gazda {
        println!("T");
    } else {
        println!("F")
    };
}

// vytvor funkciu plus, ktorá má ako vstup 2x i64 a výstup i64
// funkcia má spočítať oba vstupy +

// vytvor funkciu osiris_for, ktorá
// vypíše pomocou for alebo while čísla od 1 do i32 (vrátane), ktorý bude vstupom funkcie

// final task
// sprav funkciu osiris_all, ktorá spustí
// osiris, osiris_if, plus a osiris_for. Koľko vstupov musí mať aby mala všetko pre všetky funkcie?
// aký musí mať výstup?
