trait Fala {
    fn falar(&self, stri: String);
}

struct Pessoa {
    nome : String,
    idade : u32,
}

impl Fala for Pessoa {
    fn falar(&self, stri: String) {
        println!("{} {}", stri, self.nome);
    }
}

fn main() {
    let joao = Pessoa{
        nome: String::from("João"),
        idade: 18,
    };
    let stri = String::from("Meu nome é...");
    
    joao.falar(stri);
}
