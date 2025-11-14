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

All operations work on `Vec<Fr>` using helper functions.
## 📌 Example Output

Running the program (using the sample sets A={0,1,2} and B={2,3,4}) produces:
```sql
 --- 1. Sets and Cartesian Products ---
Set A: [0, 1, 2]
Set B: [2, 3, 4]
A ∪ B = [1, 2, 4, 0, 3]
A ∩ B = [2]
A \ B = [0, 1]
|P(A)| = 8 (Power set of A)
A × B has 9 pairs
Sample of A × B: [(0, 2), (0, 3), (0, 4), (1, 2), (1, 3)]

```
