# Proyecto 3 — Simulador de máquinas de Turing

Simulador determinista de **una cinta en Rust**, configurable mediante YAML. Incluye cinta extensible en ambas direcciones, un registro finito de memoria auxiliar y descripciones instantáneas completas. El motor ejecuta exclusivamente las transiciones declaradas; no contiene algoritmos específicos para reconocer o copiar cadenas.

**Integrantes:** Brian ([ChenBrian833](https://github.com/ChenBrian833)) y Emilio ([EmilioChenf](https://github.com/EmilioChenf)). El repositorio del proyecto es privado.

## Requisitos e instalación

Se necesita Rust estable con Cargo, rustfmt, Clippy y un enlazador del sistema. `rust-toolchain.toml` selecciona el canal estable y `Cargo.lock` fija las dependencias, incluida `serde-saphyr` para deserializar YAML. No se requieren servicios, credenciales ni Python para ejecutar el programa.

### Linux

Instalar Rust desde <https://rustup.rs/>:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/instalar-rust.sh
sh /tmp/instalar-rust.sh -y --profile minimal --default-toolchain stable
. "$HOME/.cargo/env"
```

Si falta el enlazador `cc`, instalar las herramientas de compilación de la distribución. Desde la carpeta que contiene el repositorio:

```bash
cd Proyecto-3-Teoria-de-la-computacion
rustup component add rustfmt clippy
cargo build --release --locked
```

### Windows PowerShell

Instalar Rust desde <https://rustup.rs/> o mediante winget:

```powershell
winget install --exact --id Rustlang.Rustup
```

Completar las indicaciones del instalador para instalar las herramientas C++ de Visual Studio: carga **Desarrollo para el escritorio con C++**, enlazador MSVC y Windows SDK. Abrir una nueva PowerShell y ejecutar desde la carpeta que contiene el repositorio:

```powershell
cd Proyecto-3-Teoria-de-la-computacion
rustup default stable
rustup component add rustfmt clippy
cargo build --release --locked
```

## Compilación y pruebas

Desde la raíz del proyecto, en Linux o PowerShell:

```text
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --locked
cargo build --release --locked
```

La suite contiene una prueba unitaria y diez de integración. Incluye blancos YAML, movimientos L/R/S, posiciones negativas, memoria auxiliar, validación, aceptación inicial, rechazo sin transición, límite de pasos, independencia entre entradas y exportación íntegra de trazas. Compara exhaustivamente 9.841 cadenas reconocedoras hasta longitud 8 y 255 cadenas alteradoras hasta longitud 7, incluida la vacía, con oráculos independientes del motor.

Los scripts ejecutan las verificaciones anteriores y ambas máquinas, y guardan registros y trazas regenerables en `target/verificacion/`, fuera del seguimiento de Git:

```bash
# Linux
bash scripts/verificar.sh
```

```powershell
# Windows PowerShell
.\scripts\verificar.ps1
```

Las verificaciones se ejecutaron en Linux. Los comandos y el script de PowerShell están preparados, pero no se probaron en Windows.

## Ejecución

Estos comandos Cargo funcionan desde la raíz del proyecto en ambos sistemas:

```text
cargo run --release --locked -- maquinas/reconocedora.yaml
cargo run --release --locked -- maquinas/alteradora.yaml
cargo run --release --locked -- maquinas/reconocedora.yaml --entrada aabbcc
cargo run --release --locked -- maquinas/reconocedora.yaml --entrada aabbbcc
cargo run --release --locked -- maquinas/reconocedora.yaml --entrada abc --limite 1
```

Sin `--entrada` se procesan todas las `simulation_strings` del YAML. Los tres últimos comandos muestran, respectivamente, aceptación, rechazo y resultado inconcluso. El límite predeterminado es de 100.000 transiciones por entrada. Alcanzar el límite nunca implica rechazo automático.

Para ejecutar el binario optimizado y exportar todas las descripciones instantáneas, crear primero el directorio de salida:

```bash
# Linux
mkdir -p target/trazas
./target/release/simulador-mt maquinas/reconocedora.yaml --traza target/trazas/reconocedora.txt
./target/release/simulador-mt maquinas/alteradora.yaml --traza target/trazas/alteradora.txt
```

```powershell
# Windows PowerShell
New-Item -ItemType Directory -Force target/trazas | Out-Null
.\target\release\simulador-mt.exe maquinas/reconocedora.yaml --traza target/trazas/reconocedora.txt
.\target\release\simulador-mt.exe maquinas/alteradora.yaml --traza target/trazas/alteradora.txt
```

`--traza ARCHIVO` escribe el mismo texto completo que aparece en consola; el directorio destino debe existir. `--ayuda` muestra la sintaxis. Errores de configuración, argumentos, archivos o símbolos de entrada producen código de salida 1. Aceptación, rechazo e inconclusión son resultados de simulación y producen código 0; se distinguen en el texto.

## Configuración YAML

Las definiciones completas y sus tablas de transición están en [reconocedora.yaml](maquinas/reconocedora.yaml) y [alteradora.yaml](maquinas/alteradora.yaml). Para ejecutar una máquina nueva basta crear otro YAML; no se modifica Rust.

Este ejemplo mínimo acepta la entrada `a` y la sustituye por blanco en un paso:

```yaml
q_states:
  q_list: ['inicio', 'aceptar']
  initial: 'inicio'
  final: 'aceptar'
alphabet: ['a']
tape_alphabet: ['a', null]
delta:
  - params:
      initial_state: 'inicio'
      mem_cache_value: null
      tape_input: 'a'
    output:
      final_state: 'aceptar'
      mem_cache_value: null
      tape_output: null
      tape_displacement: S
simulation_strings:
  - 'a'
```

- `q_states` declara estados, estado inicial y estado final; `alphabet` declara el alfabeto de entrada y `tape_alphabet` el de cinta.
- Cada regla de `delta` selecciona por **estado, memoria y lectura** y determina el nuevo estado, memoria, escritura y movimiento.
- `L`, `R` y `S` significan izquierda, derecha y sin movimiento.
- El blanco se representa con YAML `null` o un valor vacío. Es distinto de `'#'` y de la cadena literal `"null"`, que no se admite como símbolo porque contiene cuatro caracteres.
- Cada símbolo no blanco es un único carácter Unicode de Rust (`char`). Las longitudes cuentan caracteres, no bytes ni grafemas; no hay normalización Unicode.
- El alfabeto de cinta debe contener el de entrada y el blanco. El alfabeto de entrada no admite blanco.

La memoria inicial es blanca y cada transición la actualiza: `null` significa escribir blanco en el registro, no comodín ni conservar el valor anterior. Sus valores forman un conjunto finito derivado de las reglas. El par `(estado, memoria)` puede incorporarse al control finito, de modo que el registro no constituye otra cinta.

Se rechazan YAML mal formado, campos obligatorios ausentes o desconocidos, estados no declarados, símbolos de cinta desconocidos, alfabetos incompatibles, movimientos inválidos y transiciones duplicadas para la misma clave. Una cadena fuera del alfabeto se informa como **entrada inválida**, diferenciada de una entrada válida que la máquina rechaza.

Los ejemplos de la guía son incompletos: la imagen de ejemplo omite `a` y `b` del alfabeto de cinta e incluye entradas con `#` fuera del alfabeto de entrada. Su segunda actualización de memoria tampoco coincide con la fórmula. Se conserva el esquema exigido, pero las dos máquinas incluidas tienen definiciones independientes y válidas.

## Semántica y visualización

Cada entrada comienza con estado inicial, memoria blanca, contador cero y cinta independiente. La entrada ocupa posiciones desde 0 y el cabezal comienza en 0. La cinta admite posiciones negativas y celdas blancas en ambos extremos.

Se acepta al alcanzar el estado final, incluso antes de ejecutar una transición. Sin regla aplicable fuera del estado final se rechaza. Si queda una regla pendiente y se agotó el límite, el resultado es inconcluso.

La descripción instantánea formal es `u⟨estado,memoria⟩v`: el cabezal lee la primera celda de `v`. Por ejemplo:

```text
Paso 0 | estado=inicio | memoria=□ | cabezal=0
ID: -1:□ ⟨inicio,□⟩ 0:'a' 1:□
```

Cada celda lleva su coordenada; `□` sin comillas representa blanco y los caracteres no blancos aparecen entre comillas. La salida muestra el intervalo visitado completo, más un blanco exterior por lado. La transición rotulada en el paso k es la que produjo esa configuración desde el paso k−1.

Se conserva la configuración inicial y una por transición: una ejecución de t pasos contiene t+1 IDs. Al finalizar se informa resultado, motivo, cinta final y cantidad de transiciones. El resumen útil recorta exclusivamente blancos exteriores; mantiene blancos internos y todos los demás símbolos, incluidos marcadores en ejecuciones rechazadas. El carácter literal `□` se distingue del blanco en las IDs por sus comillas.

Conservar todas las instantáneas consume memoria proporcional a pasos por celdas ocupadas. Para entradas grandes conviene ajustar el límite y exportar la traza; exportarla no elimina el coste de conservarla en memoria. Las posiciones usan índices `i64`, sujetos a los límites prácticos de la máquina física.

## Máquinas incluidas y resultados

### Reconocedora de {aⁿbⁿcⁿ | n ≥ 1}

Los estados `inicio` y `elegir` seleccionan una `a` y la marcan como `X`; `buscar_b` marca una `b` como `Y`; `buscar_c` marca una `c` como `Z`; `volver` regresa al extremo izquierdo. Cuando no quedan `a`, `verificar` comprueba que solo queden marcadores antes de llegar a `aceptar`. La memoria permanece blanca.

Cada ronda completa consume una letra de cada bloque. Para una entrada ordenada, tras i rondas la cinta tiene la forma `Xⁱaⁿ⁻ⁱYⁱbᵐ⁻ⁱZⁱcᵏ⁻ⁱ`. Solo se acepta cuando se agotaron los tres bloques por igual y hubo al menos una ronda. Las búsquedas bloquean símbolos en orden incorrecto; cada ronda reduce las `a` pendientes y los recorridos son finitos. Su coste es O(N²) transiciones y O(N) celdas.

### Alteradora w → w#w para w ∈ {a,b}⁺

`inicio` exige una entrada no vacía y `separar` añade `#`. `elegir` marca el siguiente original con `A` o `B` y guarda su valor en memoria. `llevar` lo transporta al blanco derecho para anexarlo; `regresar` vuelve al extremo izquierdo. El separador impide seleccionar la copia como original. Al terminar, `restaurar` convierte `A/B` nuevamente en `a/b` y se alcanza `aceptar`.

Tras i rondas hay exactamente i símbolos originales marcados y una copia de ese prefijo a la derecha de `#`. Cada ronda reduce los originales pendientes; al restaurar, la cinta es exactamente `w#w`, sin marcadores residuales. Su coste es O(n²) transiciones y O(n) celdas. `#` pertenece al alfabeto de cinta, pero no al de entrada.

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

Las dos entradas rechazadas son válidas para su alfabeto. Las cuatro transformaciones aceptan y terminan sin `A/B`. Las ocho entradas tienen longitud mínima cinco; los comandos y scripts anteriores permiten reproducir sus trazas completas.

## Organización del código

| Archivo | Responsabilidad |
|---|---|
| `src/configuracion.rs` | Deserialización, modelo formal y validación |
| `src/cinta.rs` | Cinta dispersa, lectura, escritura y cabezal |
| `src/motor.rs` | Transiciones, memoria, condiciones de parada e instantáneas |
| `src/presentacion.rs` | Representación de IDs y resultados |
| `src/main.rs` | CLI, archivos, exportación y mensajes |
| `tests/integracion.rs` | Oráculos independientes, casos límite y CLI |

La cinta usa un `BTreeMap` que guarda solo celdas no blancas; borrar una celda elimina su entrada. Una tabla `HashMap` indexa las transiciones por estado, memoria y lectura. El motor y la presentación se mantienen separados.
