# clicord

Chat en terminal escrito en Rust. **Estado: prototipo experimental. No lo uses para conversaciones sensibles.**

Hoy hay una sola sala en memoria. El servidor recibe los mensajes en texto legible, los reenvía a todas las conexiones y confía en el nombre elegido por cada cliente. Solo escucha en `127.0.0.1:3000`; no hay cuentas, cifrado de extremo a extremo, historial ni garantía de entrega. La [política de seguridad](SECURITY.md) y el [modelo de amenazas](docs/threat-model.md) describen los límites con detalle.

## Probar localmente

Necesitas Rust estable y Cargo. En Windows, instala una toolchain con un linker disponible (MSVC con Visual Studio Build Tools o GNU con MinGW). Desde la raíz:

```sh
cargo run -p server
```

En otras dos terminales:

```sh
cargo run -p client
```

El cliente solicita un nombre y se conecta al servidor local. Pulsa Enter para enviar y Esc o Ctrl+C para salir. El cliente TUI todavía requiere una verificación manual en dos terminales reales.
El nombre admite hasta 64 bytes y cada mensaje hasta 4096 bytes; no se aceptan caracteres de control.

## Desarrollo

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

La primera ejecución necesita descargar dependencias. Revisa [CONTRIBUTING.md](CONTRIBUTING.md) antes de enviar cambios y [docs/roadmap.md](docs/roadmap.md) para el orden de trabajo.

## Estructura

- `protocol`: tipos JSON compartidos.
- `server`: servidor WebSocket y sala en memoria.
- `client`: interfaz de terminal.

`docker-compose.yml` es un ejemplo de desarrollo para una futura base de datos; el programa no usa Postgres actualmente.

## Licencia

MIT. Consulta [LICENSE](LICENSE).
