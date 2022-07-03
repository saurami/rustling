# Rust

Covers general purpose programming with Rust.

Sources:

1. [Microsoft Learning Path][1]
2. [Beginning Rust: From Novice to Professional][2]

Setup (on Windows 11 using Chocolatey)

+ **Install Visual C++ build tools**

  Rust requires the Microsoft C++ [build tools][3] for Visual Studio 2013 or later. Ensure that the following optional packages are also installed:

  1. MSVC
  2. Windows SDK
  3. C++ CMake tools for Windows
  4. Testing tools core features
  5. C++ AddressSanitizer

+ **Install Rust**

  [rustup][4] is the recommended way to install Rust

+ Verify compiler and package manager

  Run the below commands in Command Prompt to ensure that the rust compiler `rustc` and package manager `cargo` have been installed.
  
  ```
  rustc --version
  cargo --version
  ```

+ **Execute Script**

  ```
  rustc hello.rs
  hello.exe
  ```

+ **Miscellaneous**

  1. Install a text editor
  2. Install rust-analyzer

[1]: https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/
[2]: https://www.oreilly.com/library/view/beginning-rust-from/9781484234686/
[3]: https://visualstudio.microsoft.com/visual-cpp-build-tools/
[4]: https://www.rust-lang.org/tools/install
