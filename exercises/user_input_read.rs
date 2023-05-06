use std::io;

fn main() {
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer);
  // Agora, "buffer" contém os valores da entrada do usuário.
}
