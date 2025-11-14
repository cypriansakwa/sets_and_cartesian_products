# Sets and Cartesian Products in Rust (with Arkworks)

- This project provides a simple Rust implementation of basic set–theoretic operations
(union, intersection, difference, power set, and Cartesian product) on elements of the
BN254 scalar field (`ark_bn254::Fr`).

- Although the examples are small, the same operations form the foundation of algebraic
domains used in polynomial IOPs and STARK proof systems, where sets of field elements
determine evaluation domains for constraint polynomials.

## 📁 Project Structure
```bash
 src/
 └── main.rs   # Contains all set operations and examples
Cargo.toml    # Includes Arkworks dependencies

```
## ✨ Features

This project implements:
- Construction of sets as vectors of field elements.
- Union of two sets
- Intersection
- Difference (A \ B)
- Power Set of a finite set
- Cartesian Product A×B

All operations work on Vec<Fr> using helper functions.
