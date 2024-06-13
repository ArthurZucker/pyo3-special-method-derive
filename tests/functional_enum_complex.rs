use pyo3::pyclass;
use pyo3_special_method_derive::{DirHelper, StrReprHelper};

#[pyclass]
#[derive(DirHelper, StrReprHelper)]
#[allow(dead_code)]
enum Tester {
    Alpha { x: u32 },
    Beta { x: u32, y: u32 },
    Gamma { x: u32, y: u32, z: u32 },
}

#[test]
fn test_with_dir() {
    let res = Tester::Alpha { x: 12 }.__dir__();
    assert_eq!(
        vec!["Alpha".to_string(), "Beta".to_string(), "Gamma".to_string()],
        res
    );
}

#[test]
fn test_with_str() {
    let res = Tester::Beta { x: 1, y: 2 }.__str__();
    assert_eq!("Tester.Beta(x=1, y=2)".to_string(), res);
}

#[test]
fn test_with_repr() {
    let res = Tester::Gamma { x: 1, y: 2, z: 3 }.__repr__();
    assert_eq!("Tester.Gamma(x=1, y=2, z=3)".to_string(), res);
}
