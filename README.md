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
