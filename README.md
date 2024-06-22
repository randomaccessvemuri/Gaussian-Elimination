# Gaussian Elimination:

As part of learning from Gilbert Strang's amazing "Linear Algebra and Its Applications", I've implemented Gaussian implementation.


## Algorithm Steps

### Input
- An augmented matrix \( A \) of size \( n \times (n+1) \) representing the system of linear equations \( Ax = b \).

### Output
- Solution vector \( x \) if the system has a unique solution, or a notification of a singular or inconsistent system.

### Steps

1. **Forward Elimination:**
    1.1. For each row \( i \) from \( 0 \) to \( n-1 \):
        1.1.1. **Pivot Selection** (Partial Pivoting):
            - Find the row \( k \) with the maximum absolute value in column \( i \) below or at the diagonal (i.e., among \( i \) to \( n-1 \)).
            - Swap row \( i \) with row \( k \) to bring the largest absolute value to the pivot position.
        1.1.2. **Pivot Validation**:
            - If the pivot element \( A[i][i] \) is zero after swapping, the matrix is singular (no unique solution).
        1.1.3. **Elimination**:
            - For each row \( j \) below row \( i \) (i.e., \( j = i+1 \) to \( n-1 \)):
                - Compute the elimination factor \( f = \frac{A[j][i]}{A[i][i]} \).
                - For each column \( k \) from \( i \) to \( n \) (including the RHS):
                    - Update \( A[j][k] = A[j][k] - f \cdot A[i][k] \).

2. **Back Substitution:**
    2.1. Initialize a solution vector \( x \) of size \( n \).
    2.2. For each row \( i \) from \( n-1 \) to \( 0 \) (in reverse order):
        2.2.1. Compute the sum of known values:
            - \( \text{sum} = \sum_{j=i+1}^{n-1} A[i][j] \cdot x[j] \).
        2.2.2. Compute the value of the unknown \( x[i] \):
            - \( x[i] = \frac{A[i][n] - \text{sum}}{A[i][i]} \).

3. **Result:**
    - The vector \( x \) contains the solutions to the system of linear equations \( Ax = b \).
