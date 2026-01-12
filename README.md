# Q#-rs: A Quantum Circuit Simulator in Rust

A comprehensive quantum computing simulator written in Rust, implementing the fundamental concepts from Microsoft's Q# documentation and Quantum Katas. This project serves as both a learning tool for quantum computing concepts and a functional quantum circuit simulator with support for multi-qubit systems, entanglement, and various quantum gates.

## Project Overview

This simulator was built as a hands-on approach to understanding quantum computing fundamentals while learning Rust. It implements a complete quantum computing stack from the ground up, including:

- **Complex number arithmetic** with full mathematical operations
- **Matrix operations** optimized for quantum computations
- **Quantum state representation** using state vectors
- **Complete quantum gate library** (Pauli, Hadamard, rotation, phase gates)
- **Multi-qubit systems** with entanglement support
- **Quantum measurement simulation** with probability visualization

## Features

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

## Implementation Highlights

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

## Educational Value

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

## Technical Details

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

## Primary Resources
- [Microsoft Quantum Katas](https://github.com/microsoft/QuantumKatas/tree/c15d99e4e505a67ef58c2c60ae50d11b0d09a443) - Home Repository
- [Complex Arithmetic Tutorial](https://github.com/microsoft/QuantumKatas/tree/c15d99e4e505a67ef58c2c60ae50d11b0d09a443/tutorials/ComplexArithmetic)
- [Linear Algebra Tutorial](https://github.com/microsoft/QuantumKatas/tree/c15d99e4e505a67ef58c2c60ae50d11b0d09a443/tutorials/LinearAlgebra)

### Additional Resources
- [Awesome Q#](https://github.com/ebraminio/awesome-qsharp) - Curated list of Q# resources
- [The Hitchhiker's Guide to Quantum Computing and Q#](https://learn.microsoft.com/en-us/archive/blogs/uk_faculty_connection/the-hitchhikers-guide-to-the-quantum-computing-and-q-blog)
- [Quirk - Quantum Circuit Simulator](https://algassert.com/quirk) - Interactive quantum circuit visualization
