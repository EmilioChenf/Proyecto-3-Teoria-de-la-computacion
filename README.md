# Proyecto 3 — Simulador configurable de máquinas de Turing

Simulador determinista de **una cinta en Rust**, con registro finito auxiliar, YAML estricto, cinta bidireccional, trazas completas y CLI en español. Las dos máquinas se ejecutan exclusivamente por sus tablas YAML: reconocedora de `{aⁿbⁿcⁿ | n≥1}` y alteradora `w → w#w` sobre `{a,b}⁺`.

Implementación y verificaciones locales realizadas. Pendientes de entrega académica: revisión de integrantes, confirmación de privacidad y publicación remota, colaboradores, Canvas y grabación/publicación del video no listado. Los diez commits están autorizados; consultar el resultado del push en el informe final de la sesión. No se ha publicado video.

## Instalación

Se necesita Rust **estable** con Cargo, rustfmt y clippy. `rust-toolchain.toml` selecciona stable; la ejecución verificada utilizó Rust/Cargo 1.100.0. `Cargo.lock` fija las dependencias, incluido serde-saphyr 2.1.1. No hay servicios, variables secretas ni Python como requisito de ejecución.

### Linux

Desde la carpeta que contiene el repositorio:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/instalar-rust.sh
sh /tmp/instalar-rust.sh -y --profile minimal --default-toolchain stable
. "$HOME/.cargo/env"
cd Proyecto-3-Teoria-de-la-computacion
rustup component add rustfmt clippy
cargo build --release --locked
cargo test --locked
```

Se necesita un enlazador del sistema (por ejemplo `cc`); si falta, instalar las herramientas de compilación de la distribución. En el entorno de esta sesión Rust reside en `/workspace/.cargo` y rustup en `/workspace/.rustup`; inicializar cada terminal con:

```bash
export CARGO_HOME=/workspace/.cargo
export RUSTUP_HOME=/workspace/.rustup
export PATH=/workspace/.cargo/bin:$PATH
cd /workspace/Proyecto-3-Teoria-de-la-computacion
```

### Windows PowerShell

Instalar Rust con el instalador oficial de <https://rustup.rs/> o con winget:

```powershell
winget install --exact --id Rustlang.Rustup
```

Completar las indicaciones del instalador para las herramientas C++ de Visual Studio (carga **Desarrollo para el escritorio con C++**, enlazador MSVC y Windows SDK). Abrir una nueva PowerShell y, desde la carpeta que contiene el repositorio:

```powershell
cd Proyecto-3-Teoria-de-la-computacion
rustup default stable
rustup component add rustfmt clippy
cargo build --release --locked
cargo test --locked
```

Windows no estuvo disponible en esta sesión: sus comandos y script están preparados, pero no se declaran probados en ese sistema.

## Ejecución

Desde la raíz del proyecto, estos comandos Cargo sirven en Linux y PowerShell:

```text
cargo run --release --locked -- maquinas/reconocedora.yaml
cargo run --release --locked -- maquinas/alteradora.yaml
cargo run --release --locked -- maquinas/reconocedora.yaml --entrada aabbcc
cargo run --release --locked -- maquinas/reconocedora.yaml --entrada aabbbcc
cargo run --release --locked -- maquinas/reconocedora.yaml --entrada abc --limite 1
cargo run --release --locked -- maquinas/alteradora.yaml --entrada aabab --traza evidencias/ejemplo.txt
```

Los dos primeros procesan las ocho entradas de los YAML. Los siguientes ejemplifican aceptación, rechazo, resultado inconcluso y exportación. La carpeta destino de `--traza` debe existir. Se conservan todas las IDs tanto en consola como en archivo. Predeterminado: 100.000 transiciones por entrada. Un límite no es un rechazo.

Para ejecutar directamente el binario compilado:

```bash
# Linux
./target/release/simulador-mt maquinas/reconocedora.yaml --traza evidencias/reconocedora.txt
./target/release/simulador-mt maquinas/alteradora.yaml --traza evidencias/alteradora.txt
```

```powershell
# Windows PowerShell
.\target\release\simulador-mt.exe maquinas/reconocedora.yaml --traza evidencias/reconocedora.txt
.\target\release\simulador-mt.exe maquinas/alteradora.yaml --traza evidencias/alteradora.txt
```

`--ayuda` muestra la sintaxis. Configuración inválida, error de archivo/opción o entrada fuera de Σ produce código de salida 1. Aceptación, rechazo e inconclusión son resultados de simulación y producen código 0; se distinguen en el texto. Cada cadena se procesa independientemente, incluso si otra es inválida.

## YAML y visualización

Para una máquina nueva, preparar un YAML con `q_states` (`q_list`, `initial`, `final`), `alphabet`, `tape_alphabet`, `delta` y `simulation_strings`. Cada regla lleva `params` (`initial_state`, `mem_cache_value`, `tape_input`) y `output` (`final_state`, `mem_cache_value`, `tape_output`, `tape_displacement`). Véanse los YAML completos en [maquinas](maquinas/).

Los símbolos son **un carácter Unicode**; las longitudes cuentan caracteres, no bytes ni grafemas. El blanco es YAML `null` o una entrada vacía, nunca la cadena `"null"` ni `#`. Se admiten L/R/S. La memoria empieza blanca, participa en la clave de transición y se actualiza siempre; una salida null la borra. Es un registro finito incorporable al estado, no otra cinta.

La ID formal es `u⟨estado,memoria⟩v`, con el cabezal sobre la primera celda de v. Cada celda muestra su posición entera, incluso negativa; `□` significa blanco y un carácter entre comillas es no blanco. Se muestra el intervalo visitado completo más un blanco exterior por lado. La transición del paso k es la que produjo esa ID. La última ID conserva la cinta relevante; el resumen útil elimina solo blancos exteriores, sin ocultar símbolos internos ni marcadores. Detalles en [arquitectura](docs/arquitectura.md).

## Resultados reales

| Máquina | Entrada | Resultado | Cinta útil final | Pasos |
|---|---|---|---|---:|
| Reconocedora | aabbcc | Aceptada | XXYYZZ | 29 |
| Reconocedora | aaabbbccc | Aceptada | XXXYYYZZZ | 58 |
| Reconocedora | aabbbcc | Rechazada | XXYYbZZ | 30 |
| Reconocedora | aaabbccc | Rechazada | XXXYYZZc | 31 |
| Alteradora | aabab | Aceptada | aabab#aabab | 114 |
| Alteradora | abbaa | Aceptada | abbaa#abbaa | 114 |
| Alteradora | babab | Aceptada | babab#babab | 114 |
| Alteradora | bbaab | Aceptada | bbaab#bbaab | 114 |

Las dos entradas rechazadas pertenecen al alfabeto; todas las demostraciones cumplen la longitud mínima. Las cuatro transformaciones terminan sin A/B residuales. Las trazas reales completas están en [reconocedora.txt](evidencias/reconocedora.txt) y [alteradora.txt](evidencias/alteradora.txt). [limite.txt](evidencias/limite.txt) muestra abc inconclusa tras una transición.

## Verificación reproducible

```text
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --locked
cargo build --release --locked
```

Todos pasaron en Linux. Suite: una prueba unitaria de cinta y diez pruebas de integración con oráculos exhaustivos (9.841 entradas reconocedoras hasta longitud 8; 255 alteradoras hasta longitud 7, incluida vacía), ocho demostraciones, memoria, null, movimientos, Unicode, errores, límite, independencia y exportación. Los conteos actualizados de pruebas constan en [pruebas.txt](evidencias/pruebas.txt); no se cuentan los ejecutables sin pruebas ni los doc-tests vacíos como validación.

Para repetir y regenerar las evidencias:

```bash
bash scripts/verificar.sh
```

```powershell
.\scripts\verificar.ps1
```

Los scripts paran ante fallos y guardan las salidas de los cuatro comandos; el de PowerShell no se ejecutó en esta sesión. Guardar una copia de `evidencias/` antes de regenerarla si se quiere conservar los registros previos. La conservación completa de instantáneas consume memoria proporcional a pasos por celdas ocupadas; ajustar el límite para entradas largas.

## Entregables y revisión

- [Arquitectura, notación y costes](docs/arquitectura.md).
- [Descripción formal, invariantes, corrección y terminación de las máquinas](docs/maquinas.md).
- Tablas completas: [reconocedora](docs/tabla_reconocedora.md) y [alteradora](docs/tabla_alteradora.md).
- [Auditoría de requisitos y rúbrica](docs/cumplimiento.md).
- [Guion para Brian y Emilio](docs/guion_video.md).
- [Diez commits autorizados y autoría](docs/commits.md).
- [Lista final administrativa y técnica](docs/entrega.md).

El destino local es el repositorio abierto `EmilioChenf/Proyecto-3-Teoria-de-la-computacion`. No se utilizó un repositorio de Proyecto 2. La privacidad remota no está verificada: debe confirmarse antes de cualquier publicación. El push de los diez commits a main está autorizado y se verifica por separado; no se crearon invitaciones. Los integrantes deben grabar y subir el video a YouTube como **no listado** y entregar su enlace; el PDF no fija duración máxima.
