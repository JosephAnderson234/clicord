# Contribuir a clicord

Gracias por mejorar el proyecto. El idioma principal de la documentación es español; los nombres de código y las descripciones de PR pueden estar en español o inglés.

## Antes de empezar

Lee [README.md](README.md), [docs/threat-model.md](docs/threat-model.md) y [docs/roadmap.md](docs/roadmap.md). Para cambios de arquitectura, protocolo o comportamiento de seguridad, abre primero un issue que explique el problema, las alternativas y el efecto sobre la privacidad. Las correcciones pequeñas pueden ir directamente en un PR.

## Flujo de trabajo

1. Crea una rama para un cambio acotado.
2. Añade pruebas que comprueben el comportamiento cuando sea pertinente; documenta las pruebas manuales de la TUI.
3. Ejecuta los controles locales:

   ```sh
   cargo fmt --all -- --check
   cargo test --workspace --locked
   cargo clippy --workspace --all-targets --locked -- -D warnings
   ```

4. Describe en el PR qué cambia, por qué, cómo lo probaste y qué riesgos quedan.

No agregues datos reales de conversaciones, tokens, claves privadas, contraseñas ni datos personales a commits, fixtures, logs o capturas. Usa datos sintéticos.

## Regla de privacidad

La privacidad es un requisito de revisión, no una etiqueta de marketing. Un PR que afecte transporte, identidad, claves, protocolo, almacenamiento, registros o métricas debe:

- actualizar el modelo de amenazas y declarar qué información ve el servidor;
- justificar el tratamiento y la retención de metadatos;
- incluir pruebas para errores, límites y rutas de rechazo;
- evitar diseñar criptografía nueva o guardar secretos sin una revisión específica;
- mantener la advertencia de prototipo hasta que la propiedad de seguridad esté implementada y revisada.

La revisión de seguridad es obligatoria antes de fusionar cambios criptográficos. Una prueba automática por sí sola no equivale a una auditoría.

## Compatibilidad

Documenta cualquier cambio en los mensajes JSON y cómo actuarán clientes y servidores de versiones distintas. Cambios incompatibles requieren una versión de protocolo y un plan de migración.

## Vulnerabilidades

Sigue [SECURITY.md](SECURITY.md). No publiques detalles explotables en un issue público antes de coordinar su corrección.

## Conducta

Sigue [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
