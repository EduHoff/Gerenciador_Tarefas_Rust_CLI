use std::io::{self, Write};

struct Tarefa{
    ativo: bool,
    titulo: String,
}


fn main() {


    let teste = Tarefa {
        ativo: true,
        titulo: String::from("Título Experimental"),
    };

    loop {
        println!("--Conceito Menu Interativo--");
        println!("Digite EOF para sair do loop!");
        print!("||");
        io::stdout().flush().expect("Falha ao limpar o buffer");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Falha ao ler a linha");

        if input.trim() == "EOF" {
            break;
        }
    }

    println!("Ativo: {}\nTítulo: {}", teste.ativo, teste.titulo);
    //OBS.: isso é apenas um projeto de aquecimento.
}
