# Descripción formal de las máquinas

Convenciones del modelo, del blanco y de δ en [arquitectura](arquitectura.md). Las tablas enumeran todas las reglas; las combinaciones ausentes rechazan. Los YAML son las definiciones ejecutables, y las tablas se corresponden fila por fila con ellas.

## Reconocedora de {aⁿbⁿcⁿ | n≥1}

Q={inicio,elegir,buscar_b,buscar_c,volver,verificar,aceptar}; q₀=inicio; qf=aceptar; Σ={a,b,c}; Γ={a,b,c,X,Y,Z,□}; C={□}.

[Tabla completa](tabla_reconocedora.md) y [configuración](../maquinas/reconocedora.yaml).

| Estado | Significado |
|---|---|
| inicio | Exigir una primera a: excluye n=0 |
| elegir | Saltar X y marcar la siguiente a como X; si se llega a Y, verificar el sufijo |
| buscar_b | Saltar a y Y, marcar la primera b como Y |
| buscar_c | Saltar b y Z, marcar la primera c como Z |
| volver | Regresar hasta el blanco izquierdo y reanudar en elegir |
| verificar | Exigir solo Y o Z hasta blanco |
| aceptar | Detener con aceptación |

Cada ronda marca una a, una b y una c y regresa al inicio. En entradas de la forma aⁿbᵐcᵏ, después de i rondas completas la cinta es Xⁱaⁿ⁻ⁱYⁱbᵐ⁻ⁱZⁱcᵏ⁻ⁱ. La memoria permanece blanca. Al agotarse las a, solo se acepta si ya no quedan b ni c sin marcar.

**Corrección.** Para aⁿbⁿcⁿ, cada búsqueda encuentra el símbolo correspondiente, las n rondas terminan y verificar alcanza el blanco. En sentido inverso, cada a procesada exige una b y una c, elegidas en ese orden. No se puede saltar una c en buscar_b ni una a en buscar_c. La vuelta no altera la cinta. Un símbolo fuera del orden esperado bloquea una búsqueda o queda sin marcar y bloquea verificar. El barrido final solo permite marcadores; por tanto una aceptación implica bloques ordenados con igual cantidad positiva. Las marcas nunca se convierten en símbolos de entrada ni se reutilizan como una nueva b/c.

**Terminación.** En una ronda cada búsqueda avanza monótonamente hasta una coincidencia o el primer símbolo no permitido; la vuelta avanza a la izquierda hasta blanco. Toda ronda exitosa disminuye en uno el número de a sin marcar. El barrido final avanza a la derecha. No hay ciclos infinitos en entradas finitas. Para longitud N hay O(N) rondas de O(N) movimientos: O(N²) transiciones y O(N) celdas. La cinta final aceptada retiene XⁿYⁿZⁿ: esta máquina reconoce, no restaura.

Se prueban exhaustivamente todas las 9.841 cadenas hasta longitud 8, incluyendo vacía, abc, órdenes incorrectos y cantidades desiguales, contra un oráculo que divide en tres bloques iguales. También se prueba expresamente aaabbbccc (longitud 9) y las cuatro demostraciones.

## Alteradora w → w#w, w∈{a,b}⁺

Q={inicio,separar,volver,elegir,llevar,regresar,restaurar,aceptar}; q₀=inicio; qf=aceptar; Σ={a,b}; Γ={a,b,A,B,#,□}; C={□,a,b}.

[Tabla completa](tabla_alteradora.md) y [configuración](../maquinas/alteradora.yaml).

| Estado | Significado |
|---|---|
| inicio | Exigir entrada no vacía |
| separar | Buscar el final y escribir # |
| volver | Retornar al blanco izquierdo tras colocar separador |
| elegir | Saltar A/B; seleccionar siguiente símbolo original; al llegar a # iniciar restauración |
| llevar | Recorrer hasta el blanco derecho con a/b guardado en memoria y anexarlo |
| regresar | Volver al blanco izquierdo con memoria blanca |
| restaurar | Cambiar A→a y B→b hacia la izquierda |
| aceptar | Detener; no quedan marcadores |

Primero se añade #. Después de i copias, la cinta tiene la forma marcar(prefijoᵢ(w)) · sufijoᵢ(w) · # · prefijoᵢ(w). A/B denotan originales ya copiados; la parte copiada contiene exclusivamente a/b. Elegir nunca cruza # al seleccionar un símbolo: así no copia la copia. Llevar usa el valor de memoria seleccionado para escribir en el extremo derecho y lo restablece a blanco. Tras n rondas, restaurar recupera el original.

**Corrección.** El invariante vale inicialmente para i=0. Una ronda marca exactamente el siguiente símbolo original, guarda su valor en el registro, cruza # y la copia existente y escribe ese mismo valor al final; establece el invariante para i+1 manteniendo el orden. Cuando elegir ve #, todos los n originales están marcados y la copia es w. Restaurar cambia solo A/B a a/b, preserva # y la copia. La aceptación deja exactamente w#w y ninguna marca residual. La cadena vacía se rechaza en inicio; # no es entrada válida.

**Terminación y complejidad.** Cada ronda reduce en uno los originales no marcados; los barridos son monótonos entre extremos finitos. Tras n rondas la restauración finita termina. Tiempo O(n²) en transiciones y espacio 2n+1 celdas no blancas, O(n). Ambas máquinas requieren varios recorridos y control de fases; estas propiedades sustentan la discusión de dificultad, cuya puntuación corresponde al evaluador.

Se comparan exhaustivamente las 254 cadenas no vacías hasta longitud 7 con el oráculo independiente `format!("{cadena}#{cadena}")`, comprobando aceptación y ausencia de A/B. La cadena vacía también se comprueba y rechaza.

## Inconsistencias de los ejemplos del PDF

Se leyó texto e imágenes de las cinco páginas. En la página 2 la imagen define las claves respetadas aquí y admite L/R/S, así como blanco representado por una entrada vacía. La plantilla contiene descripciones y puntos suspensivos: no es una máquina ejecutable.

La imagen de la página 3 enumera Γ={#,X,□}, pero Σ={a,b}; por tanto incumple Σ⊆Γ y usa a/b en lecturas fuera de Γ. Las cadenas incluyen #, fuera de Σ. Solo hay dos transiciones desde 0, insuficientes para alcanzar el final 5. En la segunda regla YAML se escribe memoria a para lectura b, mientras que la fórmula de la página 2 indica memoria b. No se reproduce como máquina válida ni se corrige silenciosamente: se conservó el esquema y se diseñaron máquinas completas independientes.

El PDF recomienda Python, sin exigirlo: esta entrega usa Rust por instrucción del usuario. La rúbrica suma 15 puntos y no fija duración máxima de video.
