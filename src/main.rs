use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    let (separador, file_args) = parse_args(&args)?;
    
    if file_args.len() < 2 {
        instrucciones();
        return Err("Faltan argumentos: combina [OPCIONES] <ficheroprefijos> <ficherosufijos>".into());
    }

    let ficheroprefijos = File::open(&file_args[0]).map_err(|_| format!("Error abriendo ficheroprefijos: {}", &file_args[0]))?;
    let mut ficherosufijos = File::open(&file_args[1]).map_err(|_| format!("Error abriendo ficherosufijos: {}", &file_args[1]))?;

    let prefijos = io::BufReader::new(ficheroprefijos);

    let sufijos_lines: Vec<String> = {
        let sufijos = io::BufReader::new(&mut ficherosufijos);
        sufijos.lines().collect::<Result<_, _>>()?
    };

    for prefijo in prefijos.lines() {
        let prefijo = prefijo?;

        for sufijo in &sufijos_lines {
            println!("{}{}{}", prefijo, separador, sufijo);
        }
    }

    Ok(())
}

fn parse_args(args: &[String]) -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {
    let mut separador = "".to_string();
    let mut file_args = vec![];

    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-s" | "--separador" => {
                if let Some(sep) = iter.next() {
                    separador = sep.clone();
                } else {
                    return Err("Se esperaba un valor para la opción -s/--separador".into());
                }
            }
            _ => file_args.push(arg.clone()),
        }
    }

    Ok((separador, file_args))
}

fn instrucciones() {
    eprintln!("Combina las líneas de dos archivos en cada combinación.\n");
    eprintln!("Instrucciones:");
    eprintln!("  combina [OPCIONES] <ficheroprefijos> <ficherosufijos>\n");
    eprintln!("Opciones:");
    eprintln!("  -s, --separador <cad>  Separador entre el prefijo y el sufijo\n");
}
