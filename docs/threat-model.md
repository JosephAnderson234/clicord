# Modelo de amenazas — versión inicial

Este documento describe el código actual. Se actualiza antes de añadir autenticación, persistencia o acceso remoto.

## Objetivo

Permitir conversaciones donde solo los participantes autorizados puedan leer el contenido y verificar con quién hablan. La promesa futura debe cubrir confidencialidad, integridad, autenticación, disponibilidad razonable y retención mínima de metadatos. Ningún sistema puede garantizar privacidad absoluta ante dispositivos comprometidos o participantes que divulgan el contenido.

## Activos y adversarios

Activos: texto, claves futuras, identidad de los participantes, pertenencia a conversaciones y metadatos de conexión. Adversarios considerados: cliente no autorizado, participante malicioso, operador o intruso del servidor y observador de red. Un dispositivo comprometido y un participante que copia mensajes quedan fuera de la protección criptográfica del sistema, pero deben figurar en la documentación pública.

## Lo que protege hoy

El servidor escucha solo en loopback. Los campos `from` y `ts` de un mensaje saliente los escribe el servidor, así que un cliente no puede elegirlos en cada mensaje. Esto **no autentica** el nombre de usuario: se elige libremente al conectar.

## Lo que no protege hoy

- El servidor lee cada mensaje en texto legible y lo envía a todas las conexiones de una sala global.
- `ws://` no cifra el transporte. El cliente solo conecta a loopback; no existe un modo remoto seguro.
- No hay autenticación, verificación de identidad, claves ni cifrado de extremo a extremo.
- No hay aislamiento entre conversaciones, control completo de abuso, historial ni garantía de entrega.
- Hay límites básicos de longitud y tiempo de saludo, pero no cuotas por usuario ni protección completa contra denegación de servicio.
- La UI conserva mensajes en memoria durante la sesión. El proceso no implementa borrado seguro de memoria.

No se debe publicar una afirmación de «mensajería privada» basada en este estado.

## Requisitos antes de una versión privada

1. Elegir y revisar una implementación mantenida de un protocolo de cifrado de extremo a extremo adecuado al alcance inicial de dos personas; el servidor solo debe transportar ciphertext.
2. Vincular identidades a claves verificables por los usuarios y detectar cambios de clave.
3. Proteger claves locales y establecer reglas de recuperación, revocación y dispositivos múltiples.
4. Exigir transporte cifrado para conexiones remotas, controlar límites y documentar los metadatos visibles para el servidor.
5. Probar casos de suplantación, repetición, pérdida, reconexión, cliente lento y cambios de versión; obtener revisión de seguridad independiente antes de declarar una versión estable.

Para grupos, evaluar MLS por separado. Añadir persistencia solo después de decidir qué datos cifrados y metadatos son indispensables y por cuánto tiempo se conservan.
