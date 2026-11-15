# Arquitectura y semántica

El motor es Rust estable. No hay lógica de reconocimiento ni de copia en Rust: `maquinas/*.yaml` proporciona todas las reglas. Serde y serde-saphyr 2.1.1 deserializan YAML; Cargo.lock fija las versiones y sumas de comprobación. Se verificó su disponibilidad descargando desde crates.io y compilando con Rust 1.100.0. No se usa el descontinuado serde_yaml ni un parser improvisado.

| Archivo | Responsabilidad |
|---|---|
| src/configuracion.rs | Deserialización estricta, modelo formal, validación e índice de transiciones |
| src/cinta.rs | Mapa disperso de posiciones i64 a caracteres, lectura/escritura/movimiento |
| src/motor.rs | Bucle determinista, registro auxiliar, parada y traza completa |
| src/presentacion.rs | Descripciones instantáneas y representación textual |
| src/main.rs | Argumentos, archivos, salida y errores |
| tests/integracion.rs | Oráculos independientes, validación y CLI |

## Modelo

M = (Q, Σ, Γ, C, δ, q₀, qf, □). δ: Q × C × Γ ⇀ Q × C × Γ × {L,R,S}. C es el conjunto finito de valores de memoria mencionados en las reglas, unido a {□}. No se exige que C esté contenido en Γ: es un registro auxiliar independiente. El par (q,m) puede incorporarse al control finito Q×C, por lo que no constituye otra cinta. Las demostraciones usan C={□} y C={□,a,b}, respectivamente.

Cada símbolo es exactamente un valor escalar Unicode de Rust (`char`), no un byte ni un grafema. Por ejemplo ñ tiene longitud 1; una vocal y un acento combinante cuentan como 2. No hay normalización Unicode. El blanco es `Simbolo(None)`: YAML `null`, `~` o una entrada vacía. `"null"` tiene cuatro caracteres y es inválido como símbolo; `'#'` es un carácter ordinario, distinto del blanco. El espacio entre comillas también es un carácter ordinario. Los nombres de estado son cadenas. Memoria y símbolos de transición son campos obligatorios, aunque admiten blanco explícito. Se rechazan campos desconocidos.

La cinta comienza con la entrada en posiciones 0..longitud-1 y blanco en el resto, con cabezal 0. Un BTreeMap guarda solo celdas no blancas. Borrar elimina una celda; las posiciones negativas son válidas. Los límites visitados conservan la extensión histórica, incluso después de borrar. La representación muestra todo ese intervalo y un blanco adicional a cada extremo. Las coordenadas i64 y los recursos físicos son finitos; no se promete simular literalmente una cinta infinita ni ejecuciones arbitrariamente grandes.

Cada simulación crea cinta, estado inicial, memoria blanca y contador 0 nuevos. Se comprueba aceptación antes de buscar regla y antes del límite. Sin regla fuera del final se rechaza, aun con límite 0; si hay regla pendiente y se han ejecutado N transiciones se informa inconclusa. Una transición cambia simultáneamente el estado lógico, la memoria y el símbolo, y mueve el cabezal. `null` en la memoria de salida borra el registro: no es comodín ni conservación.

## Descripciones instantáneas

La notación formal es u⟨q,m⟩v: la primera celda de v está bajo el cabezal. En la salida se anotan además las coordenadas de cada celda. `□` representa blanco, y los caracteres no blancos aparecen entre comillas Rust para distinguir espacios, controles y delimitadores. Por ejemplo `-1:□ ⟨inicio,□⟩ 0:'a' 1:□` sitúa el cabezal en 0. Se muestran paso, estado, memoria y posición explícitos. La transición rotulada en el paso k>0 es la que lleva de la ID k−1 a la ID k.

Se conserva la ID inicial y exactamente una por transición: longitud de traza = pasos + 1. `--traza` guarda el mismo texto completo que stdout; no hay truncamiento ni modo silencioso. El resumen útil recorta únicamente blancos exteriores, mantiene los blancos internos como □ y conserva todos los demás símbolos, incluidos marcadores en rechazos. La ID final muestra también blancos y posiciones. Para distinguir un carácter literal □ de blanco, consulte las IDs, donde el carácter aparece entre comillas.

## Validación y costes

Se verifican declaraciones de estados, unicidad de estados y alfabetos, Σ⊆Γ, □∈Γ y □∉Σ, símbolos de lectura/escritura, movimientos, claves duplicadas (estado,memoria,lectura), estructura y tipos. La entrada se valida antes de crear la simulación; una entrada inválida no se etiqueta como rechazada. Las reglas pueden ser parciales; eso modela rechazo. Una transición saliente de qf nunca se ejecuta.

La tabla HashMap da búsqueda esperada O(1); la cinta tiene operaciones O(log s), para s celdas ocupadas. Guardar cada instantánea clona la cinta: para t pasos, tiempo y memoria de conservación O(t·s), además del formato O(t·v), siendo v el intervalo visitado. El límite configurable protege de máquinas que no terminan; exportar a archivo permite revisar trazas largas, pero no elimina el coste de conservarlas. No se requieren servicios, puertos, claves ni credenciales para ejecutar o probar el proyecto.
