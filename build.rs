use cxx_build::CFG;

fn main() {
    CFG.include_prefix = "";
    let rdkit_conda = std::env::var("RDKIT_CONDA")
    .expect("RDKIT_CONDA environment variable not set. \nPlease set to the location of your conda RDKit installation, e.g. `/home/user/anaconda3/envs/chem`");
    // let include_path_1 = format!("{}/include", rdkit_conda);
    // let include_path_2 = format!("{}/include/rdkit", rdkit_conda);
    let lib_path = format!("{rdkit_conda}/lib");

    cxx_build::bridge("src/lib.rs") // returns a cc::Build
        .file("src/rdmol.cc")
        .include(format!("{rdkit_conda}/include/rdkit"))
        .include(format!("{rdkit_conda}/include"))
        .flag_if_supported("-std=c++14")
        .compile("rdkit");

    println!("cargo:rustc-link-search=native={lib_path}");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=RDKitGraphMol");
    println!("cargo:rustc-link-lib=dylib=RDKitSmilesParse");
    println!("cargo:rustc-link-lib=dylib=RDKitDescriptors");
    println!("cargo:rustc-link-lib=dylib=RDKitSubstructMatch");
    println!("cargo:rustc-link-lib=dylib=RDKitChemTransforms");

    println!("cargo:rustc-link-lib=dylib=RDKitMolDraw2D");
    println!("cargo:rustc-link-lib=dylib=RDKitcoordgen");
    println!("cargo:rustc-link-lib=dylib=RDKitDepictor");
}
