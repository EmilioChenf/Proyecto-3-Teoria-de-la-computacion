# Diez commits autorizados del proyecto

Brian y Emilio autorizaron explícitamente en esta sesión usar sus identidades como autores, crear los diez commits e integrarlos en main y publicarlos en el remoto del Proyecto 3. No se modificó la configuración global de Git. Se preservó el commit inicial `e02c592` y no se reescribió historia publicada.

Se usó `git commit --author` por bloque. La identidad de committer conservada es la que ya estaba configurada: Josué Emilio Chen Borrayo <emiliojosue321@gmail.com>. Las fechas son las reales del entorno; no se añadieron firmas ni afirmaciones de revisión humana. Los cambios fueron preparados con asistencia automatizada: el autor Git no demuestra por sí solo quién implementó o revisó una sección.

| N.º | Autor | Mensaje | Contenido real |
|---|---|---|---|
| 1 | ChenBrian833 <5.68bol@gmail.com> | Crear la estructura del proyecto y los modelos de configuración | Cargo, lockfile, toolchain, modelo de símbolos, estados, parámetros y salidas |
| 2 | ChenBrian833 <5.68bol@gmail.com> | Implementar la lectura y validación de archivos YAML | Maquina, carga, validación e inicio de pruebas de configuración |
| 3 | ChenBrian833 <5.68bol@gmail.com> | Implementar la cinta extensible y las operaciones del cabezal | Cinta dispersa, blancos, cabezal y prueba unitaria bidireccional |
| 4 | EmilioChenf <che24841@uvg.edu.gt> | Implementar el motor de simulación y la memoria auxiliar | Motor, resultados e instantáneas; prueba de selección y actualización de memoria |
| 5 | EmilioChenf <che24841@uvg.edu.gt> | Mostrar las descripciones instantáneas y los resultados | Presentación, CLI, exportación y comprobaciones de formato de trazas/Unicode |
| 6 | ChenBrian833 <5.68bol@gmail.com> | Definir y verificar la máquina reconocedora | YAML, tabla completa y oráculo exhaustivo hasta longitud ocho |
| 7 | ChenBrian833 <5.68bol@gmail.com> | Definir y verificar la máquina alteradora | YAML, tabla completa y prueba exhaustiva de copia hasta longitud siete |
| 8 | EmilioChenf <che24841@uvg.edu.gt> | Completar las pruebas de integración y los casos límite | Ocho demostraciones, independencia, CLI, límite y blancos obligatorios |
| 9 | EmilioChenf <che24841@uvg.edu.gt> | Documentar la arquitectura y el funcionamiento de las máquinas | Modelo formal, estados, invariantes, corrección, terminación y costes |
| 10 | EmilioChenf <che24841@uvg.edu.gt> | Preparar las demostraciones y auditar los entregables | README, registros reales, scripts, guion, matriz, lista administrativa y auditoría del historial; corrección y prueba de espacios finales en las trazas |

El patrón es Brian ×3, Emilio ×2, Brian ×2 y Emilio ×3: cinco commits por autor. Cada uno contiene cambios reales, sin commits vacíos ni duplicaciones para inflar la actividad. Los incrementos 1–8 se probaron a partir del contenido exacto del índice en directorios temporales fuera del checkout, sin crear worktrees ni alterar los archivos de trabajo. Las pruebas se incorporaron a medida que aparecieron las capacidades correspondientes.

## Consulta reproducible del historial

Desde la raíz del proyecto:

```sh
git log --reverse --format='%h | %an <%ae> | %cn <%ce> | %s' e02c592..main
git rev-list --count e02c592..main
git status --short
git rev-parse main origin/main
git ls-remote origin refs/heads/main
```

El hash de este último bloque se consulta con Git después de crearlo: incluir su propio hash dentro del mismo commit sería circular. El informe de publicación en la conversación proporciona los hashes observados y el estado remoto. La creación local, la privacidad y el push son verificaciones distintas; un historial local no prueba publicación.

El destino es exclusivamente `https://github.com/EmilioChenf/Proyecto-3-Teoria-de-la-computacion`, rama main. La publicación debe realizarse con push normal y únicamente tras verificar privacidad. Si aparece historia remota nueva, se conserva mediante integración sin squash ni force push. No se autorizaron invitaciones en este paso.
