# podcli-rs

<p align="center">
	<a href="https://github.com/atareao/podcli-rs"><img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" width="120" alt="podcli logo"/></a>
</p>

<p align="center">
	<strong>Gestiona y reproduce podcasts desde la línea de comandos — sencillo, rápido y multiplataforma.</strong>
</p>

<p align="center">
	<img src="https://img.shields.io/github/actions/workflow/status/atareao/podcli-rs/ci.yml?branch=main&style=flat-square" alt="build"/>
	<img src="https://img.shields.io/github/last-commit/atareao/podcli-rs?style=flat-square" alt="last-commit"/>
	<img src="https://img.shields.io/github/license/atareao/podcli-rs?style=flat-square" alt="license"/>
	<img src="https://img.shields.io/crates/v/podcli-rs?style=flat-square" alt="crates.io"/>
	<img src="https://docs.rs/podcli-rs/badge.svg" alt="docs.rs"/>
</p>

---

## Contenido

- [Descripción](#descripción)
- [Características](#características)
- [Instalación](#instalación)
- [Uso](#uso)
- [Contribuir](#contribuir)
- [Licencia](#licencia)

---

## Descripción

`podcli-rs` es una utilidad de línea de comandos escrita en Rust para descubrir, descargar y reproducir podcasts de forma eficiente. Está diseñada para usuarios que prefieren trabajar desde la terminal y para entornos donde se busca un binario ligero y rápido.

---

## Características

- Cliente CLI multiplataforma compilado en Rust.
- Gestión y descarga de episodios desde feeds RSS.
- Reproducción local de audio usando bibliotecas nativas.
- Estructura modular para facilitar extensiones.
- Manejo de errores y logging configurable.

---

## Estructura del proyecto

```text
podcli-rs/
├── Cargo.toml   # configuración y dependencias
├── src/
│   ├── main.rs
│   └── podcast.rs
├── Dockerfile
├── Makefile
└── README.md
```

---

## Instalación

Requisitos previos: `rust` (con `cargo`) o `docker`.

Opciones de instalación:

- Desde el código (instalación local):

```sh
git clone https://github.com/atareao/podcli-rs.git
cd podcli-rs
cargo build --release
# El binario quedará en target/release/podcli
```

- Usando Docker:

```sh
docker build -t podcli-rs .
docker run --rm -it podcli-rs
```

---

## Uso

Ejemplos básicos:

- Ejecutar desde Cargo (modo desarrollo):

```sh
cargo run -- <subcomando> [opciones]
```

- Ejecutable compilado (release):

```sh
./target/release/podcli <subcomando> [opciones]
```

Consulta `--help` para ver comandos y opciones disponibles.

---

## Contribuir

Las contribuciones son bienvenidas. Para contribuir:

1. Haz fork del repositorio.
2. Crea una rama con tu feature o corrección.
3. Envía un pull request describiendo los cambios.

Lee `CONTRIBUTING.md` (si existe) para normas específicas.

---

## Licencia

Este proyecto incluye un archivo `LICENSE` en la raíz. Consulta el fichero para los detalles legales.

---

## Contacto

Para dudas o reportes de errores utiliza el sistema de issues del repositorio.

## Estructura del proyecto

```text
podcli-rs/
├── Cargo.toml   # configuración y dependencias
├── src/
│   ├── main.rs
│   └── podcast.rs
├── Dockerfile
├── Makefile
└── README.md
```

- Gestión y descarga de episodios desde feeds RSS.
- Reproducción local de audio usando bibliotecas nativas.
- Estructura modular para facilitar extensiones.
- Manejo de errores y logging configurable.

---

    ├── Dockerfile
    ├── Makefile
    ├── README.md
    ├── platform.sh
    └── src
        ├── main.rs
        └── podcast.rs

````


###  Project Index
<details open>
	<summary><b><code>PODCLI-RS.GIT/</code></b></summary>
	<details> <!-- __root__ Submodule -->
		<summary><b>__root__</b></summary>
		<blockquote>
			<table>
			<tr>
				<td><b><a href='https://github.com/atareao/podcli-rs.git/blob/master/platform.sh'>platform.sh</a></b></td>
				<td>- Sets platform-dependent variables during the Docker build process based on the target platform<br>- Writes the appropriate Rust target triple and library directory path to files, enabling cross-platform compilation<br>- Supports x86_64, ARM64, ARMv7, and ARMv6 architectures, defaulting to x86_64 for unknown platforms<br>- Facilitates building the project for multiple platforms using a single Dockerfile.</td>
			</tr>
			<tr>
				<td><b><a href='https://github.com/atareao/podcli-rs.git/blob/master/Makefile'>Makefile</a></b></td>
				<td>- Makefile automates the build and installation process for the podcli project<br>- It allows building the project in release or debug mode, with corresponding executable names<br>- The Makefile also provides a help target to display usage instructions<br>- By running make with appropriate targets, users can easily compile and install the podcli executable to their local bin directory.</td>
			</tr>
			<tr>
				<td><b><a href='https://github.com/atareao/podcli-rs.git/blob/master/Dockerfile'>Dockerfile</a></b></td>
				<td>- Dockerfile builds a lightweight, multi-platform podcli application using Rust and musl<br>- It compiles the source code in a builder stage, targeting the specified platform<br>- The resulting binary is then copied into a minimal Alpine-based runtime image, allowing the application to run efficiently in various environments without unnecessary dependencies.</td>
			</tr>
			<tr>
				<td><b><a href='https://github.com/atareao/podcli-rs.git/blob/master/Cargo.toml'>Cargo.toml</a></b></td>
				<td>- Defines the podcli package configuration, specifying dependencies and binary settings<br>- It sets up the project structure, enabling functionality for parsing and converting data, handling user input, making HTTP requests, and playing audio<br>- The main binary is defined as podcli, with its entry point in the src/main.rs file.</td>
			</tr>
			</table>
		</blockquote>
	</details>
	<details> <!-- src Submodule -->
		<summary><b>src</b></summary>
		<blockquote>
			<table>
			<tr>
				<td><b><a href='https://github.com/atareao/podcli-rs.git/blob/master/src/podcast.rs'>podcast.rs</a></b></td>
				<td>- Defines the Podcast and Episode structs for parsing and managing podcast RSS feeds<br>- Retrieves podcast data asynchronously, allowing users to access episode information, display formatted descriptions, and download episodes<br>- Provides a convenient interface for interacting with podcast content within the larger application architecture.</td>
			</tr>
			<tr>
				<td><b><a href='https://github.com/atareao/podcli-rs.git/blob/master/src/main.rs'>main.rs</a></b></td>
				<td>- main.rs serves as the entry point for the podcast CLI application<br>- It handles command-line arguments, initializes logging, and orchestrates the execution flow<br>- The file defines the application's structure, including subcommands for listing episodes and running an interactive mode<br>- It also contains functions for downloading and playing podcast episodes, as well as managing user interactions through a menu-driven interface.</td>
			</tr>
			</table>
		</blockquote>
	</details>
</details>

---
##  Getting Started

###  Prerequisites

Before getting started with podcli-rs.git, ensure your runtime environment meets the following requirements:

- **Programming Language:** Rust
- **Package Manager:** Cargo
- **Container Runtime:** Docker


###  Installation

Install podcli-rs.git using one of the following methods:

**Build from source:**

1. Clone the podcli-rs.git repository:
```sh
❯ git clone https://github.com/atareao/podcli-rs.git
````

2. Navigate to the project directory:

```sh
❯ cd podcli-rs.git
```

3. Install the project dependencies:

**Using `cargo`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Rust-000000.svg?style={badge_style}&logo=rust&logoColor=white" />](https://www.rust-lang.org/)

```sh
❯ cargo build
```

**Using `docker`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Docker-2CA5E0.svg?style={badge_style}&logo=docker&logoColor=white" />](https://www.docker.com/)

```sh
❯ docker build -t atareao/podcli-rs.git .
```

### Usage

Run podcli-rs.git using the following command:
**Using `cargo`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Rust-000000.svg?style={badge_style}&logo=rust&logoColor=white" />](https://www.rust-lang.org/)

```sh
❯ cargo run
```

**Using `docker`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Docker-2CA5E0.svg?style={badge_style}&logo=docker&logoColor=white" />](https://www.docker.com/)

````sh
## Instalación

Requisitos previos: `rust` (con `cargo`) o `docker`.

Opciones de instalación:

- Desde el código (instalación local):

```sh
git clone https://github.com/atareao/podcli-rs.git
cd podcli-rs
cargo build --release
# El binario quedará en target/release/podcli
````

- Usando Docker:

```sh
docker build -t podcli-rs .
docker run --rm -it podcli-rs
```

---

## Uso

Ejemplos básicos:

- Ejecutar desde Cargo (modo desarrollo):

```sh
cargo run -- <subcomando> [opciones]
```

- Ejecutable compilado (release):

```sh
./target/release/podcli <subcomando> [opciones]
```

Consulta `--help` para ver comandos y opciones disponibles.

---

## Acknowledgments

- List any resources, contributors, inspiration, etc. here.

---
