const PI:f32 = 3.14;
static mut GLOBAL:u8 = 1;

fn soma(a:i32, b:i32) -> i32
{
    println!("{} + {} = {}", a, b, a + b);
    a + b
}

fn sombra() {
    let a = 123;

    {
        let b = 456;
        println!("dentro, b = {}", b);

        let a = 777;
        println!("dentro, a = {}", a);
    }

    println!("fora, a = {}", a);
}

fn escopo() {
    println!("PI = {}", PI);

    unsafe {
        println!("variavel_global = {}", GLOBAL);
    }

    let variavel:i32 = 300;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));
    let variavel:i32 = 301;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("decimal = {}", decimal);

    let booleana:bool = true;
    println!("Booleana = {}, Tamanho booleana = {}", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("Tamanho do char = {}", std::mem::size_of_val(&letra));
}

fn condicionais() {
    
    let idade: u8 = 18;
    let responsavel_autorizou = true;
    let maior_idade = idade >= 18;

    if maior_idade {
        println!("Pode entrar na balada");
    } else if idade > 16 && responsavel_autorizou {
        println!("Pode entrar com a assinatura do reponsável");
    } else {
        println!("Não pode entrar na balada");
    }

    let condicao = if maior_idade { "Maior"} else { "Menor"};

    println!("È {} de idade", condicao);

    let linguagem = "PHP";
    let proposito = match linguagem {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Desconhecido"
    };

        println!("O proposito de {} é {}", linguagem, proposito);
}

fn repeticoes() {
    let  multiplicador:u8 = 1;
    let mut contador:u8 = 0;

    while contador < 10 {
        contador += 1;

        if contador == 5 {
            continue;
        }
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    }

    contador = 0;
    loop {

        contador += 1; 

        println!("{} X {} = {}", multiplicador, contador, multiplicador * contador);
        
        if contador == 10 {
            break;
        }
    }

    for i in 1..11 {
        println!("{} x {} = {}", multiplicador, i, multiplicador * i);
    }

}

fn ownership() {
    let uma_string = String::from("Vinicius");
    rouba(uma_string);

    println!("{}", uma_string);
} 

fn rouba(String: String) {
    println!("{}", String);
}

fn main() {
    escopo();
    sombra();
    ownership();

    println!("Soma = {}", soma(2, 2));
    // println!("decimal = {}", decimal);
   
    condicionais();
    repeticoes();
}
