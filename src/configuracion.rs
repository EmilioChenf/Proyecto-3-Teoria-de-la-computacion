//! Esquema YAML y validación del modelo formal.
use serde::{Deserialize, Deserializer};

/// Un símbolo es un carácter Unicode o el blanco YAML null.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Simbolo(pub Option<char>);
impl<'de> Deserialize<'de> for Simbolo {
    fn deserialize<D: Deserializer<'de>>(deserializador: D) -> Result<Self, D::Error> {
        let valor = Option::<String>::deserialize(deserializador)?;
        match valor {
            None => Ok(Self(None)),
            Some(texto) => {
                let mut caracteres = texto.chars();
                match (caracteres.next(), caracteres.next()) {
                    (Some(caracter), None) => Ok(Self(Some(caracter))),
                    _ => Err(serde::de::Error::custom(
                        "símbolo no admitido: use un carácter Unicode o null",
                    )),
                }
            }
        }
    }
}
fn simbolo_obligatorio<'de, D: Deserializer<'de>>(deserializador: D) -> Result<Simbolo, D::Error> {
    Simbolo::deserialize(deserializador)
}
impl std::fmt::Display for Simbolo {
    fn fmt(&self, formato: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(caracter) => write!(formato, "{caracter:?}"),
            None => write!(formato, "□"),
        }
    }
}
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum Movimiento {
    L,
    R,
    S,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Estados {
    #[serde(rename = "q_list")]
    pub lista: Vec<String>,
    #[serde(rename = "initial")]
    pub inicial: String,
    #[serde(rename = "final")]
    pub final_: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Parametros {
    #[serde(rename = "initial_state")]
    pub estado: String,
    #[serde(rename = "mem_cache_value", deserialize_with = "simbolo_obligatorio")]
    pub memoria: Simbolo,
    #[serde(rename = "tape_input", deserialize_with = "simbolo_obligatorio")]
    pub lectura: Simbolo,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Salida {
    #[serde(rename = "final_state")]
    pub estado: String,
    #[serde(rename = "mem_cache_value", deserialize_with = "simbolo_obligatorio")]
    pub memoria: Simbolo,
    #[serde(rename = "tape_output", deserialize_with = "simbolo_obligatorio")]
    pub escritura: Simbolo,
    #[serde(rename = "tape_displacement")]
    pub movimiento: Movimiento,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Transicion {
    #[serde(rename = "params")]
    pub parametros: Parametros,
    #[serde(rename = "output")]
    pub salida: Salida,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Configuracion {
    #[serde(rename = "q_states")]
    pub estados: Estados,
    #[serde(rename = "alphabet")]
    pub alfabeto: Vec<Simbolo>,
    #[serde(rename = "tape_alphabet")]
    pub alfabeto_cinta: Vec<Simbolo>,
    #[serde(rename = "delta")]
    pub transiciones: Vec<Transicion>,
    #[serde(rename = "simulation_strings")]
    pub cadenas: Vec<String>,
}
