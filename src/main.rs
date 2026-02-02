struct Tarefa{
    ativo: bool,
    titulo: String,
}


fn main() {


    let teste = Tarefa {
        ativo: true,
        titulo: String::from("Título Experimental"),
    };

    println!("Ativo: {}\nTítulo: {}", teste.ativo, teste.titulo);
    //OBS.: isso é apenas um projeto de aquecimento.
}
