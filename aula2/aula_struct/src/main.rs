
struct Conta {
    Titular: TitularConta,
    Saldo: f32
}

struct TitularConta {
    Nome:String,
    Idade:u8
}

impl Conta {
    fn decimo_terceiro(self)->f32{
        return self.Saldo/2.0;
    }
}

fn main() {
    let titular: TitularConta = TitularConta {
        Nome: String::from("teste"),
        Idade: 1
    };
    let conta: Conta = Conta {
        Titular: titular,
        Saldo: 200.0
    };

    println!("Nome {}", conta.Titular.Nome);
    println!("Idade {}", conta.Titular.Idade);
    println!("Saldo {}", conta.Saldo);
    println!("Decimo {}", conta.decimo_terceiro());
}
