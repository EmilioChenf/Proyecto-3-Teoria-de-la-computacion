//! Ejecución general: no conoce los lenguajes de las demostraciones.
use crate::{
    cinta::Cinta,
    configuracion::{Maquina, Parametros, Simbolo, Transicion},
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Terminacion {
    Aceptada,
    Rechazada,
    Inconclusa,
}
#[derive(Clone, Debug)]
pub struct Instantanea {
    pub paso: usize,
    pub estado: String,
    pub memoria: Simbolo,
    pub cinta: Cinta,
    /// Transición que produjo esta configuración; ninguna en la inicial.
    pub aplicada: Option<Transicion>,
}
#[derive(Debug)]
pub struct Resultado {
    pub entrada: String,
    pub traza: Vec<Instantanea>,
    pub terminacion: Terminacion,
    pub motivo: String,
}
impl Maquina {
    pub fn simular(&self, entrada: &str, limite: usize) -> Result<Resultado, String> {
        for caracter in entrada.chars() {
            if !self
                .configuracion
                .alfabeto
                .contains(&Simbolo(Some(caracter)))
            {
                return Err(format!(
                    "Cadena inválida: símbolo {caracter:?} fuera del alfabeto de entrada"
                ));
            }
        }
        let mut actual = Instantanea {
            paso: 0,
            estado: self.configuracion.estados.inicial.clone(),
            memoria: Simbolo(None),
            cinta: Cinta::nueva(entrada),
            aplicada: None,
        };
        let mut traza = vec![actual.clone()];
        let (terminacion, motivo) = loop {
            if actual.estado == self.configuracion.estados.final_ {
                break (Terminacion::Aceptada, "estado final alcanzado");
            }
            let parametros = Parametros {
                estado: actual.estado.clone(),
                memoria: actual.memoria,
                lectura: actual.cinta.leer(),
            };
            let Some(salida) = self.tabla.get(&parametros) else {
                break (
                    Terminacion::Rechazada,
                    "no existe transición aplicable fuera del estado final",
                );
            };
            if actual.paso == limite {
                break (
                    Terminacion::Inconclusa,
                    "límite de pasos alcanzado / resultado inconcluso",
                );
            }
            actual.cinta.escribir(salida.escritura);
            actual.cinta.mover(salida.movimiento);
            actual.estado = salida.estado.clone();
            actual.memoria = salida.memoria;
            actual.paso += 1;
            actual.aplicada = Some(Transicion {
                parametros,
                salida: salida.clone(),
            });
            traza.push(actual.clone());
        };
        Ok(Resultado {
            entrada: entrada.into(),
            traza,
            terminacion,
            motivo: motivo.into(),
        })
    }
}
