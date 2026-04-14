// use std::sync::LazyLock;
//
// use num_complex::Complex64;
// use ndarray::{arr2, Array2};
//
//
//
// // Pauli Matrices
// static PAULI_X: LazyLock<Array2<Complex64>> = LazyLock::new(|| { arr2(&[[Complex64::ZERO, Complex64::ONE ], 
//                                                                         [Complex64::ONE , Complex64::ZERO]])});
// static PAULI_Y: LazyLock<Array2<Complex64>> = LazyLock::new(|| { arr2(&[[Complex64::ZERO, -Complex64::I   ], 
//                                                                         [Complex64::I   ,  Complex64::ZERO]])});
// static PAULI_Z: LazyLock<Array2<Complex64>> = LazyLock::new(|| { arr2(&[[Complex64::ONE , Complex64::ZERO], 
//                                                                         [Complex64::ZERO, Complex64::ONE ]])});
