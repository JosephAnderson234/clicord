# Seguridad

## Estado de soporte

No hay versiones estables ni despliegues recomendados. La serie `0.1.x` es experimental y no ofrece confidencialidad frente al servidor, autenticación de identidad ni comunicación remota segura. **No uses clicord para información sensible.**

## Reportar una vulnerabilidad

Si el repositorio está alojado en GitHub y tiene habilitado **Private vulnerability reporting**, usa esa función en la pestaña Security. Si no está habilitada, contacta a los mantenedores mediante un canal privado que publiquen antes de compartir detalles. No abras un issue público con instrucciones de explotación, claves o datos personales.

Incluye la versión o commit, plataforma, impacto, pasos para reproducir con datos sintéticos y una forma segura de contactarte. Los mantenedores deben confirmar la recepción, coordinar una corrección y publicar un aviso cuando sea apropiado. Todavía no hay un SLA de respuesta; se definirá antes de una versión estable.

## Alcance actual

El servidor recibe texto legible y cualquier cliente local puede elegir cualquier nombre. Un compromiso del servidor o del dispositivo puede exponer conversaciones. Consulta [docs/threat-model.md](docs/threat-model.md).
