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
## 🧮 Mathematical Background

This program mirrors fundamental set operations:
| Mathematical Concept | Operation Name        | Symbol / Example   |
| -------------------- | --------------------- | ------------------ |
| Union                | `set_union()`         | ( A \cup B )       |
| Intersection         | `set_intersection()`  | ( A \cap B )       |
| Difference           | `set_difference()`    | ( A \setminus B )  |
| Power Set            | `power_set()`         | ( \mathcal{P}(A) ) |
| Cartesian Product    | `cartesian_product()` | ( A \times B )     |

These operations mirror how STARKs manipulate evaluation domains for polynomials.

## 🚀 Running the Code
1. Clone the repository
   ```bash
    git clone https://github.com/cypriansakwa/sets_and_cartesian_products.git
    cd sets_and_cartesian_products

   ```
2. Build and run
   ```bash
    cargo run
   ```
## 📦 Dependencies

This project uses:
```toml
 ark-std = "0.5.0"
ark-ff = "0.5.0"
ark-bn254 = "0.5.0"
```
Ensure you have the latest stable Rust toolchain.
## 🔧 How It Works
Each operation is implemented as a pure Rust function:
- **Union** uses a `BTreeSet` to maintain uniqueness
- **Intersection** filters elements present in both sets
- **Difference** removes elements of B from A
- **Power Set** is generated using bitmasks
- **Cartesian Product** is built via nested iterators

All operations are compatible with Arkworks field elements.

## 📘 Educational Note (STARKs Context)
In STARK proof systems:
 - Evaluation domains are sets of field elements.
 - Union, intersection, and products create the polynomial domains.
 - Cartesian products define multivariate polynomial domains.
 - Power sets appear in combinatorial constraint constructions.

Thus, learning these set operations is foundational for understanding modern ZK proofs.

## Diagram: How Sets Map to STARK Evaluation Domains
```less
                   +---------------------------+
                  |        A Set A           |
                  |  {a₀, a₁, a₂, ...}       |
                  +-------------+-------------+
                                |
                                |  Elements of a field (e.g., Fr)
                                v
                  +---------------------------+
                  |  Evaluation Domain D_A    |
                  |  Points where trace       |
                  |  polynomials are sampled  |
                  +-------------+-------------+

         +------------------+        +------------------+
         |      Set A       |        |      Set B       |
         | {a₀, a₁, a₂}     |        | {b₀, b₁, b₂}     |
         +---------+--------+        +---------+--------+
                   |                           |
                   | union/intersection/etc    |
                   v                           v
           +---------------+          +---------------------+
           | Combined or   |          |  Restricted or      |
           | Derived Set   |          |  Filtered Domain    |
           +-------+-------+          +----------+----------+
                   |                             |
                   +-------------+---------------+
                                 |
                                 v
                     +-------------------------+
                     |   Evaluation Domain D   |
                     | for constraint polynomials
                     |   in IOPs / FRI / AIR   |
                     +-------------------------+

```
**Interpretation**
**Set A or B**

Simple sets of field elements:
 - {0, 1, 2}
 - {2, 3, 4}

**Evaluation Domain**
A set of points where polynomials are:
 - evaluated,
 - queried by the verifier,
 - folded during FRI.
Operations correspond to STARK concepts

**Operations correspond to STARK concepts**
| Set Operation         | Role in STARKs                                         |
| --------------------- | ------------------------------------------------------ |
| **Union**             | Combine domains for multiple constraints               |
| **Intersection**      | Identify shared evaluation points                      |
| **Difference**        | Exclude restricted points (e.g., boundary constraints) |
| **Power set**         | Model combinatorial constraint structures              |
| **Cartesian product** | Build multivariate domains (e.g., 2D traces)           |

