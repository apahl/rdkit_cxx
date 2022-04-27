use rdkit::molecule::Mol;

#[test]
fn mol_to_smiles() {
    let smi = "c1ccccc1C(=O)NC";
    let m = Mol::from_smiles(smi).unwrap();
    // println!("{}", m.to_smiles());
    assert_eq!(m.num_atoms(), 10);
    assert_eq!(m.to_smiles(), "CNC(=O)c1ccccc1");
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
