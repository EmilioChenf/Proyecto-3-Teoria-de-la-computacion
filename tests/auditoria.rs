//! Casos independientes y regresiones de la auditoría técnica.
use simulador_mt::{
    configuracion::{Maquina, Simbolo},
    motor::Terminacion,
};
use std::process::Command;

const GENERAL: &str = "q_states: {q_list: [inicio, derecha, fin, trampa], initial: inicio, final: fin}\nalphabet: ['α']\ntape_alphabet: ['α', 'β', '#', null]\ndelta:\n  - params: {initial_state: inicio, mem_cache_value: null, tape_input: 'α'}\n    output: {final_state: inicio, mem_cache_value: x, tape_output: null, tape_displacement: L}\n  - params: {initial_state: inicio, mem_cache_value: x, tape_input: null}\n    output: {final_state: inicio, mem_cache_value: y, tape_output: '#', tape_displacement: S}\n  - params: {initial_state: inicio, mem_cache_value: y, tape_input: '#'}\n    output: {final_state: derecha, mem_cache_value: null, tape_output: null, tape_displacement: R}\n  - params: {initial_state: derecha, mem_cache_value: null, tape_input: null}\n    output: {final_state: derecha, mem_cache_value: z, tape_output: 'α', tape_displacement: R}\n  - params: {initial_state: derecha, mem_cache_value: z, tape_input: null}\n    output: {final_state: fin, mem_cache_value: null, tape_output: 'β', tape_displacement: S}\n  - params: {initial_state: inicio, mem_cache_value: null, tape_input: null}\n    output: {final_state: trampa, mem_cache_value: null, tape_output: null, tape_displacement: S}\nsimulation_strings: ['α', 'α']\n";

#[test]
fn maquina_independiente_coincide_con_calculo_manual_paso_a_paso() {
    let maquina = Maquina::desde_yaml(GENERAL).unwrap();
    let esperado = [
        ("inicio", None, 0, vec![(0, 'α')]),
        ("inicio", Some('x'), -1, vec![]),
        ("inicio", Some('y'), -1, vec![(-1, '#')]),
        ("derecha", None, 0, vec![]),
        ("derecha", Some('z'), 1, vec![(0, 'α')]),
        ("fin", None, 1, vec![(0, 'α'), (1, 'β')]),
    ];
    let resultado = maquina.simular("α", 5).unwrap();
    assert_eq!(resultado.terminacion, Terminacion::Aceptada);
    assert_eq!(resultado.traza.len(), esperado.len());
    for (paso, (estado, memoria, cabezal, celdas)) in esperado.iter().enumerate() {
        let actual = &resultado.traza[paso];
        assert_eq!(actual.paso, paso);
        assert_eq!(&actual.estado, estado);
        assert_eq!(actual.memoria, Simbolo(*memoria));
        assert_eq!(actual.cinta.posicion_cabezal, *cabezal);
        assert_eq!(actual.cinta.celdas, celdas.iter().copied().collect());
        if paso > 0 {
            let anterior = &resultado.traza[paso - 1];
            let aplicada = actual.aplicada.as_ref().unwrap();
            assert_eq!(aplicada.parametros.estado, anterior.estado);
            assert_eq!(aplicada.parametros.memoria, anterior.memoria);
            assert_eq!(aplicada.parametros.lectura, anterior.cinta.leer());
            assert_eq!(
                actual.cinta.en(anterior.cinta.posicion_cabezal),
                aplicada.salida.escritura
            );
        }
    }
}

#[test]
fn fronteras_exactas_del_limite_y_memoria_nula_no_es_comodin() {
    let maquina = Maquina::desde_yaml(GENERAL).unwrap();
    for limite in 0..=6 {
        let resultado = maquina.simular("α", limite).unwrap();
        assert_eq!(resultado.traza.len(), limite.min(5) + 1);
        assert_eq!(
            resultado.terminacion,
            if limite < 5 {
                Terminacion::Inconclusa
            } else {
                Terminacion::Aceptada
            }
        );
    }
    let incompleta = GENERAL.replace(
        "mem_cache_value: x, tape_input: null",
        "mem_cache_value: w, tape_input: null",
    );
    let resultado = Maquina::desde_yaml(&incompleta)
        .unwrap()
        .simular("α", 10)
        .unwrap();
    assert_eq!(resultado.terminacion, Terminacion::Rechazada);
    assert_eq!(resultado.traza.last().unwrap().paso, 1);
}

#[test]
fn maquinas_terminan_y_restauran_en_entradas_mayores() {
    let reconocedora = Maquina::cargar("maquinas/reconocedora.yaml").unwrap();
    for cantidad in [1, 2, 3, 4, 8, 16] {
        let cadena = format!(
            "{}{}{}",
            "a".repeat(cantidad),
            "b".repeat(cantidad),
            "c".repeat(cantidad)
        );
        let limite = 10 * cadena.chars().count().pow(2) + 10;
        let resultado = reconocedora.simular(&cadena, limite).unwrap();
        assert_eq!(resultado.terminacion, Terminacion::Aceptada);
    }
    let alteradora = Maquina::cargar("maquinas/alteradora.yaml").unwrap();
    for cadena in [
        "a".repeat(16),
        "b".repeat(16),
        "ab".repeat(16),
        "bbaaabab".repeat(4),
    ] {
        let resultado = alteradora
            .simular(&cadena, 10 * (cadena.chars().count() + 1).pow(2))
            .unwrap();
        assert_eq!(resultado.terminacion, Terminacion::Aceptada);
        assert_eq!(
            resultado.traza.last().unwrap().cinta.contenido_util(),
            format!("{cadena}#{cadena}")
        );
    }
}

#[test]
fn exportacion_no_destruye_el_archivo_yaml_de_entrada() {
    let carpeta =
        std::env::temp_dir().join(format!("auditoria-proteger-yaml-{}", std::process::id()));
    std::fs::create_dir_all(&carpeta).unwrap();
    let ruta = carpeta.join("maquina.yaml");
    std::fs::write(&ruta, GENERAL).unwrap();
    let equivalente = carpeta.join(".").join("maquina.yaml");
    let salida = Command::new(env!("CARGO_BIN_EXE_simulador-mt"))
        .arg(&ruta)
        .arg("--traza")
        .arg(&equivalente)
        .output()
        .unwrap();
    let conservado = std::fs::read(&ruta).unwrap();
    std::fs::remove_dir_all(carpeta).unwrap();
    assert!(
        !salida.status.success(),
        "Debe rechazar exportar sobre su configuración"
    );
    assert_eq!(
        conservado,
        GENERAL.as_bytes(),
        "El YAML debe permanecer intacto"
    );
    assert!(
        String::from_utf8(salida.stderr)
            .unwrap()
            .contains("mismo archivo")
    );
}

#[cfg(unix)]
#[test]
fn canal_de_salida_cerrado_informa_error_sin_panico() {
    use std::process::Stdio;
    let mut proceso = Command::new(env!("CARGO_BIN_EXE_simulador-mt"))
        .arg("maquinas/alteradora.yaml")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    drop(proceso.stdout.take());
    let salida = proceso.wait_with_output().unwrap();
    assert_eq!(salida.status.code(), Some(1));
    let error = String::from_utf8(salida.stderr).unwrap();
    assert!(!error.contains("panicked"));
    assert!(error.contains("salida estándar"));
}

#[test]
fn exportacion_reconoce_enlaces_fisicos_al_yaml_original() {
    let carpeta =
        std::env::temp_dir().join(format!("auditoria-enlace-yaml-{}", std::process::id()));
    std::fs::create_dir_all(&carpeta).unwrap();
    let origen = carpeta.join("maquina.yaml");
    let enlace = carpeta.join("enlace.yaml");
    std::fs::write(&origen, GENERAL).unwrap();
    std::fs::hard_link(&origen, &enlace).unwrap();
    let salida = Command::new(env!("CARGO_BIN_EXE_simulador-mt"))
        .arg(&origen)
        .arg("--traza")
        .arg(&enlace)
        .output()
        .unwrap();
    let conservado = std::fs::read(&origen).unwrap();
    std::fs::remove_dir_all(carpeta).unwrap();
    assert!(!salida.status.success());
    assert_eq!(conservado, GENERAL.as_bytes());
    assert!(
        String::from_utf8(salida.stderr)
            .unwrap()
            .contains("mismo archivo")
    );
}
