// vytvor funkciu hydrant, ktorá
// nastaví premennú x, ktorá bude číslo
// nastaví premennú c, ktorá bude písmeno
// nastaví premennú v, ktorá bude veta
// vypíše na obrazovku Hydrantova veta je <v> a moje písmeno je <c>
// kde <v> a <c> vymení za hodnoty v a c

pub fn hydrant() {
//  let x: u16 = 777;
  let c: char = 'F';
  let v: &str = "NAJ Super veta na svete!";
  println!("Hydrantova veta je <{}> a moje pismeno je <{}>", v, c);
}

// vytvor funkciu hydrant_if, ktorá
// bude mať vstup bool
// pomocou if alebo match
// ak bude vstup true, tak vypíše písmeno 'T'
// ak bude vstup false, tak vypíše písmeno 'F'

pub fn hydrant_if(volaco: bool) {
  if volaco {
    println!("T");
  } else {
    println!("F");
  };
}

// vytvor funkciu vacsie_ako, ktorá má ako vstup 2x i64 a výstup bool
// funkcia vráti true ak je prvý vstup väčší ako druhý
// funkcia vráti false ak je prvý vstup menší alebo rovný ako druhý

// vytvor funkciu hydrant_for, ktorá
// vypíše pomocou for alebo while čísla od 1 do i32 (vrátane), ktorý bude vstupom funkcie

// final task
// sprav funkciu hydrant_all, ktorá spustí
// hydrant, hydrant_if, vacsie_ako a hydrant_for. Koľko vstupov musí mať aby mala všetko pre všetky funkcie?
// aký musí mať výstup?
