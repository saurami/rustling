# Rust

Covers general purpose programming with Rust.

Sources:

1. [Microsoft Learning Path][1]
2. [Beginning Rust: From Novice to Professional][2]

Setup (on Windows 11 using Chocolatey)

+ **Install Visual C++ build tools**

  Rust requires the Microsoft C++ build tools for Visual Studio 2013 or later.

  `choco install visualstudio2019buildtools`

+ **Install Rust**

  `rustup` is the recommended way to [install][3] Rust

+ Verify compiler and package manager

  Run the below command to ensure that the rust compiler `rustc` and package manager `cargo` have been installed.
  
  ```
  rustc --version
  cargo --version
  ```

[1]: https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/
[2]: https://www.oreilly.com/library/view/beginning-rust-from/9781484234686/
[3]: https://www.rust-lang.org/tools/install
