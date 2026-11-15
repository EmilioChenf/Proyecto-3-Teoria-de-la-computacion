# Auditoría de requisitos y rúbrica

Fuente: PDF «Proyecto No 3.pdf», cinco páginas, leído mediante extracción y revisión visual completa (incluidas las imágenes de páginas 2 y 3). SHA-256 del adjunto: `c2383abc2e23c2c5a002dc04f28e328841c7dad37a9c718e1bb1876ad2ed88e7`. El adjunto original permanece fuera del checkout; no se inventó un enlace público. Se distinguen requisitos de la guía y ampliaciones solicitadas por el usuario.

## Guía

| Fuente | Requisito | Archivo / evidencia concreta | Estado |
|---|---|---|---|
| p.1 | Pareja inscrita en grupo de Canvas | docs/entrega.md | Pendiente externo |
| p.1 | Repositorio privado y colaboradores docentes | Origin identifica Proyecto 3; docs/entrega.md | Privacidad sin verificar; usuarios de Canvas pendientes |
| p.1 | Instrucciones en README raíz | ../README.md | Preparadas |
| p.1–2 | Modelo configurable, parsing de configuración y entradas | ../src/configuracion.rs, ../src/main.rs | Probado |
| p.2 imagen | Claves YAML, memoria auxiliar, blanco vacío, L/R/S | ../maquinas/*.yaml; pruebas campos_blancos_vacios_son_obligatorios_y_no_comodines y memoria_diferencia_transiciones_y_null_borra_registro | Probado |
| p.3 imagen | Ejemplo incompleto | Inconsistencias enumeradas en maquinas.md | Documentado, no usado como máquina válida |
| p.4 | Una cinta, Σ⊆Γ y reglas configuradas | ../src/cinta.rs, ../src/motor.rs, validación y oráculos | Probado |
| p.4 | Todas las cadenas configuradas y listado íntegro de IDs | ../src/main.rs, ../src/presentacion.rs, trazas y prueba de exportación | Probado |
| p.4 | Aceptar en final, rechazar sin regla | ../src/motor.rs; pruebas de parada y ocho demostraciones | Probado |
| p.4 | Dos reconocidas y dos rechazadas, longitud ≥5 | ../evidencias/reconocedora.txt | Ejecuciones verificadas; video pendiente |
| p.4 | Cuatro alteraciones, longitud ≥5; no borrar todo | ../evidencias/alteradora.txt | Ejecuciones verificadas; video pendiente |
| p.4–5 | Explicar arquitectura en video | arquitectura.md, guion_video.md | Preparado; grabación pendiente |
| p.4 | Video YouTube no listado y enlace de entrega | guion_video.md, entrega.md | Pendiente externo |

## Ampliaciones solicitadas

| Requisito del usuario | Implementación / verificación | Estado |
|---|---|---|
| Rust estable, Cargo, nombres propios en español y separación de responsabilidades | Cargo.toml, rust-toolchain.toml, src/*.rs | Implementado |
| YAML mantenido, dependencias fijadas, no parser improvisado | serde-saphyr 2.1.1 descargado y compilado, Cargo.lock | Verificado |
| null distinto de "null" y #; símbolos Unicode de longitud 1 | Simbolo; pruebas blanco_vacio_yaml_y_unicode, campos_blancos_vacios_son_obligatorios_y_no_comodines y ejemplos_adicionales_reconocedores_y_separador_no_es_blanco | Verificado |
| Memoria inicial blanca, selección y actualización real, null borra | Parametros, Salida y motor; prueba específica de memoria y alteradora | Verificado |
| Cinta extensible bidireccional, negativos, blancos internos/exteriores | Cinta, prueba unitaria y prueba de memoria | Verificado |
| Independencia entre entradas, aceptación inicial y rechazo sin regla | ocho_demostraciones_e_independencia y aceptacion_inicial_rechazo_limite_y_entrada_invalida | Verificado |
| Límite configurable inconcluso, nunca rechazo automático | motor, --limite, ../evidencias/limite.txt y prueba en frontera exacta | Verificado |
| Salida completa, formal, estado/memoria/cabezal/transición/cinta final/pasos | presentacion.rs, arquitectura.md, pruebas de trazas y CLI | Verificado |
| YAML mal formado, campos ausentes, estados, símbolos, alfabetos, blanco, movimientos y duplicados | errores_de_configuracion y campos_blancos_vacios_son_obligatorios_y_no_comodines | Verificado |
| Entrada inválida distinta de rechazo válido | validar símbolos antes de simular, CLI código 1 y prueba de CLI | Verificado |
| Reconocedora aⁿbⁿcⁿ exclusivamente con reglas | reconocedora.yaml; 9.841 entradas hasta longitud 8 contra oráculo independiente | Verificado |
| Casos adicionales: vacía, abc, orden incorrecto y desigualdad | prueba explícita de ejemplos adicionales y oráculo exhaustivo | Verificado |
| Alteradora w#w exclusivamente con reglas, #∉Σ, sin marcadores finales | alteradora.yaml; 255 entradas hasta longitud 7, 254 no vacías; comprobación de salida exacta | Verificado |
| Definiciones, tablas completas, invariantes, corrección, terminación y costes | maquinas.md, tabla_reconocedora.md, tabla_alteradora.md, arquitectura.md | Preparado |
| Compilar ambos YAML y ejecutar ocho casos con binario optimizado | scripts/verificar.sh y ../evidencias/*.txt | Verificado |
| fmt, Clippy estricto, test --locked, release --locked | registros respectivos; 1 prueba unitaria + 10 de integración aprobadas | Aprobado en Linux |
| Comandos Linux y Windows PowerShell | README y scripts/verificar.* | Linux verificado; Windows no disponible, sin ejecución real |
| Diez commits con patrón y autoría autorizada | commits.md, diez incrementos con autor explícito y committer conservado | Autorizados y registrados; revisión humana independiente |
| Guion para dos integrantes y auditoría final | guion_video.md, este archivo, entrega.md | Preparado |
| Publicación explícitamente autorizada en la continuación | Diez commits en main; push normal condicionado a privacidad; no invitaciones | Consultar resultado remoto en informe final |

## Rúbrica de 15 puntos

No se otorga una nota ni se afirma cumplimiento del video. La dificultad y la calidad de la exposición son valoradas por el evaluador.

| Criterio | Puntos posibles | Evidencia preparada | Pendiente para entrega |
|---|---:|---|---|
| Dos cadenas aceptadas por reconocedora | 2 | aabbcc y aaabbbccc, IDs completas en reconocedora.txt | Mostrar en video |
| Dos cadenas rechazadas por reconocedora | 2 | aabbbcc y aaabbccc, entradas válidas, reconocedora.txt | Mostrar en video |
| Dificultad reconocedora | 3 | Tres bloques correlacionados, marcas X/Y/Z, pruebas exhaustivas y argumento formal | Exponer y evaluación docente |
| Cuatro cadenas alteradas | 4 | Cuatro w#w exactas, alteradora.txt, prueba sin marcadores | Mostrar en video |
| Dificultad alteradora | 3 | Marcado, transporte con memoria, recorridos y restauración; demostración de corrección | Exponer y evaluación docente |
| Arquitectura | 1 | Cinco módulos, modelo y guion; pruebas de memoria y cinta | Explicar en video |
| Total posible | 15 | Material local preparado | No es puntuación obtenida |

## Evidencias y límites de lo observado

`evidencias/formato.txt` vacío significa que `cargo fmt --check` terminó sin diferencias; el script detiene su ejecución si falla. Los registros clippy.txt, pruebas.txt y compilacion.txt pertenecen a la ejecución final de scripts/verificar.sh. Los módulos main y doc-tests no contienen pruebas y no se suman al total. Las trazas corresponden al binario release; cada entrada contiene exactamente pasos+1 IDs. El límite adicional produce dos IDs para un paso.

Se probó también el script de instalación guardado (instalación repetible, fetch --locked, build y test). Se guardaron `install_script` y `start_skill` en el borrador del entorno con confirmación de persistencia. Eso no publica el entorno ni demuestra restauración en una tarea nueva. La revisión y publicación del entorno corresponde al usuario en los ajustes del entorno.

No se verificó Windows, no se verificó privacidad remota, se autorizó la creación de diez commits y su publicación remota, cuyo resultado se verifica por separado; no se grabó video ni se accedió a Canvas. No se promete una nota. La conservación íntegra de instantáneas usa memoria O(pasos·celdas); el espacio de índices es i64 y la máquina física impone límites prácticos.
