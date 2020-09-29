// vytvor funkciu evie, ktorá
// nastaví premennú x, ktorá bude číslo
// nastaví premennú c, ktorá bude písmeno
// nastaví premennú v, ktorá bude veta
// vypíše na obrazovku Eviena veta je <v> a moje písmeno je <c>
// kde <v> a <c> vymení za hodnoty v a c

pub fn evie() {
    let x = 4;
    let c = 'A';
    let v = "ahoooj, ako sa mas?";
    println!("Eviena veta je {} a moje pismeno je {}", v, c);
}

// vytvor funkciu evie_if, ktorá
// bude mať vstup bool
// pomocou if alebo match
// ak bude vstup true, tak vypíše písmeno 'T'
// ak bude vstup false, tak vypíše písmeno 'F'

pub fn evie_if(nieco: bool) {
    if nieco {
        println!("T");
    } else {
        println!("F");
    };
}

// vytvor funkciu minus, ktorá má ako vstup 2x i64 a výstup i64
// funkcia má odčítať druhý vstup od prvého -

pub fn minus(x: i64, y: i64) -> i64 {
    x - y
}

// vytvor funkciu evie_for, ktorá
// vypíše pomocou for alebo while čísla od 1 do i32 (vrátane), ktorý bude vstupom funkcie

pub fn evie_for(i: i32) {
    for i in 1..=i {
        println!("{}", i);
    }
}

// Oprav funckiu evie_for, tak aby ak bude vstup menší ako 1 tak vypíše, že číslo je mimo

pub fn evie_for1(i: i32) {
  if i<1 {
        println!("Cislo je mimo");
   } else {
       for i in 1..=i {
            println!("{}", i);
    
        }
   };
}

// final task
// sprav funkciu evie_all, ktorá spustí
// evie, evie_if, minus a evie_for. Koľko vstupov musí mať aby mala všetko pre všetky funkcie?
// aký musí mať výstup?

pub fn evie_all(for1: i32, pif: bool, min1: i64, min2: i64) -> i64 {
    evie();
    evie_for1(for1);
    evie_if(pif);
    minus(min1,min2)
}

// Zadefinuj Struct pre final task
// zmen funkciu vo final tasku, tak aby používala ten Struct