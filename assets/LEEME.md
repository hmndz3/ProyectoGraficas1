# Música personalizada

Coloca aquí tus canciones con estos nombres exactos:

| Archivo | Nivel |
|---|---|
| `music1.ogg` (o `music1.wav`) | I — The Fool |
| `music2.ogg` (o `music2.wav`) | II — The Hanged Man |
| `music3.ogg` (o `music3.wav`) | III — The Hermit |

- Formatos soportados: **OGG** y **WAV** (MP3 no; conviértelo con Audacity
  o cualquier convertidor a OGG).
- Si un archivo no existe o no se puede leer, el juego usa su música
  sintetizada de respaldo — nunca se rompe.
- El juego debe ejecutarse desde la raíz del proyecto (`cargo run --release`)
  para que encuentre esta carpeta.
- Los archivos de audio de esta carpeta están en `.gitignore`: si usas
  canciones comerciales, quedan solo en tu máquina y no se suben al repo.
  Si usas música libre (royalty-free) y quieres incluirla, quita esas
  líneas del `.gitignore`.
