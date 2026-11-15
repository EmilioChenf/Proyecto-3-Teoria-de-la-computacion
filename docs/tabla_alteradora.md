# Tabla completa: alteradora

Las combinaciones omitidas rechazan. El estado aceptar detiene inmediatamente.

| Estado | Memoria | Lee | Nuevo estado | Nueva memoria | Escribe | Movimiento |
|---|---|---|---|---|---|---|
| inicio | □ | a | separar | □ | a | R |
| inicio | □ | b | separar | □ | b | R |
| separar | □ | a | separar | □ | a | R |
| separar | □ | b | separar | □ | b | R |
| separar | □ | □ | volver | □ | # | L |
| volver | □ | a | volver | □ | a | L |
| volver | □ | b | volver | □ | b | L |
| volver | □ | A | volver | □ | A | L |
| volver | □ | B | volver | □ | B | L |
| volver | □ | □ | elegir | □ | □ | R |
| elegir | □ | A | elegir | □ | A | R |
| elegir | □ | B | elegir | □ | B | R |
| elegir | □ | a | llevar | a | A | R |
| elegir | □ | b | llevar | b | B | R |
| llevar | a | a | llevar | a | a | R |
| llevar | a | b | llevar | a | b | R |
| llevar | a | A | llevar | a | A | R |
| llevar | a | B | llevar | a | B | R |
| llevar | a | # | llevar | a | # | R |
| llevar | a | □ | regresar | □ | a | L |
| llevar | b | a | llevar | b | a | R |
| llevar | b | b | llevar | b | b | R |
| llevar | b | A | llevar | b | A | R |
| llevar | b | B | llevar | b | B | R |
| llevar | b | # | llevar | b | # | R |
| llevar | b | □ | regresar | □ | b | L |
| regresar | □ | a | regresar | □ | a | L |
| regresar | □ | b | regresar | □ | b | L |
| regresar | □ | A | regresar | □ | A | L |
| regresar | □ | B | regresar | □ | B | L |
| regresar | □ | # | regresar | □ | # | L |
| regresar | □ | □ | elegir | □ | □ | R |
| elegir | □ | # | restaurar | □ | # | L |
| restaurar | □ | A | restaurar | □ | a | L |
| restaurar | □ | B | restaurar | □ | b | L |
| restaurar | □ | □ | aceptar | □ | □ | S |
