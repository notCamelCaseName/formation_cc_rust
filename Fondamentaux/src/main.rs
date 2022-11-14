#![allow(dead_code, unused_imports)]


use std::io;


mod exo1;
use crate::exo1::*;

mod exo2;
use crate::exo2::*;

struct Color(u8, u8, u8);

struct Point {
    x : f32,
    y : f32
}

enum ResultatCF {
    Echec,
    Reussite(f32)
}

fn main() {
    // Ceci est un commentaire
    /*
    Ceci est un
    commentaire multiligne
    */

    let foo: i32 = 42;  // Variable immutable
    let mut bar = 21;   // Variable mutable

    bar = foo-bar;

    println!("foo = {}; bar = {}", foo, bar); // Ceci est une macro

    if dis_bonjour("fdp") { // if + appel de fonction
        println!("Bébou");
    }

    let mut ctr = 0;
    while ctr<10 {
        ctr += 1;
    }

    let notes = [10, 15, 8, 13, 69420];
    for note in notes {
        println!("Un élève à eu : {note}");
    }

    let _c = Color(255, 127, 63);

    let _origine = Point {
        x : 0.,
        y : 0.
    };

    let mut read = String::new();
    println!("Entrez votre note");
    io::stdin()
        .read_line(&mut read)
        .expect("Failed to read line");

    let note: f32 = read.trim()
        .parse()
        .expect("Please type a number!");

    let res : ResultatCF;

    if note < 10. {
        res = ResultatCF::Echec;
    } else {
        res = ResultatCF::Reussite(note);
    }

    match res {
        ResultatCF::Echec => {println!("You failed");},
        ResultatCF::Reussite(n) => {println!("GGEZ B), j'ai eu {n}");}
    }

    let exp = LogExp::Et(&LogExp::F, &LogExp::X(0));

    println!("eval : {:?}", sat(&exp));

}

fn dis_bonjour(nom: &str) -> bool {
    println!("Bonjour, {}", nom);
    nom == "Loulou"
}
