# Fase 2: conversaciones privadas de dos personas

**Estado: diseño para revisión. La aplicación actual no implementa este documento y no es apta para mensajes sensibles.**

## Alcance y criterio de publicación

La primera versión privada admite una conversación entre dos dispositivos, uno por persona, con entrega mientras ambos están conectados. El servidor actúa como directorio de claves públicas y transportador de mensajes cifrados. Historial remoto, grupos, múltiples dispositivos, recuperación de cuenta y sincronización sin conexión quedan fuera de esta fase. No se anunciará como versión privada hasta superar las pruebas y la revisión independiente indicadas abajo.

## Elección de protocolo

El candidato para la implementación de dos personas es [vodozemac](https://github.com/matrix-org/vodozemac), usando Olm (intercambio inicial y Double Ratchet), sin implementar primitivas criptográficas propias. Es una biblioteca Rust usada por Matrix, con licencia Apache-2.0 y una auditoría publicada. Antes de añadirla al código se fijará una versión exacta, se revisarán sus avisos de seguridad y se compilará en las plataformas soportadas. La API de [Olm](https://docs.rs/vodozemac/latest/vodozemac/olm/) proporciona cuentas, claves de un solo uso y sesiones para mensajes cifrados.

[libsignal](https://github.com/signalapp/libsignal) no es la primera opción aquí: su proyecto declara que el uso fuera de Signal no tiene soporte y usa AGPL-3.0, lo que requeriría revisar la distribución y el mantenimiento del proyecto. [OpenMLS](https://github.com/openmls/openmls) se evaluará para grupos en una fase posterior.

## Identidad y confianza

1. El dispositivo genera su identidad criptográfica localmente. El servidor nunca recibe claves privadas.
2. Antes del primer mensaje sensible, el usuario debe comparar la huella de la clave de identidad del contacto por un canal independiente y confirmarla en el cliente. Un nombre visible no prueba identidad; confiar automáticamente en la primera clave permitiría una sustitución por el servidor.
3. El cliente recuerda la clave aceptada. Ante un cambio de clave, bloquea el envío y muestra una advertencia hasta una nueva verificación explícita. No acepta silenciosamente otra clave.
4. Las claves de un solo uso deben estar vinculadas criptográficamente a la identidad anunciada; se rechazan bundles inválidos o consumidos. La revisión de la API concreta y sus firmas es una condición previa a implementar este flujo.
5. La conexión al servidor debe demostrar posesión de la clave del dispositivo mediante un desafío fresco y una firma verificada. El servidor autoriza el acceso a la conversación usando esa identidad, nunca el nombre visible enviado por el cliente.
6. La pérdida de la clave local implica pérdida de acceso a la sesión. La recuperación y los dispositivos múltiples requieren otro diseño y no se simulan con una nueva identidad silenciosa.

## Datos y transporte

- El protocolo de red se versionará. El modo privado no aceptará `ChatMessageIn`/`ChatMessageOut` en texto legible ni ofrecerá una conversión automática a texto claro si falla el cifrado.
- Cada mensaje privado llevará identificadores opacos de conversación y destinatario, tipo de mensaje Olm y ciphertext. El servidor debe rechazar un destinatario distinto al de la conversación autorizada y nunca interpretar el contenido cifrado.
- El servidor verá IP, hora aproximada, participantes, frecuencia y tamaño de los mensajes mientras enruta. Estos metadatos son una limitación real; no se prometerá anonimato.
- Las conexiones remotas usarán `wss://` con validación de certificados. `ws://` queda restringido a pruebas locales en loopback. El cifrado de transporte complementa el cifrado de extremo a extremo porque reduce la exposición de metadatos en tránsito.
- No se guardarán mensajes en el servidor durante esta fase. Los logs no incluirán contenido, claves, ciphertext completo ni identificadores persistentes innecesarios. Se documentará cualquier retención operacional antes del despliegue.
- Las cuentas y sesiones locales se guardarán solo cifradas, con una clave protegida por el sistema operativo o por una derivación de contraseña revisada. Si no puede protegerse el estado, el cliente fallará de forma cerrada. No habrá un archivo de claves sin cifrar como alternativa silenciosa.

## Secuencia de implementación

1. **Contrato v2:** esquemas versionados, límites, errores explícitos y pruebas de compatibilidad. El protocolo v1 queda marcado como demostración local.
2. **Identidad local:** generación, almacenamiento cifrado, huella visible y detección de cambio de clave. Pruebas de reinicio y pérdida de clave.
3. **Entrega cifrada:** distribución y validación de bundles, sesiones Olm, enrutamiento a un único destinatario y rechazo de todo mensaje privado en texto claro. Pruebas entre dos clientes y de tercero no autorizado.
4. **Transporte y operación:** `wss://` remoto, configuración de certificados, límites de recursos, reconexión y política de metadatos.
5. **Revisión:** auditoría externa del diseño y la integración, corrección de hallazgos y publicación de los límites restantes.

## Pruebas de aceptación

- Capturar el tráfico y el estado del servidor no revela el texto de una conversación privada.
- Un tercero conectado no recibe mensajes de la conversación ni puede descifrarlos si obtiene una copia del ciphertext.
- Alterar, repetir, reordenar o truncar mensajes produce errores seguros; no se muestran como mensajes auténticos.
- Una clave de identidad sustituida por el servidor detiene el envío hasta que el usuario la verifique por otro canal.
- Tras reiniciar ambos clientes, la identidad y las sesiones se restauran sin guardar secretos en claro. Un fallo de almacenamiento no degrada a texto claro.
- Los tests, Clippy y el formato pasan en CI, y dos terminales reales intercambian mensajes cifrados de punta a punta.
- Una revisión de seguridad independiente evalúa la integración completa, no solo la biblioteca criptográfica.

## Dependencias de ejecución

Este entorno necesita descargar dependencias de Cargo y acceder a GitHub para compilar, probar y publicar el hito. Mientras esos accesos falten, no se incorporará una integración criptográfica que no pueda verificarse.
