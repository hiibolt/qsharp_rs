# Rust Implementations of the Q# documentation

Intended to end as a quantum circuit simulator written in Rust, rendered in Bevy, and packaged by Nix.

This project was primarily a means to learn Rust and choice topics in quantum compilers.

## Cheat Sheets

### Important Qubit States:
![image](https://github.com/hiibolt/qsharp_rs/assets/91273156/a2426cf8-d12c-4968-9047-28b6be5f765b)
![image](https://github.com/hiibolt/qsharp_rs/assets/91273156/c3502af3-13ba-49d0-9988-617e2bc041a8)

### Important Gates:
![image](https://github.com/hiibolt/qsharp_rs/assets/91273156/06347b68-043b-42c4-abb9-d79952f6dda0)
![image](https://github.com/hiibolt/qsharp_rs/assets/91273156/4eba89c6-e36c-49b7-aadf-07fe8262c7b9)
![image](https://github.com/hiibolt/qsharp_rs/assets/91273156/a8cb7192-14b1-49aa-87c7-74ed1a2df072)
![image](https://github.com/hiibolt/qsharp_rs/assets/91273156/08a24839-be44-4eda-bd39-ea3463c82bbb)
![image](https://github.com/hiibolt/qsharp_rs/assets/91273156/b6d018c3-f7b1-4822-b78c-e053d78b5fe3)

#### Multi-Qubit Basis States:
![image](https://github.com/hiibolt/qsharp_rs/assets/91273156/1ccf0381-f6d0-492f-b01a-d1e3c42cd522)

## [Home Repository](https://github.com/microsoft/QuantumKatas/tree/c15d99e4e505a67ef58c2c60ae50d11b0d09a443)
[Complex Arithmetic](https://github.com/microsoft/QuantumKatas/tree/c15d99e4e505a67ef58c2c60ae50d11b0d09a443/tutorials/ComplexArithmetic)

[Linear Algebra](https://github.com/microsoft/QuantumKatas/tree/c15d99e4e505a67ef58c2c60ae50d11b0d09a443/tutorials/LinearAlgebra)

## Resources
[Awesome-Q#](https://github.com/ebraminio/awesome-qsharp)

[The Hitchhiker’s Guide to the Quantum Computing and Q# Blog](https://learn.microsoft.com/en-us/archive/blogs/uk_faculty_connection/the-hitchhikers-guide-to-the-quantum-computing-and-q-blog)

[Fantastic Simulator - JS](https://algassert.com/quirk)

## Bra-Ket notation
### Basics:

![image](https://github.com/hiibolt/qsharp_rs/assets/91273156/99a6ca77-5579-4c18-96ea-bd22c8f8d12a)
- You can imagine <x|y> as the inner product of |x> and |y> because they're notationally connected _internally_ via the connector.
- Likewise, you can imagine |x> <y| as the outer product because of |x> and |y> because, well, they're not connected internally or pointing to eachother, so they're _outside_ the norm.
### Gate Representation
This allows you to represent entire gates as a series of Dirac notations without ever technically re-entering matrix territory. 

Basic example of how the X (inversion) gate can be represented solely with Dirac notation:
![image](https://github.com/hiibolt/qsharp_rs/assets/91273156/ec63af1a-8ed4-40b5-8f53-fba47c5edb7c)

This allows fast computation without ever using matrices, which can be seen here flipping |0> to |1> in Dirac notation:
![image](https://github.com/hiibolt/qsharp_rs/assets/91273156/9498038e-d15a-418b-a36b-3d329af5f0f8)

## Gate-Building
### I-Gate usage to access only certain qubits
- Tensor Product with I-Gate as the base of the operation: Push up
- Tensor Product with I-Gate as the operand of the operation: Push back
### Example Gate Usage:
Given a system of 3 qubits, in order to ignore the first and third qubits and only apply the X-Gate to the second qubit, you would build a gate as follows:
$$(I \otimes X \otimes I) * Q_s$$
