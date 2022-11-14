use std::io;

pub fn reussi_ton_annee() -> i32 {
    let mut read = String::new();
    println!("Entrez votre note");
    io::stdin()
        .read_line(&mut read)               // Lis dans stdin
        .expect("Failed to read line") ;    // Gestion d'erreur

    let mut note: f32 = read.trim()         // Retire les espaces et \n
        .parse()                            // Transforme en entier
        .expect("Please type a number!");   // Gestion d'erreur, le retour

    let mut cf: i32 = 1;

    while note < 10. {
        println!("You failed");
        cf += 1;
        let mut read = String::new();
        println!("Entrez votre note");
        io::stdin()
            .read_line(&mut read)
            .expect("Failed to read line");

        note = read.trim()
            .parse()
            .expect("Please type a number!");
    }
    println!("GGEZ B)");

    cf
}