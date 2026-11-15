# Guion de grabación para dos integrantes

Preparación: abrir una terminal en la raíz del proyecto y el editor; ampliar fuente. Ejecutar las cuatro verificaciones del README antes de grabar. El video debe estar en YouTube como no listado y su enlace debe entregarse en Canvas. No se ha grabado ni subido. El PDF no fija tiempo máximo: ajustar la duración a una explicación comprensible de la ejecución completa, sin inventar un límite.

## 1. Brian: problema, YAML y modelo

**Decir:** «Implementamos un simulador determinista de una cinta en Rust, parametrizado por YAML. El motor no conoce el lenguaje reconocido ni la transformación. Cada transición selecciona un estado, memoria y símbolo leído, y determina el estado, memoria, símbolo escrito y movimiento».

**Mostrar:** README, Cargo.toml, `src/configuracion.rs` y encabezado de `maquinas/reconocedora.yaml`. Explicar Σ⊆Γ, blanco null, L/R/S y el rechazo de duplicados. Señalar que las imágenes del ejemplo de la guía son incompletas y no constituyen una máquina válida; mostrar la sección de inconsistencias en docs/maquinas.md.

**Comprobar:** `cargo test --locked` pasa; explicar pruebas exhaustivas y diferencia entre entrada inválida y rechazo válido.

## 2. Brian: reconocedora y dificultad (3 puntos sujetos a evaluación)

**Decir:** «Reconocemos aⁿbⁿcⁿ con n positivo. Marcamos una a, buscamos una b, luego una c, y regresamos. X/Y/Z registran las rondas en la propia cinta. El barrido final exige que no quede nada sin marcar». Mostrar la tabla completa en `docs/tabla_reconocedora.md` y estados de `docs/maquinas.md`.

**Explicar:** invariante de las rondas, por qué cantidades distintas u orden incorrecto bloquean una transición, y terminación por reducción de a sin marcar. Tres bloques correlacionados y recorridos repetidos dan una dificultad mayor que un barrido regular simple; no se promete puntuación. O(N²) transiciones y O(N) cinta.

**Ejecutar**, en Linux o PowerShell con Cargo:

```text
cargo run --release --locked -- maquinas/reconocedora.yaml --traza evidencias/reconocedora.txt
```

**Comprobar en pantalla:** las cuatro entradas, sus IDs inicial y final y resultados. Aceptadas: aabbcc (29 pasos), aaabbbccc (58). Rechazadas: aabbbcc (30), aaabbccc (31). Ambas rechazadas son entradas válidas de longitud ≥5, sin transición aplicable antes del final. Este bloque cubre las demostraciones de 2+2 puntos, sujetas a revisión del video.

## 3. Emilio: IDs, motor, memoria y arquitectura (1 punto sujeto a evaluación)

**Mostrar:** `src/cinta.rs`, `src/motor.rs`, `src/presentacion.rs`, `src/main.rs` y tabla de arquitectura. Explicar por qué BTreeMap permite posiciones negativas y escritura de blanco sin otra cinta. Mostrar las fases selección, escritura, movimiento, actualización del estado/memoria y conservación de la ID.

**Decir:** «En u⟨q,m⟩v el cabezal lee la primera celda de v. Cada posición de la salida está numerada. □ es blanco; las comillas distinguen caracteres ordinarios. La transición mostrada produjo la configuración actual. La memoria es un registro finito y puede incorporarse al estado; null la borra».

**Mostrar:** una ID inicial, una intermedia y la final en `evidencias/reconocedora.txt`. Indicar el cabezal con coordenada y el cambio de estado. La traza exportada contiene todos los pasos, no solo los seleccionados para explicación.

**Ejecutar:**

```text
cargo run --release --locked -- maquinas/reconocedora.yaml --entrada abc --limite 1
```

**Comprobar:** inconclusa tras un paso, no rechazada. Explicar aceptación inicial y rechazo por tabla parcial mediante pruebas en tests/integracion.rs.

## 4. Emilio: alteradora, cuatro casos y dificultad (4+3 puntos sujetos a evaluación)

**Mostrar:** `maquinas/alteradora.yaml`, `docs/tabla_alteradora.md` y explicación formal. Seleccionar una regla de elegir que guarda a en memoria, una de llevar que escribe a y pone memoria null y otra equivalente para b.

**Decir:** «Añadimos #, marcamos originales con A/B, llevamos el símbolo en memoria al extremo derecho y repetimos. El separador impide seleccionar la copia como original. Al terminar restauramos A/B. Tras i rondas hay exactamente i caracteres copiados, en orden. Cada ronda reduce los originales no marcados y cada recorrido es finito: tiempo cuadrático y cinta lineal».

**Ejecutar:**

```text
cargo run --release --locked -- maquinas/alteradora.yaml --traza evidencias/alteradora.txt
```

**Comprobar los cuatro resúmenes:**

| Entrada | Cinta final | Pasos |
|---|---|---:|
| aabab | aabab#aabab | 114 |
| abbaa | abbaa#abbaa | 114 |
| babab | babab#babab | 114 |
| bbaab | bbaab#bbaab | 114 |

Todos aceptan, tienen longitud cinco, conservan original y copia y terminan sin A/B. Mostrar una ID con memoria a/b y otra con memoria blanca. No se borra toda la cinta ni se calcula la copia en Rust. La selección, transporte, múltiples recorridos y restauración sustentan la dificultad.

## 5. Ambos: evidencia y cierre

Brian muestra pruebas exhaustivas reconocedoras y Emilio las alteradoras, aclarando que los oráculos solo existen en pruebas. Mostrar logs de fmt, Clippy, pruebas y compilación, matriz de requisitos y lista administrativa. Explicar las limitaciones de recursos y el límite de pasos. No afirmar que el nombre del autor de un commit demuestra contribución humana. Antes de subir, revisar que el video muestra las ocho ejecuciones, arquitectura y argumentos de dificultad. Subir como no listado, verificar acceso al enlace y adjuntarlo a la entrega.
