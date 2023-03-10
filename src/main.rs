use std::env;
mod lexical;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argumento = args[1].clone();

    let cadena: String = String::from(argumento);
    let mut analizer: lexical::LexicalAnalysis = lexical::LexicalAnalysis::new(&cadena);
    analizer.analize();
    println!("La cadena {} es de tipo: {}", cadena, analizer.get_type());
    
}
