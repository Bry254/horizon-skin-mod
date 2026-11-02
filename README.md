# Custom Skin — mod nativo para Horizon

## ADVERTENCIA: USO DE IA

Este proyecto fue desarrollado utilizando herramientas
de inteligencia artificial generativa.

**Desarrollo asistido por:**
- OpenCode(DeepSeek V4.1 Flash Free): generación de código, arquitectura, resolución de problemas y documentación.
La intervención de la IA se declara por transparencia.

Ponte tu propia skin de minecraft en Horizon, sin depender de la cuenta: funciona con nombres offline, en un jugador, en LAN y en servidores.
Es un mod **de cliente**: solo tú ves el cambio en tu personaje.

- Comando: `/skin <archivo.png>` (o `/hskin`), `/skin on|off|slim|classic|reload`.
- Ajustes: **Options → Mods → Custom Skin**, con interruptor, brazos y recarga.
- El PNG se copia a `horizon-mods/.horizon-mod-data/horizon_skin/skin.png`,
  así que sobrevive a que muevas o borres el original.
- También puedes dejar un `skin.png` en esa carpeta y no usar el comando.

## Límites

- Cambia el modelo del jugador local (cuerpo en tercera persona, brazo en
  primera persona, la vista previa del inventario normal y la de la pestaña
  **Inventario** del creativo).
- La cabeza que sale en la lista de jugadores (Tab) sigue usando la skin que
  conoce el servidor o la cuenta. Lo mismo para lo que ven los demás.
- No sube nada a Mojang ni cambia tu skin en línea.

## Instalar

### Con el launcher (recomendado)

1. Actualiza Horizon a un Nightly que tenga mods nativos y los hooks que
   este mod usa (API ≥ 35; el Nightly 2026-10-31, `d91cab6`, tiene API 42).
   La v0.2.97 todavía no: su binario no lleva `native_hooks.rs`.
2. Actualiza el launcher a 0.2.31 o posterior (el 0.2.19 no conoce
   `native-build`). Los mods nativos ya no tienen interruptor: se instalan
   desde **Mods** o se compilan con `native-build`.
3. Construye el mod desde esta carpeta:

   ```
   horizon-launcher native-build carpeta/a/horizon_skin --trust-native-code
   ```

   La primera compilación descarga el código de Horizon y tarda 20–30
   minutos; las siguientes, unos minutos.
4. El launcher cambia la instancia a la versión nativa; juégala.
5. Dentro del juego: `/skin /ruta/a/tu_skin.png`.

### Como `.hnmod`

El paquete ya está hecho: `horizon_skin-1.0.0.hnmod`.
Se instala desde la página **Mods** del launcher, que lo guarda en
`horizon-native-mods/` y compila una versión nativa del juego. **No** va en
`horizon-mods/`: esa carpeta es para los `.hmod` antiguos.
