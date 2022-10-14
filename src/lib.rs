// https://rdkit.org/docs/GettingStartedInC++.html
// https://github1s.com/Xiphoseer/dpi-sys

#[cxx::bridge(namespace = "RDKit")]
mod ffi {
    unsafe extern "C++" {
        include!("src/rdmol.h");

        type ROMol;
        type RWMol;

        fn mol_from_smiles(smi: &CxxString) -> SharedPtr<ROMol>;

        fn to_smiles(mol: SharedPtr<ROMol>) -> UniquePtr<CxxString>;

        fn num_atoms(mol: SharedPtr<ROMol>) -> u32;

        fn to_svg(mol: SharedPtr<ROMol>) -> UniquePtr<CxxString>;

        // Descriptors
        fn avg_mw(mol: SharedPtr<ROMol>) -> f64;
        fn num_hbd(mol: SharedPtr<ROMol>) -> u32;
        fn num_hba(mol: SharedPtr<ROMol>) -> u32;
        fn num_hbd_lip(mol: SharedPtr<ROMol>) -> u32;
        fn num_hba_lip(mol: SharedPtr<ROMol>) -> u32;
        fn num_rotb(mol: SharedPtr<ROMol>) -> u32;
        fn num_het_atoms(mol: SharedPtr<ROMol>) -> u32;
        fn fraction_csp3(mol: SharedPtr<ROMol>) -> f64;
        fn clogp(mol: SharedPtr<ROMol>) -> f64;
        fn tpsa(mol: SharedPtr<ROMol>) -> f64;

    }
}

pub mod molecule {
    use super::ffi;
    use cxx::{let_cxx_string, SharedPtr};
    pub struct Mol {
        ptr: SharedPtr<ffi::ROMol>,
    }

    impl Mol {
        /// Generates a molecule from a SMILES string
        pub fn from_smiles(smi: &str) -> Option<Self> {
            let_cxx_string!(s = smi.to_owned());
            let ptr = ffi::mol_from_smiles(&s);
            if ptr.is_null() {
                None
            } else {
                Some(Self { ptr })
            }
        }

        /// Returns the molecule as a SMILES string
        pub fn to_smiles(&self) -> String {
            let mol_ptr = self.ptr.clone();
            let smi_ptr = ffi::to_smiles(mol_ptr);
            (*smi_ptr).to_string_lossy().into_owned()
        }

        /// Number of heavy (non-hydrogen) atoms
        pub fn num_atoms(&self) -> u32 {
            let mol_ptr = self.ptr.clone();
            ffi::num_atoms(mol_ptr)
        }

        /// Returns the molecule image as SVG string
        pub fn to_svg(&self) -> String {
            let mol_ptr = self.ptr.clone();
            let svg_ptr = ffi::to_svg(mol_ptr);
            (*svg_ptr).to_string_lossy().into_owned()
        }
        //Descriptors
        /// Average molecular weight
        pub fn avg_mw(&self) -> f64 {
            let mol_ptr = self.ptr.clone();
            ffi::avg_mw(mol_ptr)
        }
        /// Number of hydrogen donors
        pub fn num_hbd(&self) -> u32 {
            let mol_ptr = self.ptr.clone();
            ffi::num_hbd(mol_ptr)
        }
        /// Number of hydrogen acceptors
        pub fn num_hba(&self) -> u32 {
            let mol_ptr = self.ptr.clone();
            ffi::num_hba(mol_ptr)
        }
        /// Number of hydrogen donors (Lipinski definition)
        pub fn num_hbd_lip(&self) -> u32 {
            let mol_ptr = self.ptr.clone();
            ffi::num_hbd_lip(mol_ptr)
        }
        /// Number of hydrogen acceptors (Lipinski definition)
        pub fn num_hba_lip(&self) -> u32 {
            let mol_ptr = self.ptr.clone();
            ffi::num_hba_lip(mol_ptr)
        }
        /// Number of rotatable bonds
        pub fn num_rotb(&self) -> u32 {
            let mol_ptr = self.ptr.clone();
            ffi::num_rotb(mol_ptr)
        }
        /// Number of hetero atoms
        pub fn num_het_atoms(&self) -> u32 {
            let mol_ptr = self.ptr.clone();
            ffi::num_het_atoms(mol_ptr)
        }
        /// Fraction of sp3 carbons
        pub fn fraction_csp3(&self) -> f64 {
            let mol_ptr = self.ptr.clone();
            ffi::fraction_csp3(mol_ptr)
        }
        /// Crippen's logP
        pub fn clogp(&self) -> f64 {
            let mol_ptr = self.ptr.clone();
            ffi::clogp(mol_ptr)
        }
        /// topological polar surface area
        pub fn tpsa(&self) -> f64 {
            let mol_ptr = self.ptr.clone();
            ffi::tpsa(mol_ptr)
        }
    }
}
