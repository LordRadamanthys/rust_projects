static mut TESTE: i64 = 1234;

fn main() {
    // let variavel = 300;
    // let decimal: f32 = 300.01;
    // println!(
    //     "variavel = {}, tamanho = {}",
    //     variavel,
    //     std::mem::size_of_val(&variavel)
    // );
    // println!("decimal = {}", decimal);

    // let mut boleano = false;
    // println!("booleano = {}", boleano);
    // boleano = true;
    // println!("booleano = {}", boleano);

    // const PI: f64 = 3.14;
    // println!("valor de PI = {}", PI);

    // unsafe {
    //     TESTE = 888;
    //     println!("global statico = {}", TESTE);
    // }

    
    // println!("resultado da soma de {} + {} = {}",5,5, soma(5, 5));
    // println!("resultado da multiplicacao de {} + {} = {}",5,5, multiplicacao(5, 5));


    // let respo = false;
    // let idade: i64 = 19;

    // if idade > 18 || respo{
    //     println!("Pode entrar");
    // }else{
    //     println!("n pode entrar");
    // }


    // let condicao = if idade >= 18 {"maior"}else{"menor"};

    // println!("é {}",condicao);

    // tabuada(15);

    // match_pattern();

    // ownership();

    error();
}

fn soma(n: i64, x: i64) -> i64 {
    n + x
}


fn multiplicacao(a:i64, b:i64)->i64{
    return a * b;
}



fn tabuada(multiplicador:i64){
    let mut contador = 1;
    while contador <= 10{
        println!("{} x {} = {}",contador,multiplicador,(contador*multiplicador) );
        contador+=1;
    }
    
    
    println!("utilizando for");
    for i in 1..11{
        println!("{} x {} = {}",i,multiplicador,(i*multiplicador) );
    }
}



fn match_pattern(){
    let linguagem = "HTML";
    let proposito = match linguagem{
        "PHP"=>"web",
        "C#"=>"windows",
        "Kotlin"=>"android",
        "HTML"|"ajax"=>"frontend",
        _ =>"sei la"
    };

    println!("{}", proposito);
}

fn ownership(){
    let mut uma_string = String::from("Mateus");

    println!("{}",uma_string);
    change(&mut uma_string);
    
    println!("{}",uma_string);
}

fn change(string: &mut String){
    string.push_str("teste");
    println!("{}",string);
}


fn error(){
    match resultado() {
        Ok(s) => println!("Resultado sucesso {}",s),
        Err(n) => println!("Resultado erro {}",n),
    }
}


fn resultado()->Result<String,u8>{
    // Ok(String::from("Foi"))
    Err(2)
}