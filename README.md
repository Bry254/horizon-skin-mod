# Custom Skin — Native Horizon Mod

[Español](#español) | [English](#english)

---

# Español

## ⚠️ Advertencia: uso de inteligencia artificial

Este proyecto fue desarrollado utilizando herramientas de inteligencia artificial generativa.

**Desarrollo asistido por:**
- **OpenCode (DeepSeek V4.1 Flash Free):** generación de código, arquitectura, resolución de problemas y documentación.

La intervención de la inteligencia artificial se declara por transparencia.

## 📦 Descripción

Ponte tu propia skin de Minecraft en Horizon, sin depender de una cuenta. Funciona con nombres offline, en un jugador, en LAN y en servidores.

Es un mod **de cliente**: solo tú ves el cambio en tu personaje.

### Características

- **Comandos:** `/skin <archivo.png>` (o `/hskin`), `/skin on|off|slim|classic|reload`.
- **Configuración:** `Options → Mods → Custom Skin`, con interruptor, selección de brazos y recarga.
- **Almacenamiento persistente:** el PNG se copia a `horizon-mods/.horizon-mod-data/horizon_skin/skin.png`, por lo que seguirá disponible aunque muevas o elimines el archivo original.
- También puedes colocar directamente un archivo llamado `skin.png` en esa carpeta sin utilizar el comando.

## Límites

- Cambia el modelo del jugador local: cuerpo en tercera persona, brazo en primera persona, vista previa del inventario normal y vista previa de la pestaña **Inventario** del modo creativo.
- La cabeza que aparece en la lista de jugadores (Tab) sigue utilizando la skin que conoce el servidor o la cuenta.
- Los demás jugadores no verán el cambio.
- No sube nada a Mojang ni modifica tu skin en línea.

## Instalación

### Mediante el launcher (recomendado)

1. Actualiza Horizon a una versión Nightly que incluya soporte para mods nativos y los hooks utilizados por este mod (API ≥ 35; el Nightly `2026-10-31`, commit `d91cab6`, incluye API 42).

   La versión `v0.2.97` todavía no es compatible, ya que su binario no incluye `native_hooks.rs`.

2. Actualiza Horizon Launcher a la versión `0.2.31` o posterior. La versión `0.2.19` no reconoce `native-build`.

   Los mods nativos ya no tienen un interruptor independiente: se instalan desde la sección **Mods** o se compilan mediante `native-build`.

3. Construye el mod desde esta carpeta:

   ```bash
   horizon-launcher native-build carpeta/a/horizon_skin --trust-native-code
   ```

   La primera compilación descarga el código fuente de Horizon y puede tardar entre 20 y 30 minutos. Las siguientes compilaciones tardan unos minutos.

4. El launcher cambiará la instancia a la versión nativa. Inicia el juego.

5. Dentro del juego, carga tu skin:

   ```text
   /skin /ruta/a/tu_skin.png
   ```

### Mediante el paquete `.hnmod`

El paquete precompilado está disponible como:

```text
horizon_skin-1.0.0.hnmod
```

Se instala desde la sección **Mods** del launcher, que lo guarda en `horizon-native-mods/` y compila una versión nativa del juego.

**Importante:** no coloques este archivo en `horizon-mods/`, ya que esa carpeta está destinada a los antiguos paquetes `.hmod`.

## Estado del proyecto

<escape>**Archivado — Proyecto experimental.**</escape>

Este repositorio ha sido archivado porque se creó principalmente como un experimento de desarrollo de software utilizando inteligencia artificial generativa, y no como un proyecto serio o destinado a recibir mantenimiento continuo.

Aun así, se decidió compartir públicamente el resultado por si alguien encuentra útil el mod, quiere explorar su funcionamiento o simplemente desea probarlo.

No se garantizan futuras actualizaciones ni soporte.

---

# English

## ⚠️ AI Usage Disclaimer

This project was developed using generative artificial intelligence tools.

**AI-assisted development:**
- **OpenCode (DeepSeek V4.1 Flash Free):** code generation, architecture, problem-solving, and documentation.

AI involvement is disclosed for transparency.

## 📦 Description

Use your own Minecraft skin in Horizon without relying on an account. Works with offline usernames, singleplayer, LAN, and multiplayer servers.

This is a **client-side mod**: only you can see the changes to your character.

### Features

- **Commands:** `/skin <file.png>` (or `/hskin`), `/skin on|off|slim|classic|reload`.
- **Settings:** `Options → Mods → Custom Skin`, featuring a toggle, arm model selection, and reload option.
- **Persistent storage:** the PNG is copied to `horizon-mods/.horizon-mod-data/horizon_skin/skin.png`, so it remains available even if you move or delete the original file.
- You can also place a file named `skin.png` directly in that folder without using the command.

## Limitations

- Changes the local player model: third-person body, first-person arm, standard inventory preview, and the **Inventory** tab preview in Creative mode.
- The player head displayed in the player list (Tab) continues to use the skin known by the server or account.
- Other players will not see your custom skin.
- Nothing is uploaded to Mojang, and your online account skin is not modified.

## Installation

### Using the launcher (recommended)

1. Update Horizon to a Nightly build that includes native mod support and the hooks used by this mod (API ≥ 35; Nightly `2026-10-31`, commit `d91cab6`, includes API 42).

   Version `v0.2.97` is not compatible yet, as its binary does not include `native_hooks.rs`.

2. Update Horizon Launcher to version `0.2.31` or later. Version `0.2.19` does not recognize `native-build`.

   Native mods no longer have a separate toggle: they are installed through the **Mods** section or compiled using `native-build`.

3. Build the mod from this directory:

   ```bash
   horizon-launcher native-build path/to/horizon_skin --trust-native-code
   ```

   The first build downloads Horizon's source code and may take 20–30 minutes. Subsequent builds take only a few minutes.

4. The launcher will switch the instance to the native version. Start the game.

5. Inside the game, load your skin:

   ```text
   /skin /path/to/your_skin.png
   ```

### Using the `.hnmod` package

The prebuilt package is available as:

```text
horizon_skin-1.0.0.hnmod
```

Install it through the launcher's **Mods** section. The launcher stores it in `horizon-native-mods/` and compiles a native version of the game.

**Important:** do not place this file in `horizon-mods/`, as that directory is intended for legacy `.hmod` packages.

## Project Status

<escape>**Archived — Experimental project.**</escape>

This repository has been archived because it was primarily created as an experiment in AI-generated software development, rather than as a serious project intended for ongoing maintenance.

Nevertheless, the project is being shared publicly in case someone finds the mod useful, wants to explore how it works, or simply wants to give it a try.

No further updates or support are guaranteed.
