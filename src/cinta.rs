//! Cinta dispersa, infinita en ambas direcciones.
use crate::configuracion::{Movimiento, Simbolo};
use std::collections::BTreeMap;
#[derive(Clone, Debug)]
pub struct Cinta {
    pub celdas: BTreeMap<i64, char>,
    pub posicion_cabezal: i64,
    pub minimo_visitado: i64,
    pub maximo_visitado: i64,
}
impl Cinta {
    pub fn nueva(entrada: &str) -> Self {
        let celdas: BTreeMap<_, _> = entrada
            .chars()
            .enumerate()
            .map(|(indice, caracter)| (indice as i64, caracter))
            .collect();
        let maximo_visitado = celdas.keys().next_back().copied().unwrap_or(0);
        Self {
            celdas,
            posicion_cabezal: 0,
            minimo_visitado: 0,
            maximo_visitado,
        }
    }
    pub fn leer(&self) -> Simbolo {
        self.en(self.posicion_cabezal)
    }
    pub fn en(&self, posicion: i64) -> Simbolo {
        Simbolo(self.celdas.get(&posicion).copied())
    }
    pub fn escribir(&mut self, simbolo: Simbolo) {
        match simbolo.0 {
            Some(caracter) => {
                self.celdas.insert(self.posicion_cabezal, caracter);
            }
            None => {
                self.celdas.remove(&self.posicion_cabezal);
            }
        }
    }
    pub fn mover(&mut self, movimiento: Movimiento) {
        self.posicion_cabezal += match movimiento {
            Movimiento::L => -1,
            Movimiento::R => 1,
            Movimiento::S => 0,
        };
        self.minimo_visitado = self.minimo_visitado.min(self.posicion_cabezal);
        self.maximo_visitado = self.maximo_visitado.max(self.posicion_cabezal);
    }
    /// Recorta exclusivamente blancos exteriores; conserva los internos como □.
    pub fn contenido_util(&self) -> String {
        match (self.celdas.keys().next(), self.celdas.keys().next_back()) {
            (Some(&inicio), Some(&fin)) => (inicio..=fin)
                .map(|posicion| self.en(posicion).0.unwrap_or('□'))
                .collect(),
            _ => String::new(),
        }
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    #[test]
    fn conserva_blancos_internos_y_extiende_ambos_lados() {
        let mut cinta = Cinta::nueva("abc");
        cinta.mover(Movimiento::R);
        cinta.escribir(Simbolo(None));
        assert_eq!(cinta.contenido_util(), "a□c");
        cinta.mover(Movimiento::L);
        cinta.mover(Movimiento::L);
        cinta.escribir(Simbolo(Some('x')));
        assert_eq!(cinta.posicion_cabezal, -1);
        assert_eq!(cinta.contenido_util(), "xa□c");
        for _ in 0..5 {
            cinta.mover(Movimiento::R);
        }
        assert_eq!(cinta.leer(), Simbolo(None));
        cinta.escribir(Simbolo(Some('z')));
        assert_eq!(cinta.contenido_util(), "xa□c□z");
    }
}
