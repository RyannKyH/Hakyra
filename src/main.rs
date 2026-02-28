use rand::RngExt;
use std::io;
fn main() {
    let numero = rand::rng().random_range(1..=10);
    println!("Adivinhe o Numero de 1-10");
    println!("Qual é o seu Palpite: ");
    let mut palpite = String::new();
    io::stdin()
        .read_line(&mut palpite)
        .expect("Não foi possivel ler o seu Palpite");
    let num: i32 = palpite.trim().parse().expect("Error");
    println!("O seu Palpite foi {palpite} \n e o numero era {numero}");
    if num == numero {
        println!("Voce acertou o numero :p")
    } else {
        println!("Voce errou o numero :/")
    }
}
