# ImgX v1.0 : Image Processing Engine 🚀

A high-performance concurrent image processing engine built with Rust.

## 📌 Features in v1.0 : 

* Convert to grayscale
* Rotate images
* Adjust brightness
* Batch process entire folders
* Concurrent image processing using threads
* CLI support for professional workflow

## ⚡ Built With

* Rust
* Image crate
* Rayon

## 🧠 Architecture

Clean modular architecture:

src/
├── filters/
├── processor.rs
├── commands
└── main.rs

## 🚀 Why This Project

This project demonstrates:

* Systems programming in Rust
* Thread-safe concurrency
* File handling
* Trait-based architecture
* Error handling
* Production-grade modular design

## 📦 Installation

```bash
git clone <https://github.com/abhichauhan12/imgX.git>
cd imgX
cargo build --release
```

## ▶️ Run

```bash
cargo run -- grayscale input.jpg output.jpg 
```

## 📂 Example Commands

### Grayscale

```bash
cargo run -- grayscale input.jpg output.jpg
```

### Rotate

```bash
cargo run -- rotate input.jpg output.jpg 90
```

### Brightness

```bash
cargo run -- brighten input.jpg output.jpg 40
```

### Batch Process Folder

```bash
cargo run -- batch-grayscale ./images
```

## ⚙️ Concurrency

Multiple images are processed in parallel using Rust threads for maximum performance.

## 📈 Future Improvements

* Implement Different filters
* GUI support
* GPU acceleration
* Custom filters

## 👨‍💻 Author
~Abhishek Chauhan
