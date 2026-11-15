//! Representación íntegra de las descripciones instantáneas.
use crate::motor::Resultado;
use std::fmt::Write;
pub fn representar(resultado: &Resultado) -> String {
    let mut texto = format!(
        "Entrada: {:?} (longitud: {} caracteres)\n",
        resultado.entrada,
        resultado.entrada.chars().count()
    );
    for actual in &resultado.traza {
        writeln!(
            texto,
            "Paso {} | estado={} | memoria={} | cabezal={}",
            actual.paso, actual.estado, actual.memoria, actual.cinta.posicion_cabezal
        )
        .unwrap();
        if let Some(transicion) = &actual.aplicada {
            writeln!(
                texto,
                "Aplicada: δ([{},{}],{}) = ([{},{}],{},{:?})",
                transicion.parametros.estado,
                transicion.parametros.memoria,
                transicion.parametros.lectura,
                transicion.salida.estado,
                transicion.salida.memoria,
                transicion.salida.escritura,
                transicion.salida.movimiento
            )
            .unwrap();
        }
        texto.push_str("ID: ");
        for posicion in actual.cinta.minimo_visitado - 1..=actual.cinta.maximo_visitado + 1 {
            if posicion == actual.cinta.posicion_cabezal {
                write!(texto, "⟨{},{}⟩ ", actual.estado, actual.memoria).unwrap();
            }
            write!(texto, "{posicion}:{} ", actual.cinta.en(posicion)).unwrap();
        }
        texto.push('\n');
    }
    let ultima = resultado.traza.last().unwrap();
    writeln!(texto, "Resultado: {:?}; motivo: {}\nCinta final útil (solo sin blancos exteriores): {:?}\nTransiciones ejecutadas: {}\n", resultado.terminacion, resultado.motivo, ultima.cinta.contenido_util(), ultima.paso).unwrap();
    texto
}
