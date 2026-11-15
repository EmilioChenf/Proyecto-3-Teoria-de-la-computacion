use simulador_mt::configuracion::{Maquina, Simbolo};
use simulador_mt::motor::Terminacion;
use simulador_mt::presentacion::representar;

const BASE: &str = "q_states: {q_list: [inicio, fin], initial: inicio, final: fin}\nalphabet: ['a']\ntape_alphabet: ['a', null]\ndelta: []\nsimulation_strings: ['a']\n";
fn regla(
    estado: &str,
    memoria: &str,
    lectura: &str,
    nuevo: &str,
    nueva: &str,
    escritura: &str,
    movimiento: &str,
) -> String {
    format!(
        "  - params: {{initial_state: {estado}, mem_cache_value: {memoria}, tape_input: {lectura}}}\n    output: {{final_state: {nuevo}, mem_cache_value: {nueva}, tape_output: {escritura}, tape_displacement: {movimiento}}}\n"
    )
}
fn errores_de_configuracion() {
    for texto in [
        "q_states: [".into(),
        BASE.replace("alphabet: ['a']\n", ""),
        BASE.replace("initial: inicio", "initial: otro"),
        BASE.replace("final: fin", "final: otro"),
        BASE.replace("tape_alphabet: ['a', null]", "tape_alphabet: ['a']"),
        BASE.replace("alphabet: ['a']", "alphabet: ['b']"),
        BASE.replace("alphabet: ['a']", "alphabet: [null]"),
        BASE.replace("['a', null]", "['null', null]"),
        BASE.replace("['a', null]", "['', null]"),
        BASE.replace("['a', null]", "[12, null]"),
        BASE.replace("['a', null]", "[true, null]"),
        BASE.replace("['a', null]", "[[a], null]"),
        BASE.replace("[inicio, fin]", "[inicio, inicio, fin]"),
        BASE.replace("['a', null]", "['a', 'a', null]"),
    ] {
        assert!(
            Maquina::desde_yaml(&texto).is_err(),
            "Se aceptó configuración inválida: {texto}"
        );
    }
    let buena = regla("inicio", "null", "'a'", "fin", "null", "'a'", "R");
    for regla in [
        buena.replace("initial_state: inicio", "initial_state: otro"),
        buena.replace("final_state: fin", "final_state: otro"),
        buena.replace("tape_output: 'a'", "tape_output: '#'"),
        buena.replace("tape_input: 'a'", "tape_input: '#'"),
        buena.replace("displacement: R", "displacement: X"),
        format!("{buena}{buena}"),
        buena.replace("mem_cache_value: null, ", ""),
    ] {
        assert!(
            Maquina::desde_yaml(&BASE.replace("delta: []", &format!("delta:\n{regla}"))).is_err(),
            "Regla inválida aceptada: {regla}"
        );
    }
}
fn memoria_diferencia_transiciones_y_null_borra_registro() {
    let reglas = [
        regla("inicio", "null", "'a'", "inicio", "'x'", "null", "L"),
        regla("inicio", "'x'", "null", "inicio", "'y'", "'a'", "S"),
        regla("inicio", "'y'", "'a'", "derecha", "null", "null", "R"),
        regla("derecha", "null", "null", "derecha", "'z'", "'a'", "R"),
        regla("derecha", "'z'", "null", "fin", "null", "'a'", "S"),
    ]
    .concat();
    let texto = BASE
        .replace("[inicio, fin]", "[inicio, derecha, fin]")
        .replace("delta: []", &format!("delta:\n{reglas}"));
    let maquina = Maquina::desde_yaml(&texto).unwrap();
    let resultado = maquina.simular("a", 10).unwrap();
    assert_eq!(resultado.terminacion, Terminacion::Aceptada);
    assert_eq!(resultado.traza.len(), 6);
    assert_eq!(resultado.traza[1].cinta.posicion_cabezal, -1);
    assert_eq!(resultado.traza[2].cinta.posicion_cabezal, -1);
    assert_eq!(resultado.traza[3].memoria, Simbolo(None));
    let ultima = resultado.traza.last().unwrap();
    assert_eq!(ultima.cinta.posicion_cabezal, 1);
    assert_eq!(ultima.cinta.en(-1), Simbolo(None));
    assert_eq!(ultima.cinta.contenido_util(), "aa");
    assert_eq!(ultima.memoria, Simbolo(None));
    let texto = representar(&resultado);
    assert_eq!(texto.matches("ID: ").count(), 6);
    assert_eq!(texto.matches("Aplicada:").count(), 5);
}
fn blanco_vacio_yaml_y_unicode() {
    let texto = BASE
        .replace("'a'", "'ñ'")
        .replace("tape_alphabet: ['ñ', null]", "tape_alphabet:\n  - 'ñ'\n  -");
    let maquina = Maquina::desde_yaml(&texto).unwrap();
    let resultado = maquina.simular("ñ", 10).unwrap();
    assert_eq!(resultado.traza[0].cinta.maximo_visitado, 0);
    assert!(representar(&resultado).contains("longitud: 1 caracteres"));
    assert!(
        maquina
            .configuracion
            .alfabeto_cinta
            .contains(&Simbolo(None))
    );
}
fn cargar_maquina(nombre: &str) -> Maquina {
    Maquina::cargar(&format!("maquinas/{nombre}.yaml")).unwrap()
}
fn palabras(alfabeto: &[char], longitud: usize) -> Vec<String> {
    let mut todas = vec![String::new()];
    let mut nivel = todas.clone();
    for _ in 0..longitud {
        nivel = nivel
            .iter()
            .flat_map(|prefijo| {
                alfabeto
                    .iter()
                    .map(move |caracter| format!("{prefijo}{caracter}"))
            })
            .collect();
        todas.extend(nivel.clone());
    }
    todas
}
fn oraculo_reconocedor(cadena: &str) -> bool {
    let caracteres: Vec<_> = cadena.chars().collect();
    let longitud = caracteres.len();
    if longitud == 0 || !longitud.is_multiple_of(3) {
        return false;
    }
    let cantidad = longitud / 3;
    caracteres[..cantidad].iter().all(|&x| x == 'a')
        && caracteres[cantidad..2 * cantidad].iter().all(|&x| x == 'b')
        && caracteres[2 * cantidad..].iter().all(|&x| x == 'c')
}
fn reconocedora_exhaustiva_hasta_ocho() {
    let maquina = cargar_maquina("reconocedora");
    for cadena in palabras(&['a', 'b', 'c'], 8) {
        let resultado = maquina.simular(&cadena, 10000).unwrap();
        assert_ne!(resultado.terminacion, Terminacion::Inconclusa, "{cadena}");
        assert_eq!(
            resultado.terminacion == Terminacion::Aceptada,
            oraculo_reconocedor(&cadena),
            "{cadena}"
        );
    }
}
