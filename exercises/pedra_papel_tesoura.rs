use std::io;

pub fn print_welcome() {
    println!("---\nBem-vindo ao jogo Pedra, Papel ou Tesoura\n---");
}

pub fn get_choice(choice : &mut String) {
    *choice = String::new();
    io::stdin().read_line(choice);
    *choice = choice.trim().to_lowercase().to_string();
}

fn main() {
    print_welcome();
    let mut choice = String::new();
    println!("Digite pedra, papel ou tesoura: ");
    get_choice(&mut choice);
    while choice != "pedra" {
        println!("Você perdeu, tente novamente.");
        println!("{}", choice);
        get_choice(&mut choice);
    }
    println!("Parabéns, você ganhou!");
}
