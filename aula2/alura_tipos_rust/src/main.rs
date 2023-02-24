fn main() {
    
    manipulando_array();
    println!("é fim de semana? {}", fim_de_semana(DiaDaSemana::Domingo));
    // matriz();
    cores();

    conteudo_opcional();
    vectors();
}

fn vectors(){
    let mut notas:Vec<f32> = Vec::new();
    let mut notas2:Vec<f32> = vec![9.9];
    notas.push(4.8);
    notas.push(6.4);
    notas.push(2.8);
    notas.push(9.8);
    // notas.remove(2);

    // pop remove o ultmo item do vetor e retorna ele como valor
    let valorRemovido = match notas.pop() {
     Some(e) => e,
     None => 0.0
    };
    println!("{}",valorRemovido);
    notas.append(&mut notas2);
    println!("{:?}", notas);
    println!("{}", notas[0]);

    println!("Nota 4 = {}", match notas.get(1) {
        Some(e) => *e,
        None =>0.0
    });


    for v in notas {
        println!("{}",v);
    }
}


fn manipulando_array(){
    // let notas: [f64; 4] = [10.0, 8.0, 9.5, 6.0];

    // inicializando notas com valor default
    let notas: [f64; 4] = [10.0; 4];

    //Para acessar valores de um array se utiliza "usize"
    let numero: usize = 0;

    println!("{}", notas[numero]);

    // for nota in notas {
    //         println!("Nota é {}",nota)
    // }

    for indice in 0..notas.len() {
        println!("Nota{} é {}", indice + 1, notas[indice])
    }
}

fn conteudo_opcional() {
    let conteudo_arquivo = ler_arquivo(String::from(""));

    // match &conteudo_arquivo {
    //     Some(value) => println!("{}", value),
    //     None => println!("Arquivo n existe"),
    // };

        if let Some(valor) = &conteudo_arquivo{
            println!("Certeza q tem agora")
        }
        
        if  let None = &conteudo_arquivo{
            println!("ixe deu none")
        }
    // println!("{:?}",&conteudo_arquivo)
}

fn ler_arquivo(caminho: String) -> Option<String> {
    // Some(String::from("Conteudo do arquivo"))
    None
}

enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
}

enum Colors {
    Azul,
    Verde,
    Branco,
    Amarelo,
    Cinza,
    Rgb(u8, u8, u8),
    CynkColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

fn cores() {
    let cor = Colors::CynkColor {
        cyan: 1,
        magenta: 1,
        yellow: 1,
        black: 1,
    };

    println!(
        "Cor = {}",
        match cor {
            Colors::Amarelo => "Amarelo",
            Colors::Azul => "Azul",
            Colors::Branco => "Branco",
            Colors::CynkColor {
                cyan: _,
                magenta: _,
                yellow: _,
                black: _,
            } => "Cyan",
            Colors::Cinza => "Cinza",
            Colors::Rgb(1, 1, 1) => "Preto",
            Colors::Rgb(_, _, _) => "Sei la",
            _ => "Qualquer coisa",
        }
    )
}

fn fim_de_semana(dia_da_semana: DiaDaSemana) -> bool {
    match dia_da_semana {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false,
    }
}

fn matriz() {
    let matriz = [[0.0, 0.1, 0.2], [0.0, 0.6, 0.5]];

    for linha in matriz {
        for col in linha {
            println!("item {}", col)
        }
    }
}
