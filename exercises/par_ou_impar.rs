mod par_ou_impar {
    pub fn par(num : i32) {
        println!("O número {} é par", num);
    }
    pub fn impar(num : i32) {
        println!("O número {} é impar", num);
    }
}

fn main() {
    let number = 15;
    if number % 2 == 0 {
        par_ou_impar::par(number);
    } else {
        par_ou_impar::impar(number);
    }
}
