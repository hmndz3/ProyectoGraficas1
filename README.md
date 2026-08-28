# ARCANA — un raycaster de tarot

> **Video de demostración:** _(agregar link aquí)_

Proyecto 3 de Gráficas por Computadora: un **ray caster** escrito en **Rust**
(motor propio, sin engine) con tres niveles jugables inspirados en tres
arcanos mayores del tarot:

| Arcano | Nivel | Ambiente |
|---|---|---|
| **0 — The Fool** | Acantilado luminoso | Cielo abierto, estandartes dorados |
| **XII — The Hanged Man** | Anillos suspendidos | Mundo invertido: techo abismal, piso pálido |
| **IX — The Hermit** | Criptas en penumbra | Niebla corta: solo ves lo que alumbra tu farol |

## Objetivo del juego

En cada nivel debes **recoger los 3 sellos** (cartas de tarot flotantes).
Al reunirlos, el **portal se enciende**: entra en él para superar el arcano.
Los **espíritus te persiguen y su toque drena tu vida** — dispáralos con tu
báculo para disiparlos. Si tu luz se agota, el arcano te reclama (pantalla
de derrota con reintento). Al superar los tres arcanos se muestra la
pantalla final.

**La maldición del Colgado:** en el nivel XII todos los controles están
invertidos — movimiento, giro de teclado **y mouse** — porque el mundo se ve
desde la horca, de cabeza.

## Cómo correr

Requiere [Rust](https://rustup.rs/) (estable). Luego:

```
cargo run --release
```

No hay assets externos: todas las texturas, sprites, efectos de sonido y la
música se **generan proceduralmente** al iniciar.

## Controles

| Acción | Entrada |
|---|---|
| Moverse | `W A S D` (o flechas arriba/abajo) |
| Girar la cámara | **Mouse** (horizontal) o flechas izq/der |
| Correr | `Shift` izquierdo |
| Disparar | Clic izquierdo |
| Sensibilidad del mouse | `[` bajar / `]` subir |
| Pantalla completa | `F11` |
| Volver al menú | `Esc` |
| Seleccionar nivel (menú) | Flechas / mouse + `Enter` o clic, o `1` `2` `3` |

## Características

- Motor de **raycasting DDA** propio con paredes texturizadas (5 texturas
  distintas por nivel: piedra, ladrillo, estandarte, tablones y runas).
- **Colisión deslizante**: no se atraviesan paredes y no crashea.
- **Rotación horizontal con mouse** (captura de cursor durante el juego).
- **Disparo** hitscan con destello, retroceso visual y línea de visión real
  (no puedes disparar a través de paredes).
- **Minimapa** en la esquina superior derecha con jugador, dirección de
  vista, sellos restantes, espíritus y portal.
- **Música de fondo original** por nivel, sintetizada en tiempo de carga
  (pads generativos — 0% Taylor Swift).
- **Efectos de sonido** sintetizados: disparo, recoger sello, disipar
  espíritu, apertura de portal y fanfarria de victoria.
- **Sprites animados** por cuadros: cartas flotantes con halo pulsante,
  espíritus ondulantes y portal de orbes giratorios.
- **Pantalla de bienvenida** con selección entre los tres niveles.
- **Pantalla de éxito** al superar cada arcano (tiempo y espíritus
  disipados) y pantalla de victoria final.
- Niebla por distancia, gradientes de cielo/piso, viñeta ambiental y
  paleta propia por arcano.
- Test automático (`cargo test`) que verifica por BFS que cada sello,
  espíritu y portal es alcanzable desde el spawn — niveles siempre
  completables.

## Estructura

```
src/
  main.rs      máquina de estados y loop del juego
  raycast.rs   DDA, columnas texturizadas, niebla, z-buffer
  level.rs     los tres arcanos: mapas, paletas y validación BFS
  player.rs    movimiento, rotación y colisión deslizante
  sprites.rs   billboards animados: cartas, espíritus, portal
  textures.rs  texturas procedurales 64x64
  audio.rs     sintetizador WAV: SFX y música por nivel
  minimap.rs   minimapa de esquina
  ui.rs        menú, HUD, éxito y victoria
```
