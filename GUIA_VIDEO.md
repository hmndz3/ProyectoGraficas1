# Guía para grabar el video de entrega

Guion sugerido (2–3 minutos) que muestra **todos** los puntos de la rúbrica.

## Preparación

1. `cargo run --release`
2. `F11` para pantalla completa (opcional pero se ve mejor).
3. Graba con OBS / Game Bar (`Win + G`) **con audio del sistema activado**
   para que se escuche la música y los efectos.

## Guion

1. **Pantalla de bienvenida** (5 y +10 pts): deja ver el título y mueve la
   selección entre las 3 cartas con flechas y con el mouse. Entra a
   **The Fool** con `Enter`.
2. **Música de fondo** (5 pts): ya suena desde que entra el nivel — no la
   cortes al editar.
3. **Rotación con mouse** (20 pts): gira la cámara claramente con el mouse
   un par de segundos.
4. **Colisión** (requisito): camina de frente contra una pared y muestra
   que te deslizas y no la atraviesas, sin crash.
5. **Texturas distintas** (requisito): pasea mostrando piedra, ladrillo,
   estandartes con estrella, tablones y runas brillantes.
6. **Minimapa** (10 pts): señala la esquina superior derecha mientras te
   mueves — se ve tu posición y dirección.
7. **Animación de sprites** (20 pts): acércate a una carta flotante (halo
   pulsa y flota) y a un espíritu (ondula). Quédate 2 segundos.
8. **Disparo + efectos de sonido** (10 + 10 pts): dispara a un espíritu y
   disípalo — se oye el disparo y el "poof".
9. Recoge los **3 sellos** (suena el arpegio) y muestra que el portal se
   enciende (aviso en HUD + sonido).
10. **Pantalla de éxito** (10 pts): entra al portal y deja ver la pantalla
    de "ARCANO SUPERADO" con las estadísticas.
11. `ESPACIO` para pasar a **The Hanged Man**: enseña el cambio total de
    paleta (mundo invertido) unos segundos.
12. `Esc` al menú y entra directo a **The Hermit** con `3` para demostrar
    la **selección de niveles** — muestra la oscuridad con niebla corta.
13. (Si hay tiempo) completa The Hermit para cerrar con la pantalla de
    victoria de los tres arcanos.

## Checklist de rúbrica

- [x] Nivel completo y jugable, sin atravesar paredes ni crashes
- [x] Textura/color diferente por tipo de pared (5 por nivel)
- [x] Rotación horizontal con mouse (+20)
- [x] Disparar (+10)
- [x] Minimapa en esquina (+10)
- [x] Música de fondo original (+5, no es de Taylor Swift)
- [x] Efectos de sonido (+10)
- [x] Sprites con animación (+20)
- [x] Pantalla de bienvenida (+5) con selección de nivel (+10)
- [x] Pantalla de éxito por condición del nivel (+10)
- [x] Estética: 3 arcanos con paleta, niebla, música y sprites propios (0–30)
