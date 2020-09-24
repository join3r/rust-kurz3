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

pub fn delene(a: i64, b: i64) -> i64 {
    a / b
}

// vytvor funkciu bachi_for, ktorá
// vypíše pomocou for čísla od 1 do i32 (vrátane), ktorý bude vstupom funkcie
// Oprav funckiu bachi, tak aby ak bude vstup menší ako 1 tak vypíše, že číslo je mimo

pub fn bachi_for(dole: i32, hore: i32) {
    if dole > hore {
        println!("cislo je mimo")
    } else {
        for a in dole..=hore {
            println!("{}", a);
        }
    }
}

// final task
// sprav funkciu bachi_all, ktorá spustí
pub fn bachi_all(delene1: i64, delene2: i64, z:i32, q:i32, h: bool) -> i64 {
    bachi_for(z, q);
    bachi_if(h);
    let vyst_del = delene(delene1, delene2);
    baci();
    vyst_del
}

//  pub fn delene(a: i64, b: i64) -> i64 {
//     a / b;
// }
