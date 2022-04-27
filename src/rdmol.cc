#include <string>
#include "GraphMol/GraphMol.h"
#include "GraphMol/SmilesParse/SmilesParse.h"
#include "GraphMol/SmilesParse/SmilesWrite.h"

#include "GraphMol/Depictor/RDDepictor.h"
#include "GraphMol/MolDraw2D/MolDraw2DSVG.h"

namespace RDKit
{

    std::shared_ptr<ROMol> mol_from_smiles(const std::string &smiles)
    {
        // SmilesParserParams params;
        std::shared_ptr<ROMol> mol(SmilesToMol(smiles)); // , &params);
        return mol;
    }

    uint num_atoms(std::shared_ptr<ROMol> mol)
    {
        return mol->getNumAtoms();
    }

    std::unique_ptr<std::string> to_smiles(std::shared_ptr<ROMol> mol)
    {
        return std::make_unique<std::string>(MolToSmiles(*mol));
    }

    std::unique_ptr<std::string> to_svg(std::shared_ptr<ROMol> mol)
    {
        RDDepict::preferCoordGen = true;
        RDDepict::compute2DCoords(*mol);
        RDKit::MolDraw2DSVG svg_drawer(300, 300);
        svg_drawer.drawMolecule(*mol);
        svg_drawer.finishDrawing();
        return std::make_unique<std::string>(svg_drawer.getDrawingText());
    }

} // namespace RDKit