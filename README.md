# podcli-rs

<p align="center">
	<a href="https://github.com/atareao/podcli-rs"><img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" width="120" alt="podcli logo"/></a>
</p>

<p align="center">
	<strong>Gestiona y reproduce podcasts desde la línea de comandos — sencillo, rápido y multiplataforma.</strong>
</p>

<p align="center">
	<img src="https://img.shields.io/github/actions/workflow/status/atareao/podcli-rs/release.yml?branch=main&style=flat-square" alt="build"/>
	<img src="https://img.shields.io/github/last-commit/atareao/podcli-rs?style=flat-square" alt="last-commit"/>
	<img src="https://img.shields.io/github/license/atareao/podcli-rs?style=flat-square" alt="license"/>
	<img src="https://img.shields.io/crates/v/podcli-rs?style=flat-square" alt="crates.io"/>
</p>

---

## Contenido

- [Descripción](#descripción)
- [Características](#características)
- [Instalación](#instalación)
- [Uso](#uso)
  - [Comandos disponibles](#comandos-disponibles)
  - [Modo lista](#modo-lista)
  - [Modo interactivo](#modo-interactivo)
- [Contribuir](#contribuir)
- [Licencia](#licencia)

---

## Descripción

`podcli-rs` es una utilidad de línea de comandos escrita en Rust para descubrir, descargar y reproducir podcasts de forma eficiente. Está diseñada para usuarios que prefieren trabajar desde la terminal y para entornos donde se busca un binario ligero y rápido.

---

## Características

- Cliente CLI multiplataforma compilado en Rust (binario único)
- Gestión y descarga de episodios desde feeds RSS
- Reproducción local de audio usando `rodio`
- Interfaz interactiva para navegar episodios
- Salida en JSON para integración con otros tools
- Logging configurable vía `RUST_LOG`

---

## Instalación

### Prerrequisitos

- **Desde código fuente:** Rust y Cargo
- **Linux:** `libasound2-dev` y `pkg-config` (para audio)
- **Docker:** alternativa sin instalar dependencias

### Opciones de instalación

**Desde código fuente:**

```sh
git clone https://github.com/atareao/podcli-rs.git
cd podcli-rs
cargo build --release
# El binario queda en target/release/podcli
```

**Usando Cargo (si está publicado en crates.io):**

```sh
cargo install podcli
```

**Usando Docker:**

```sh
docker build -t podcli .
docker run --rm -it podcli
```

---

## Uso

### Comandos disponibles

```sh
podcli --help
```

Salida:
```
Gestiona y reproduce podcasts desde la línea de comandos

Usage: podcli [OPTIONS] <COMMAND>

Commands:
  list          Lista episodios de un podcast
  interactive  Modo interactivo para navegar episodios
  help         Print this message or the help of the given subcommand(s)

Options:
  -d, --debug  Activa modo debug
  -h, --help   Print help
  -V, --version Print version
```

### Modo lista

Lista episodios de un feed RSS:

```sh
# Listar todos los episodios
podcli list --url https://feeds.simplecast.com/54nVymd2

# Listar los primeros 5 episodios
podcli list --url https://feeds.simplecast.com/54nVymd2 --first 5

# Listar los últimos 3 episodios
podcli list --url https://feeds.simplecast.com/54nVymd2 --last 3

# Salida en JSON
podcli list --url https://feeds.simplecast.com/54nVymd2 --json
```

### Modo interactivo

Navega episodios con un menú interactivo:

```sh
podcli interactive --url https://feeds.simplecast.com/54nVymd2
```

Opciones en modo interactivo:
1. **List episodes** - Muestra todos los episodios
2. **Get episode** - Muestra detalles de un episodio específico
3. **Play episode** - Descarga y reproduce un episodio
4. **Reload** - Recarga el feed RSS
5. **Exit** - Salir

### Variables de entorno

```sh
# Cambiar nivel de logging (por defecto: DEBUG)
RUST_LOG=info podcli list --url https://example.com/feed
RUST_LOG=debug podcli interactive --url https://example.com/feed
```

Niveles disponibles: `trace`, `debug`, `info`, `warn`, `error`

---

## Desarrollo

### Estructura del proyecto

```
podcli-rs/
├── Cargo.toml       # configuración y dependencias
├── src/
│   ├── main.rs      # CLI entrypoint, comandos, reproducción
│   └── podcast.rs   # Parser RSS, tipos Podcast/Episode
├── Dockerfile       # Multi-arch build
├── Makefile         # Build helpers
└── README.md
```

### Comandos útiles

```sh
# Compilar en modo debug
cargo build

# Ejecutar en desarrollo
cargo run -- list --url https://feeds.simplecast.com/54nVymd2

# Ejecutar tests
cargo test

# Compilar release
cargo build --release
```

### Dependencias principales

- `clap` - CLI parser (derive mode)
- `rodio` - Audio playback
- `roxmltree` - RSS/XML parsing
- `reqwest` - HTTP client
- `tokio` - Async runtime
- `termimad` - Terminal markdown rendering
- `inquire` - Interactive prompts

---

## Contribuir

Las contribuciones son bienvenidas. Para contribuir:

1. Haz fork del repositorio
2. Crea una rama con tu feature o corrección (`git checkout -b feature/nueva-funcionalidad`)
3. Haz commit de tus cambios (`git commit -am 'Añade nueva funcionalidad'`)
4. Push a la rama (`git push origin feature/nueva-funcionalidad`)
5. Envía un pull request describiendo los cambios

---

## Licencia

Este proyecto está bajo la licencia MIT. Consulta el archivo `LICENSE` para los detalles.

---

## Contacto

Para dudas o reportes de errores utiliza el [sistema de issues](https://github.com/atareao/podcli-rs/issues) del repositorio.
# Test CI
