// vytvor funkciu hydrant, ktorá
// nastaví premennú x, ktorá bude číslo
// nastaví premennú c, ktorá bude písmeno
// nastaví premennú v, ktorá bude veta
// vypíše na obrazovku Hydrantova veta je <v> a moje písmeno je <c>
// kde <v> a <c> vymení za hodnoty v a c

pub fn hydrant() {
  let x: u8 = 32;
  let c: char = 'P';
  let v: &str = "NAJSUPER veta na svete.";
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

pub fn vacsie_ako(x: i64, y: i64) -> bool {
  x > y
}

// vytvor funkciu hydrant_for, ktorá
// vypíše pomocou for alebo while čísla od 1 do i32 (vrátane), ktorý bude vstupom funkcie

pub fn hydrant_for(i: i32) {

  if i >= 1 {
    for i in 1..=i {
      print!("{} ", i);
    }
  } else {
    println!("Ta co si mimon?");
  };
  println!("{}", i);
}

// Oprav funckiu hydrant_for, tak aby ak bude vstup menší ako 1 tak vypíše, že číslo je mimo

// final task
// sprav funkciu hydrant_all, ktorá spustí
// hydrant, hydrant_if, vacsie_ako a hydrant_for. Koľko vstupov musí mať aby mala všetko pre všetky funkcie?
// aký musí mať výstup?

pub fn hydrant_all(x1: bool, x2: i64, y2: i64, x3: i32) -> bool {
  hydrant();
  hydrant_if(x1);  
  let z = vacsie_ako(x2,y2);
  hydrant_for(x3);
  z
}