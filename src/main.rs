use simulador_mt::{configuracion::Maquina, presentacion::representar};
fn ejecutar() -> Result<(), String> {
    let mut argumentos = std::env::args().skip(1);
    let ruta = argumentos.next().ok_or(
        "Uso: simulador-mt ARCHIVO.yaml [--limite N] [--entrada CADENA] [--traza ARCHIVO]",
    )?;
    if ruta == "--help" || ruta == "--ayuda" {
        println!(
            "Uso: simulador-mt ARCHIVO.yaml [--limite N] [--entrada CADENA] [--traza ARCHIVO]\nSin --entrada se ejecutan todas las simulation_strings. La traza siempre es completa."
        );
        return Ok(());
    }
    let mut limite = 100_000;
    let mut entrada = None;
    let mut archivo = None;
    while let Some(opcion) = argumentos.next() {
        let valor = argumentos
            .next()
            .ok_or_else(|| format!("Falta valor para {opcion}"))?;
        match opcion.as_str() {
            "--limite" => {
                limite = valor
                    .parse()
                    .map_err(|_| "Límite inválido: use un entero no negativo")?
            }
            "--entrada" => entrada = Some(valor),
            "--traza" => archivo = Some(valor),
            _ => return Err(format!("Opción desconocida: {opcion}")),
        }
    }
    let maquina = Maquina::cargar(&ruta)?;
    let cadenas = entrada
        .map(|valor| vec![valor])
        .unwrap_or_else(|| maquina.configuracion.cadenas.clone());
    let mut salida = String::new();
    let mut invalida = false;
    for cadena in cadenas {
        match maquina.simular(&cadena, limite) {
            Ok(resultado) => salida.push_str(&representar(&resultado)),
            Err(error) => {
                salida.push_str(&format!("Entrada {cadena:?}: {error}\n"));
                invalida = true;
            }
        }
    }
    if let Some(ruta) = archivo {
        std::fs::write(&ruta, &salida)
            .map_err(|error| format!("No se pudo exportar {ruta}: {error}"))?;
    }
    print!("{salida}");
    if invalida {
        return Err("Se encontraron cadenas inválidas".into());
    }
    Ok(())
}
fn main() {
    if let Err(error) = ejecutar() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}
