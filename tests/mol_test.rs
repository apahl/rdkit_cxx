use rdkit::molecule::Mol;

fn is_close(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.01
}

#[test]
fn mol_to_smiles() {
    let smi = "c1ccccc1C(=O)NC";
    let m = Mol::from_smiles(smi).unwrap();
    // println!("{}", m.to_smiles());
    assert_eq!(m.num_atoms(), 10);
    assert_eq!(m.to_smiles(), "CNC(=O)c1ccccc1");
}

#[test]
fn failing_mol() {
    let smi = "c1ccccc"; // invalid Smiles
    let m = Mol::from_smiles(smi);
    assert!(m.is_none());
}

#[test]
fn draw() {
    let smi = "c1ccccc1C(=O)NC";
    let m = Mol::from_smiles(smi).unwrap();
    let svg = m.to_svg();
    // println!("{}", m.to_svg());
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg"));
}

#[test]
fn descriptors() {
    let smi = "c1ccccc1C(=O)NC";
    let m = Mol::from_smiles(smi).unwrap();
    assert!(is_close(m.avg_mw(), 135.166));
    assert_eq!(m.num_hbd(), 1);
    assert_eq!(m.num_hba(), 1);
    assert_eq!(m.num_hbd_lip(), 1);
    assert_eq!(m.num_hba_lip(), 2);
    assert_eq!(m.num_rotb(), 1);
    assert_eq!(m.num_het_atoms(), 2);
    assert!(is_close(m.fraction_csp3(), 0.125));
    assert!(is_close(m.clogp(), 1.0462));
    assert!(is_close(m.tpsa(), 29.1));
}
