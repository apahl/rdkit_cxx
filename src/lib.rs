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

    }
}

pub mod molecule {
    use super::ffi;
    use cxx::{let_cxx_string, SharedPtr};
    pub struct Mol {
        ptr: SharedPtr<ffi::ROMol>,
    }

    impl Mol {
        pub fn from_smiles(smi: &str) -> Option<Self> {
            let_cxx_string!(s = smi.to_owned());
            let ptr = ffi::mol_from_smiles(&s);
            if ptr.is_null() {
                None
            } else {
                Some(Self { ptr })
            }
        }

        pub fn to_smiles(&self) -> String {
            let mol_ptr = self.ptr.clone();
            let smi_ptr = ffi::to_smiles(mol_ptr);
            (*smi_ptr).to_string_lossy().into_owned()
        }

        pub fn num_atoms(&self) -> u32 {
            let mol_ptr = self.ptr.clone();
            ffi::num_atoms(mol_ptr)
        }

        pub fn to_svg(&self) -> String {
            let mol_ptr = self.ptr.clone();
            let svg_ptr = ffi::to_svg(mol_ptr);
            (*svg_ptr).to_string_lossy().into_owned()
        }
    }
}
