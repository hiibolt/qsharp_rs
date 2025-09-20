# QSharp-RS: A Quantum Circuit Simulator in Rust

A comprehensive quantum computing simulator written in Rust, implementing the fundamental concepts from Microsoft's Q# documentation and Quantum Katas. This project serves as both a learning tool for quantum computing concepts and a functional quantum circuit simulator with support for multi-qubit systems, entanglement, and various quantum gates.

## 🎯 Project Overview

This simulator was built as a hands-on approach to understanding quantum computing fundamentals while learning Rust. It implements a complete quantum computing stack from the ground up, including:

- **Complex number arithmetic** with full mathematical operations
- **Matrix operations** optimized for quantum computations
- **Quantum state representation** using state vectors
- **Complete quantum gate library** (Pauli, Hadamard, rotation, phase gates)
- **Multi-qubit systems** with entanglement support
- **Quantum measurement simulation** with probability visualization

## 🚀 Features

### Core Quantum Operations
- **Single-qubit gates**: X, Y, Z (Pauli gates), H (Hadamard), S, T, rotation gates (Rx, Ry, Rz, R1)
- **Multi-qubit gates**: CNOT, SWAP, and extensible controlled gate framework
- **Quantum state preparation**: Support for basis states (|0⟩, |1⟩), superposition states (|+⟩, |-⟩), and arbitrary states
- **Measurement simulation**: Visual probability bars and phase information

### Mathematical Foundation
- **Complex number operations**: Addition, multiplication, division, conjugation, polar conversion
- **Matrix operations**: Multiplication, inversion, transposition, tensor products, eigenvalue/eigenvector computation
- **Quantum-specific operations**: Inner/outer products, normalization, unitary verification

### System Architecture
- **Modular design**: Separate modules for complex numbers, matrices, qubits, and quantum systems
- **Entanglement handling**: Automatic detection and management of entangled qubit states
- **Memory efficient**: Smart state representation that tracks individual qubits vs. entangled systems

## 🔬 Implementation Highlights

### Complex Number System
- Full complex arithmetic with optimized operations
- Polar/Cartesian coordinate conversion
- Support for arbitrary complex exponentiation

### Matrix Operations
- Efficient matrix multiplication for quantum gate operations
- Tensor product implementation for multi-qubit operations
- Eigenvalue/eigenvector computation for quantum state analysis
- Inverse tensor product for quantum state decomposition

### Quantum State Management
- Automatic entanglement detection and state consolidation
- Visual measurement output with probability bars and phase information
- Support for arbitrary quantum state preparation

### Gate Implementation
All major quantum gates are implemented with proper mathematical foundations:
- **Pauli Gates**: X (NOT), Y, Z (phase flip)
- **Hadamard Gate**: Creates superposition states
- **Phase Gates**: S (π/2 phase), T (π/4 phase), arbitrary phase rotation
- **Rotation Gates**: Arbitrary rotations around X, Y, Z axes
- **Multi-qubit Gates**: CNOT, SWAP with extensible framework

## 🧮 Example Usage

The main.rs file contains numerous examples implementing exercises from the Microsoft Quantum Katas:

```rust
// Create a Bell state (entangled pair)
let mut system = System::new();
system.allocate();
system.allocate();
system[0].unwrap_qubit().H();  // Put first qubit in superposition
system.CNOT(0, 1);             // Entangle with second qubit
system.dump();                 // Display the entangled state
```

```rust
// Prepare an arbitrary quantum state
let mut system = System::new();
let q = system.allocate();
q.R_y(theta);  // Rotate to desired amplitude
q.R_1(phi);    // Apply phase rotation
q.measure();   // Display measurement probabilities
```

## 🎓 Educational Value

This project implements solutions to exercises from:
- **Microsoft Quantum Katas**: Basic quantum operations and multi-qubit systems
- **Quantum State Preparation**: Various superposition and entangled states
- **Gate Composition**: Building complex operations from fundamental gates
- **Quantum Algorithms**: Foundational quantum computing algorithms

Each implementation includes detailed examples showing:
- How quantum gates affect qubit states
- Probability calculations for measurement outcomes
- Phase relationships in quantum superposition
- Entanglement creation and manipulation

## 🛠️ Technical Details

### Performance Considerations
- Efficient complex number operations using f32 precision
- Optimized matrix multiplication for common quantum gate sizes
- Memory-efficient representation of quantum states
- Lazy evaluation for entanglement operations

### Mathematical Accuracy
- Proper normalization of quantum states
- Accurate complex number arithmetic including phase calculations
- Correct implementation of quantum gate matrices
- Verified against known quantum computing results

## 🔮 Future Plans

The project was initially intended to expand into:
- **3D Visualization**: Using Bevy engine for quantum state visualization
- **Nix Packaging**: Complete development environment setup
- **Extended Gate Library**: Implementation of more advanced quantum gates
- **Quantum Algorithm Library**: Higher-level quantum algorithms and circuits

## 🚦 Getting Started

```bash
# Clone the repository
git clone <repository-url>
cd qsharp-rs

# Run the quantum simulator examples
cargo run

# Run specific examples
cargo test
```

The output will show various quantum operations, state preparations, and measurement results with visual probability indicators.

---

## 📚 Quick Reference

### Important Qubit States
![Qubit States](https://github.com/hiibolt/qsharp_rs/assets/91273156/a2426cf8-d12c-4968-9047-28b6be5f765b)
![Bloch Sphere](https://github.com/hiibolt/qsharp_rs/assets/91273156/c3502af3-13ba-49d0-9988-617e2bc041a8)

### Important Gates
![Pauli Gates](https://github.com/hiibolt/qsharp_rs/assets/91273156/06347b68-043b-42c4-abb9-d79952f6dda0)
![Hadamard Gate](https://github.com/hiibolt/qsharp_rs/assets/91273156/4eba89c6-e36c-49b7-aadf-07fe8262c7b9)
![Phase Gates](https://github.com/hiibolt/qsharp_rs/assets/91273156/a8cb7192-14b1-49aa-87c7-74ed1a2df072)
![Rotation Gates](https://github.com/hiibolt/qsharp_rs/assets/91273156/08a24839-be44-4eda-bd39-ea3463c82bbb)
![Multi-Qubit Gates](https://github.com/hiibolt/qsharp_rs/assets/91273156/b6d018c3-f7b1-4822-b78c-e053d78b5fe3)

#### Multi-Qubit Basis States
![Multi-Qubit States](https://github.com/hiibolt/qsharp_rs/assets/91273156/1ccf0381-f6d0-492f-b01a-d1e3c42cd522)

### Bra-Ket Notation
![Bra-Ket Basics](https://github.com/hiibolt/qsharp_rs/assets/91273156/99a6ca77-5579-4c18-96ea-bd22c8f8d12a)

- You can imagine ⟨x|y⟩ as the inner product of |x⟩ and |y⟩ because they're notationally connected _internally_ via the connector.
- Likewise, you can imagine |x⟩⟨y| as the outer product because |x⟩ and |y⟩ are not connected internally, so they're _outside_ the norm.

#### Gate Representation
This allows you to represent entire gates as a series of Dirac notations without ever technically re-entering matrix territory.

Basic example of how the X (inversion) gate can be represented solely with Dirac notation:
![X Gate Dirac](https://github.com/hiibolt/qsharp_rs/assets/91273156/ec63af1a-8ed4-40b5-8f53-fba47c5edb7c)

This allows fast computation without ever using matrices, which can be seen here flipping |0⟩ to |1⟩ in Dirac notation:
![Dirac Computation](https://github.com/hiibolt/qsharp_rs/assets/91273156/9498038e-d15a-418b-a36b-3d329af5f0f8)

### Gate Building
#### I-Gate usage to access only certain qubits
- Tensor Product with I-Gate as the base of the operation: Push up
- Tensor Product with I-Gate as the operand of the operation: Push back

#### Example Gate Usage
Given a system of 3 qubits, in order to ignore the first and third qubits and only apply the X-Gate to the second qubit, you would build a gate as follows:
$$(I \otimes X \otimes I) * Q_s$$

---

## 📖 References

### Primary Resources
- [Microsoft Quantum Katas](https://github.com/microsoft/QuantumKatas/tree/c15d99e4e505a67ef58c2c60ae50d11b0d09a443) - Home Repository
- [Complex Arithmetic Tutorial](https://github.com/microsoft/QuantumKatas/tree/c15d99e4e505a67ef58c2c60ae50d11b0d09a443/tutorials/ComplexArithmetic)
- [Linear Algebra Tutorial](https://github.com/microsoft/QuantumKatas/tree/c15d99e4e505a67ef58c2c60ae50d11b0d09a443/tutorials/LinearAlgebra)

### Additional Resources
- [Awesome Q#](https://github.com/ebraminio/awesome-qsharp) - Curated list of Q# resources
- [The Hitchhiker's Guide to Quantum Computing and Q#](https://learn.microsoft.com/en-us/archive/blogs/uk_faculty_connection/the-hitchhikers-guide-to-the-quantum-computing-and-q-blog)
- [Quirk - Quantum Circuit Simulator](https://algassert.com/quirk) - Interactive quantum circuit visualization
