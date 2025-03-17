<p align="center">
    <img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" align="center" width="30%">
</p>
<p align="center"><h1 align="center">PODCLI-RS.GIT</h1></p>
<p align="center">
	<em>Discover, download, enjoy - podcasts simplified!</em>
</p>
<p align="center">
	<img src="https://img.shields.io/github/license/atareao/podcli-rs.git?style=default&logo=opensourceinitiative&logoColor=white&color=0080ff" alt="license">
	<img src="https://img.shields.io/github/last-commit/atareao/podcli-rs.git?style=default&logo=git&logoColor=white&color=0080ff" alt="last-commit">
	<img src="https://img.shields.io/github/languages/top/atareao/podcli-rs.git?style=default&color=0080ff" alt="repo-top-language">
	<img src="https://img.shields.io/github/languages/count/atareao/podcli-rs.git?style=default&color=0080ff" alt="repo-language-count">
</p>
<p align="center"><!-- default option, no dependency badges. -->
</p>
<p align="center">
	<!-- default option, no dependency badges. -->
</p>
<br>

##  Table of Contents

- [ Overview](#-overview)
- [ Features](#-features)
- [ Project Structure](#-project-structure)
  - [ Project Index](#-project-index)
- [ Getting Started](#-getting-started)
  - [ Prerequisites](#-prerequisites)
  - [ Installation](#-installation)
  - [ Usage](#-usage)
  - [ Testing](#-testing)
- [ Project Roadmap](#-project-roadmap)
- [ Contributing](#-contributing)
- [ License](#-license)
- [ Acknowledgments](#-acknowledgments)

---

##  Overview

podcli-rs is an open-source, cross-platform command-line tool that simplifies podcast management. With an intuitive interface, it allows users to effortlessly discover, download, and enjoy their favorite podcasts right from the terminal. Built with Rust and designed for efficiency, podcli-rs is perfect for podcast enthusiasts and command-line aficionados seeking a streamlined podcast experience.

---

##  Features

|      | Feature         | Summary       |
| :--- | :---:           | :---          |
| ⚙️  | **Architecture**  | <ul><li>Modular design with separate `src/main.rs` and `src/podcast.rs` files</li><li>Uses `clap` for parsing command-line arguments</li><li>Asynchronous execution using `tokio` runtime</li></ul> |
| 🔩 | **Code Quality**  | <ul><li>Follows Rust best practices and idioms</li><li>Uses `tracing` for structured logging</li><li>Handles errors gracefully using Rust's `Result` type</li></ul> |
| 📄 | **Documentation** | <ul><li>Provides a `Makefile` with usage instructions</li><li>Documents dependencies and their purposes in `Cargo.toml`</li><li>Includes inline comments explaining key functionality</li></ul> |
| 🔌 | **Integrations**  | <ul><li>Retrieves podcast data from RSS feeds using `reqwest`</li><li>Parses XML data using `roxmltree`</li><li>Converts HTML to Markdown using `html2md`</li><li>Plays audio using `rodio`</li></ul> |
| 🧩 | **Modularity**    | <ul><li>Separates podcast-related functionality into `src/podcast.rs`</li><li>Defines clear boundaries between modules</li><li>Allows for easy extension and maintenance</li></ul> |
| 🧪 | **Testing**       | <ul><li>Includes unit tests for key functionality</li><li>Can be run using `cargo test`</li></ul> |
| ⚡️  | **Performance**   | <ul><li>Uses asynchronous programming with `tokio` for efficient execution</li><li>Compiles to native code for optimal performance</li></ul> |
| 🛡️ | **Security**      | <ul><li>Uses `reqwest` for secure HTTP communication</li><li>Validates and sanitizes user input</li></ul> |
| 📦 | **Dependencies**  | <ul><li>Manages dependencies using `cargo` and `Cargo.toml`</li><li>Uses popular and well-maintained Rust libraries</li><li>Includes `tracing` for logging, `clap` for argument parsing, `reqwest` for HTTP requests, and more</li></ul> |
| 🚀 | **Scalability**   | <ul><li>Can handle multiple concurrent requests using `tokio`</li><li>Designed to be easily extensible for new features and enhancements</li></ul> |

---

##  Project Structure

```sh
└── podcli-rs.git/
    ├── Cargo.toml
    ├── Dockerfile
    ├── Makefile
    ├── README.md
    ├── platform.sh
    └── src
        ├── main.rs
        └── podcast.rs
```


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
```

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




###  Usage
Run podcli-rs.git using the following command:
**Using `cargo`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Rust-000000.svg?style={badge_style}&logo=rust&logoColor=white" />](https://www.rust-lang.org/)

```sh
❯ cargo run
```


**Using `docker`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Docker-2CA5E0.svg?style={badge_style}&logo=docker&logoColor=white" />](https://www.docker.com/)

```sh
❯ docker run -it {image_name}
```


###  Testing
Run the test suite using the following command:
**Using `cargo`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Rust-000000.svg?style={badge_style}&logo=rust&logoColor=white" />](https://www.rust-lang.org/)

```sh
❯ cargo test
```


---
##  Project Roadmap

- [X] **`Task 1`**: <strike>Implement feature one.</strike>
- [ ] **`Task 2`**: Implement feature two.
- [ ] **`Task 3`**: Implement feature three.

---

##  Contributing

- **💬 [Join the Discussions](https://github.com/atareao/podcli-rs.git/discussions)**: Share your insights, provide feedback, or ask questions.
- **🐛 [Report Issues](https://github.com/atareao/podcli-rs.git/issues)**: Submit bugs found or log feature requests for the `podcli-rs.git` project.
- **💡 [Submit Pull Requests](https://github.com/atareao/podcli-rs.git/blob/main/CONTRIBUTING.md)**: Review open PRs, and submit your own PRs.

<details closed>
<summary>Contributing Guidelines</summary>

1. **Fork the Repository**: Start by forking the project repository to your github account.
2. **Clone Locally**: Clone the forked repository to your local machine using a git client.
   ```sh
   git clone https://github.com/atareao/podcli-rs.git
   ```
3. **Create a New Branch**: Always work on a new branch, giving it a descriptive name.
   ```sh
   git checkout -b new-feature-x
   ```
4. **Make Your Changes**: Develop and test your changes locally.
5. **Commit Your Changes**: Commit with a clear message describing your updates.
   ```sh
   git commit -m 'Implemented new feature x.'
   ```
6. **Push to github**: Push the changes to your forked repository.
   ```sh
   git push origin new-feature-x
   ```
7. **Submit a Pull Request**: Create a PR against the original project repository. Clearly describe the changes and their motivations.
8. **Review**: Once your PR is reviewed and approved, it will be merged into the main branch. Congratulations on your contribution!
</details>

<details closed>
<summary>Contributor Graph</summary>
<br>
<p align="left">
   <a href="https://github.com{/atareao/podcli-rs.git/}graphs/contributors">
      <img src="https://contrib.rocks/image?repo=atareao/podcli-rs.git">
   </a>
</p>
</details>

---

##  License

This project is protected under the [SELECT-A-LICENSE](https://choosealicense.com/licenses) License. For more details, refer to the [LICENSE](https://choosealicense.com/licenses/) file.

---

##  Acknowledgments

- List any resources, contributors, inspiration, etc. here.

---
