use simulador_mt::configuracion::{Maquina, Simbolo};

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
