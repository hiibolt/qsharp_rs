# Rust Implementations of the Q# documentation

Intended to end as a quantum circuit simulator written in Rust, rendered in Bevy, and packaged by Nix.

## [Home Repository](https://github.com/microsoft/QuantumKatas/tree/c15d99e4e505a67ef58c2c60ae50d11b0d09a443)
[Complex Arithmetic](https://github.com/microsoft/QuantumKatas/tree/c15d99e4e505a67ef58c2c60ae50d11b0d09a443/tutorials/ComplexArithmetic)

[Linear Algebra](https://github.com/microsoft/QuantumKatas/tree/c15d99e4e505a67ef58c2c60ae50d11b0d09a443/tutorials/LinearAlgebra)

## Resources
[Awesome-Q#](https://github.com/ebraminio/awesome-qsharp)

[The Hitchhiker’s Guide to the Quantum Computing and Q# Blog](https://learn.microsoft.com/en-us/archive/blogs/uk_faculty_connection/the-hitchhikers-guide-to-the-quantum-computing-and-q-blog)

## Bra-Ket notation
### Basics:

![image](https://github.com/hiibolt/qsharp_rs/assets/91273156/99a6ca77-5579-4c18-96ea-bd22c8f8d12a)
- You can imagine <x|y> as the inner product of |x> and |y> because they're notationally connected _internally_ via the connector.
- Likewise, you can imagine |x> <y| as the outer product because of |x> and |y> because, well, they're not connected internally or pointing to eachother, so they're _outside_ the norm.
### Gate Representation
This allows you to represent entire gates as a series of Dirac notations without ever technically re-entering matrix territory. 

Basic example of how the X (inversion) gate can be represented solely with Dirac notation:
![image](https://github.com/hiibolt/qsharp_rs/assets/91273156/6e63cef5-0a98-4dfd-a277-c767eab48fe8)

## Ranting
- statements by the deranged:
![image](https://github.com/hiibolt/qsharp_rs/assets/91273156/b311f772-5f07-41e9-ba34-8e7d2053c680)

