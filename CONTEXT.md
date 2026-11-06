# clicord — contexto de diseño

"Discord de terminal": dos usuarios se conectan desde terminales distintas y
chatean en tiempo real. Este documento existe para que una futura sesión
(humana o de IA) pueda retomar el proyecto sin releer todo el historial de
decisiones.

## Estado actual

Scaffold inicial. `cargo build` se verificó anteriormente y compiló limpio (workspace
completo: protocol + server + client) con Rust 1.97.1, toolchain
`stable-x86_64-pc-windows-gnu`.

Implementa la Meta 1: un room global, sin canales, sin auth, sin
persistencia.

Revisión de noviembre de 2026: se añadieron documentación open source,
límites de longitud, timeout del saludo y desconexión explícita de clientes
rezagados. `cargo fmt --all -- --check` pasa. No se pudo repetir la
compilación ni los tests en este entorno: faltan crates en la caché y el
acceso a `static.crates.io` no funciona. El servidor sigue viendo texto
legible; lee `docs/threat-model.md` antes de tratarlo como mensajero privado.

**Servidor y protocolo verificados en runtime**: se levantó `server` y se
simularon dos clientes WebSocket (script Node, no el cliente TUI real) —
`hello` + broadcast bidireccional confirmado, con `from`/`ts` puestos por el
servidor y no por el cliente, como estaba diseñado.

**Cliente TUI NO probado interactivamente todavía** — Ratatui necesita una
terminal real (modo raw + alternate screen), que no estaba disponible en el
entorno donde se hizo esta verificación. Pendiente: abrir dos terminales de
verdad, correr `cargo run -p server` en una y `cargo run -p client` en cada
una de las otras dos, y confirmar que el chat se ve y funciona de punta a
punta con la UI real.

### Nota de toolchain (Windows)

Esta máquina no tenía `link.exe` (MSVC), así que se optó por el toolchain
GNU en vez de instalar Visual Studio Build Tools:

- Rust instalado vía `winget install Rustlang.Rustup`.
- Linker vía `winget install MSYS2.MSYS2` + `pacman -S mingw-w64-x86_64-gcc`.
- `rustup default stable-x86_64-pc-windows-gnu`.
- `C:\msys64\mingw64\bin` agregado al PATH de usuario (variable persistente,
  no solo de la sesión).

Si se clona este repo en otra máquina Windows sin este setup, `cargo build`
fallará con `error: linker \`link.exe\` not found` a menos que se repita este
mismo setup, o se instale Visual Studio Build Tools y se use el toolchain
MSVC por defecto en su lugar.

## Decisiones ya tomadas (no re-discutir sin razón nueva)

- **Serialización**: JSON, no binario. Prioriza debuggability sobre
  eficiencia en esta etapa.
- **Forma del mensaje**: envelope con tag interno
  `{ "type": "...", "payload": {...} }` vía `serde(tag = "type", content =
  "payload")`. Ver `protocol/src/lib.rs`, enum `Envelope`.
- **Transporte**: WebSocket. Servidor: Axum. Cliente: tokio-tungstenite.
- **Identidad**: `Hello { username }` enviado al conectar, sin validar, sin
  unicidad. Es un placeholder consciente — se reemplaza por auth real
  cuando llegue, sin tocar el resto del protocolo (el server simplemente
  empezará a resolver `username` desde una sesión autenticada en vez de
  confiar en el primer frame del cliente).
- **Fan-out en el servidor**: `tokio::sync::broadcast::channel`, no un
  `HashMap<ConnId, Sender>` manual. Un solo `AppState.tx` para el room
  global. Cuando se agreguen canales/rooms, la migración es
  `AppState.tx: HashMap<ChannelId, broadcast::Sender<ChatMessageOut>>` detrás
  de un `RwLock` — el patrón subscribe/send no cambia.
- **Estructura**: Cargo workspace, 3 members (`protocol`, `server`,
  `client`), no un solo crate con `src/bin/*`. Razón: cliente y servidor
  corren en procesos/máquinas distintas, y `protocol` compartido evita que
  el JSON se desincronice entre los dos lados.
- **Cliente — loop de eventos**: un solo `tokio::select!` unificando tres
  fuentes: teclado (leído en un hilo de OS aparte porque
  `crossterm::event::read()` es bloqueante), red (mpsc alimentado por
  `net::connect`), y un tick fijo de 100ms
  (`MissedTickBehavior::Skip` para no acumular ticks atrasados). Se optó
  por tick fijo en vez de redibujar solo on-event, específicamente para
  dejar espacio a animaciones futuras (spinners, cursor blink) sin tener
  que rediseñar el loop.
- **Docker/Postgres**: presente como placeholder (`docker-compose.yml`) pero
  intencionalmente desconectado de todo. No conectar hasta que la Meta 1
  funcione de punta a punta.

## Pendiente / próximos pasos naturales

En este orden, cada uno construye sobre el anterior:

1. Probar la Meta 1 real: `cargo run -p server`, luego dos instancias de
   `cargo run -p client` en terminales distintas, confirmar que un mensaje
   escrito en una aparece en la otra.
2. Repetir `cargo test --workspace --locked` y Clippy con acceso a dependencias.
3. Definir identidad verificable y cifrado de extremo a extremo para dos personas.
4. Canales/rooms — migrar `AppState.tx` según lo descrito arriba, después de
   resolver la seguridad de conversaciones de dos personas.
5. Persistencia cifrada y retención mínima, solo tras definir el modelo de
   metadatos; `docker-compose.yml` sigue siendo un placeholder.

## Cosas NO resueltas todavía (decidir cuando toque)

- Forma de `app.rs` del cliente más allá de lo mínimo (buffer de input,
  lista de mensajes, tick_count) — falta estado de conexión
  (conectando/conectado/caído) y cómo se refleja en la UI.
- Si el redibujo debe seguir siendo "cada tick sin condición" o pasar a un
  flag `dirty` cuando el historial de mensajes crezca y el render se vuelva
  costoso.
