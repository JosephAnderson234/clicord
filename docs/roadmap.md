# Hoja de ruta

Los hitos se completan por criterios verificables, no por fechas prometidas.

## 0. Base de proyecto abierto

- Licencia, guía de instalación, contribución, reporte privado de seguridad y modelo de amenazas.
- Formato, tests y Clippy en CI.
- Prueba manual de la TUI en dos terminales y documentación de sus resultados.

## 1. Prototipo local robusto

- Límites de nombre y mensaje, timeout de saludo y manejo explícito de clientes lentos.
- Estado visible de desconexión y pruebas de protocolo y servidor.
- Contrato de protocolo versionado antes de cambiar mensajes incompatiblemente.

## 2. Conversaciones privadas de dos personas

Diseño y criterios de aceptación: [phase-2-private-messaging.md](phase-2-private-messaging.md). Aún no implementado.

- Identidad criptográfica verificable y cifrado de extremo a extremo usando una implementación revisada.
- Claves protegidas en el dispositivo, detección de cambios de identidad y cifrado del transporte remoto.
- Servidor sin acceso al contenido, política explícita de metadatos y revisión externa de seguridad.

## 3. Funciones posteriores

- Grupos con evaluación de MLS, múltiples dispositivos, entrega sin conexión y persistencia cifrada.
- Auditoría de privacidad y pruebas de compatibilidad entre versiones.

Hasta completar el hito 2, toda documentación y versión debe señalar que clicord no es apto para conversaciones sensibles.
