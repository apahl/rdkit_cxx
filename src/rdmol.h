#include <string>
#include "GraphMol/GraphMol.h"
#include "GraphMol/SmilesParse/SmilesParse.h"
#include "GraphMol/SmilesParse/SmilesWrite.h"

namespace RDKit
{

    std::shared_ptr<ROMol> mol_from_smiles(const std::string &smiles);

    uint num_atoms(std::shared_ptr<ROMol> mol);

    std::unique_ptr<std::string> to_smiles(std::shared_ptr<ROMol> mol);

    std::unique_ptr<std::string> to_svg(std::shared_ptr<ROMol> mol);

} // namespace RDKit