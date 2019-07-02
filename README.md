<p align="center">
  <a href="" rel="noopener">
 <img width=200px height=200px src="https://i.imgur.com/hKwFkEZ.png" alt="Project logo"></a>
</p>

<h3 align="center">Novus Command Line Edition.</h3>

<div align="center">

  ![Forks](https://img.shields.io/github/forks/Official-polar-team/NovusCLI.svg) 
  ![GitHub Issues](https://img.shields.io/github/issues/Official-polar-team/NovusCLI.svg)
  ![GitHub Stars](https://img.shields.io/github/stars/Official-polar-team/NovusCLI.svg)
  ![License](https://img.shields.io/github/license/Official-polar-team/NovusCLI.svg)

</div>

---

<p align="center"> The best APT front-end for macOS (Or Linux)
    <br> 
</p>

## 📝 Table of Contents
- [About](#about)
- [Getting Started](#getting_started)
- [Usage](#usage)
- [Built Using](#built_using)
- [Contributing](../CONTRIBUTING.md)
- [Authors](#authors)
- [Acknowledgments](#acknowledgement)

## 🧐 About <a name = "about"></a>
NovusCLI is a lite version of the full Novus package manager (Coming later this year) that allows users to start with a simple syntax, for their favorite APT package manager.

## 🏁 Getting Started <a name = "getting_started"></a>
Getting started for contributing to NovusCLI is very easy, you just need to follow the instructions below if you wish to build from source.

### Prerequisites
In order to start building NovusCLI, you are going to need to have rustup and cargo installed (Installing rustup automatically installs cargo so just worry about rustup). You can get rustup [here](https://www.rust-lang.org/tools/install/).

NovusCLI uses Rust's Rust compiler so this is all you need, but getting [MacPT](https://github.com/Official-polar-team/MacPT) is higly recommended.

### Building

For installation NovusCLI makes it really easy:

* cd into the nvs directory.
* Run cargo build --release
* Enjoy Novus commnad line edition!

This are all the steps that you need to follow to get NovusCLI running on your system!

## 🎈 Usage <a name="usage"></a>
NovusCLI has a memorable set of commands that will allow you to use it everyday as your syntax for MacPT:

* ```nvs search [query]``` - Searches for a package in your resporitories
* ```list```  - Lists all packages in your resporitories
* ```nvs install [package]``` - Installs a package 
* ```reinstall [package]``` - Reinstalls a package
* ``` remove [package]``` - Removes a package
* ```edit-sources``` - Opens the APT repo editor
* ```autoremove``` - Removes unneeded packages (orphans)
* ```update``` - Update the repo lists
* ```upgrade``` - Upgrade all packages
* ```clean``` - Clear the download cache
* ```help``` - Open a help page similar to this one
* ```about``` - View legal information

Using NovusCLI is a lot of fun, as you see, and it doesnt require you to use the "sudo" prefix.

## ⛏️ Built Using <a name = "built_using"></a>
- [RustLang](https://www.rust-lang.org/) - Main programming language.
- [MacPT](https://github.com/DiegoMagdaIeno/MacPT) - Backend/Package manager.
- [Project Serna](https://sernarepo.com/) - Main repository.

## ✍️ Authors <a name = "authors"></a>
- [@DiegoMagdaleno](https://github.com/DiegoMagdaIeno) - Initial C++ build.
- [@Diatrus](https://github.com/Diatrus) - Initial C++ build.
- [@SmushyTaco](https://github.com/realSmushyTaco) - Rust rewrite

See also the list of [contributors](https://github.com/kylelobo/The-Documentation-Compendium/contributors) who participated in this project.

## 🎉 Acknowledgements <a name = "acknowledgement"></a>
- [PacAPT by ICY](https://github.com/icy/pacapt)
