// ES: Sonda de instalación; no valida el núcleo ni la semántica de SV.
// EN: Installation probe; it does not validate the SV core or semantics.
fn total(values: &[u32]) -> u32 {
    values.iter().sum()
}

fn main() {
    println!("SV_TOOLCHAIN_OK:{}", total(&[40, 35, 25]));
}

#[test]
fn sums_values() {
    assert_eq!(total(&[40, 35, 25]), 100);
    assert_eq!(total(&[]), 0);
}
