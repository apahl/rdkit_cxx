#include <string>
#include "GraphMol/GraphMol.h"
#include "GraphMol/SmilesParse/SmilesParse.h"
#include "GraphMol/SmilesParse/SmilesWrite.h"
#include "GraphMol/Descriptors/MolDescriptors.h"
#include "GraphMol/Descriptors/MolSurf.h"

namespace RDKit
{

    std::shared_ptr<ROMol> mol_from_smiles(const std::string &smiles);

    uint num_atoms(std::shared_ptr<ROMol> mol);

    std::unique_ptr<std::string> to_smiles(std::shared_ptr<ROMol> mol);

    std::unique_ptr<std::string> to_svg(std::shared_ptr<ROMol> mol);

    // Functions from module Descriptors
    double avg_mw(std::shared_ptr<ROMol> mol);

    uint num_hbd(std::shared_ptr<ROMol> mol);
    uint num_hba(std::shared_ptr<ROMol> mol);

    uint num_hbd_lip(std::shared_ptr<ROMol> mol);
    uint num_hba_lip(std::shared_ptr<ROMol> mol);

    uint num_rotb(std::shared_ptr<ROMol> mol);
    uint num_het_atoms(std::shared_ptr<ROMol> mol);

    double fraction_csp3(std::shared_ptr<ROMol> mol);
    double clogp(std::shared_ptr<ROMol> mol);

    double tpsa(std::shared_ptr<ROMol> mol);


} // namespace RDKit