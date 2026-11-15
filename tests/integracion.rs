use simulador_mt::{
    configuracion::{Maquina, Simbolo},
    motor::Terminacion,
    presentacion::representar,
};
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
#[test]
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
#[test]
fn alteradora_exhaustiva_hasta_siete() {
    let maquina = cargar_maquina("alteradora");
    for cadena in palabras(&['a', 'b'], 7) {
        let resultado = maquina.simular(&cadena, 10000).unwrap();
        if cadena.is_empty() {
            assert_eq!(resultado.terminacion, Terminacion::Rechazada);
            continue;
        }
        assert_eq!(resultado.terminacion, Terminacion::Aceptada, "{cadena}");
        let cinta = &resultado.traza.last().unwrap().cinta;
        assert_eq!(cinta.contenido_util(), format!("{cadena}#{cadena}"));
        assert!(
            cinta
                .celdas
                .values()
                .all(|caracter| "ab#".contains(*caracter))
        );
    }
}
#[test]
fn ocho_demostraciones_e_independencia() {
    for (nombre, entradas) in [
        (
            "reconocedora",
            vec!["aabbcc", "aaabbbccc", "aabbbcc", "aaabbccc"],
        ),
        ("alteradora", vec!["aabab", "abbaa", "babab", "bbaab"]),
    ] {
        let maquina = cargar_maquina(nombre);
        for (indice, cadena) in entradas.iter().enumerate() {
            let resultado = maquina.simular(cadena, 10000).unwrap();
            let esperado = if nombre == "reconocedora" && indice >= 2 {
                Terminacion::Rechazada
            } else {
                Terminacion::Aceptada
            };
            assert_eq!(resultado.terminacion, esperado);
            assert_eq!(resultado.traza[0].paso, 0);
            assert_eq!(resultado.traza[0].memoria, Simbolo(None));
            assert_eq!(resultado.traza[0].cinta.posicion_cabezal, 0);
            assert_eq!(
                resultado.traza.len(),
                resultado.traza.last().unwrap().paso + 1
            );
            for (paso, actual) in resultado.traza.iter().enumerate() {
                assert_eq!(paso, actual.paso);
            }
            if nombre == "alteradora" {
                assert_eq!(
                    resultado.traza.last().unwrap().cinta.contenido_util(),
                    format!("{cadena}#{cadena}")
                );
            }
            assert_eq!(
                representar(&resultado),
                representar(&maquina.simular(cadena, 10000).unwrap())
            );
        }
    }
}
const BASE: &str = "q_states: {q_list: [inicio, fin], initial: inicio, final: fin}\nalphabet: ['a']\ntape_alphabet: ['a', null]\ndelta: []\nsimulation_strings: ['a']\n";
#[test]
fn aceptacion_inicial_rechazo_limite_y_entrada_invalida() {
    let maquina = Maquina::desde_yaml(BASE).unwrap();
    assert_eq!(
        maquina.simular("a", 0).unwrap().terminacion,
        Terminacion::Rechazada
    );
    assert!(
        maquina
            .simular("b", 10)
            .unwrap_err()
            .contains("Cadena inválida")
    );
    let inicial = Maquina::desde_yaml(&BASE.replace("initial: inicio", "initial: fin")).unwrap();
    let resultado = inicial.simular("", 0).unwrap();
    assert_eq!(resultado.terminacion, Terminacion::Aceptada);
    assert_eq!(resultado.traza.len(), 1);
    let maquina = cargar_maquina("reconocedora");
    let resultado = maquina.simular("abc", 0).unwrap();
    assert_eq!(resultado.terminacion, Terminacion::Inconclusa);
    assert_eq!(resultado.traza.len(), 1);
    let completa = maquina.simular("abc", 1000).unwrap();
    let pasos = completa.traza.last().unwrap().paso;
    assert_eq!(
        maquina.simular("abc", pasos).unwrap().terminacion,
        Terminacion::Aceptada
    );
    assert_eq!(
        maquina.simular("abc", pasos - 1).unwrap().terminacion,
        Terminacion::Inconclusa
    );
}
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
#[test]
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
#[test]
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
#[test]
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
#[test]
fn cli_exporta_trazas_completas_y_distingue_errores() {
    let ruta = std::env::temp_dir().join(format!("traza-mt-{}.txt", std::process::id()));
    let salida = std::process::Command::new(env!("CARGO_BIN_EXE_simulador-mt"))
        .args(["maquinas/reconocedora.yaml", "--entrada", "abc", "--traza"])
        .arg(&ruta)
        .output()
        .unwrap();
    assert!(salida.status.success());
    assert_eq!(salida.stdout, std::fs::read(&ruta).unwrap());
    std::fs::remove_file(ruta).unwrap();
    let salida = std::process::Command::new(env!("CARGO_BIN_EXE_simulador-mt"))
        .args(["maquinas/reconocedora.yaml", "--entrada", "#"])
        .output()
        .unwrap();
    assert!(!salida.status.success());
    assert!(
        String::from_utf8(salida.stdout)
            .unwrap()
            .contains("Cadena inválida")
    );
}

#[test]
fn campos_blancos_vacios_son_obligatorios_y_no_comodines() {
    let reglas = "  - params:\n      initial_state: inicio\n      mem_cache_value:\n      tape_input:\n    output:\n      final_state: fin\n      mem_cache_value:\n      tape_output:\n      tape_displacement: S\n";
    let texto = BASE.replace("delta: []", &format!("delta:\n{reglas}"));
    let maquina = Maquina::desde_yaml(&texto).unwrap();
    assert_eq!(
        maquina.simular("", 1).unwrap().terminacion,
        Terminacion::Aceptada
    );
    assert_eq!(
        maquina.simular("a", 1).unwrap().terminacion,
        Terminacion::Rechazada
    );
    for campo in [
        "      mem_cache_value:\n",
        "      tape_input:\n",
        "      tape_output:\n",
    ] {
        assert!(
            Maquina::desde_yaml(&texto.replacen(campo, "", 1)).is_err(),
            "Se permitió omitir {campo}"
        );
    }
    assert!(Maquina::desde_yaml(&texto.replace("tape_input:", "tape_input: 'null'")).is_err());
}

#[test]
fn ejemplos_adicionales_reconocedores_y_separador_no_es_blanco() {
    let reconocedora = cargar_maquina("reconocedora");
    for (cadena, esperado) in [
        ("", false),
        ("abc", true),
        ("acb", false),
        ("bac", false),
        ("aabbc", false),
        ("abbc", false),
        ("aabbccc", false),
    ] {
        assert_eq!(
            reconocedora.simular(cadena, 1000).unwrap().terminacion == Terminacion::Aceptada,
            esperado,
            "{cadena}"
        );
    }
    let alteradora = cargar_maquina("alteradora");
    assert!(alteradora.simular("a#b", 1000).is_err());
    let resultado = alteradora.simular("a", 1000).unwrap();
    assert_eq!(
        resultado.traza.last().unwrap().cinta.en(1),
        Simbolo(Some('#'))
    );
    assert_eq!(resultado.traza.last().unwrap().cinta.en(3), Simbolo(None));
}
