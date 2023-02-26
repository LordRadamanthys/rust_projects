use std::string;

fn main() {
    let mut mateus: Usuario = Usuario {
        nome: String::from("Mateus"),
        idade: 26,
        livros: vec![],
    };
    let clean_code = Livro {
        nome: String::from("Clean Code"),
        autor: String::from("Uncle Bob"),
        valor: 50.0,
    };
    let materia_escura = Livro {
        nome: String::from("Materia escura"),
        autor: String::from("Sei la"),
        valor: 100.0,
    };

    mateus.add_book(clean_code);
    mateus.add_book(materia_escura);
    for i in &mateus.livros {
        
        println!("Autor: {} - Titulo: {} - Valor: {}",i.autor, i.nome, i.valor);
    }
    
    mateus.livros.pop();
    println!("Eliminando ");
    for i in &mateus.livros {
        
        println!("Autor: {} - Titulo: {} - Valor: {}",i.autor, i.nome, i.valor);
    }
    
}

struct Livro {
    nome: String,
    autor: String,
    valor: f32,
}


impl Usuario {
    fn add_book(&mut self, book: Livro) {
        self.livros.push(book);
    }
}

struct Usuario {
    nome: String,
    idade: u8,
    livros: Vec<Livro>,
}
