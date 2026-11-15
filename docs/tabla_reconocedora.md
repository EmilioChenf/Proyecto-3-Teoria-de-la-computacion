# Tabla completa: reconocedora

Las combinaciones omitidas rechazan. El estado aceptar detiene inmediatamente.

| Estado | Memoria | Lee | Nuevo estado | Nueva memoria | Escribe | Movimiento |
|---|---|---|---|---|---|---|
| inicio | □ | a | buscar_b | □ | X | R |
| elegir | □ | X | elegir | □ | X | R |
| elegir | □ | a | buscar_b | □ | X | R |
| elegir | □ | Y | verificar | □ | Y | R |
| buscar_b | □ | a | buscar_b | □ | a | R |
| buscar_b | □ | Y | buscar_b | □ | Y | R |
| buscar_b | □ | b | buscar_c | □ | Y | R |
| buscar_c | □ | b | buscar_c | □ | b | R |
| buscar_c | □ | Z | buscar_c | □ | Z | R |
| buscar_c | □ | c | volver | □ | Z | L |
| volver | □ | a | volver | □ | a | L |
| volver | □ | b | volver | □ | b | L |
| volver | □ | c | volver | □ | c | L |
| volver | □ | X | volver | □ | X | L |
| volver | □ | Y | volver | □ | Y | L |
| volver | □ | Z | volver | □ | Z | L |
| volver | □ | □ | elegir | □ | □ | R |
| verificar | □ | Y | verificar | □ | Y | R |
| verificar | □ | Z | verificar | □ | Z | R |
| verificar | □ | □ | aceptar | □ | □ | S |
