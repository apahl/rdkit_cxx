#include <string>
#include "GraphMol/GraphMol.h"
#include "GraphMol/SmilesParse/SmilesParse.h"
#include "GraphMol/SmilesParse/SmilesWrite.h"
#include "GraphMol/Descriptors/MolDescriptors.h"
#include "GraphMol/Descriptors/MolSurf.h"

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

    // Functions from module Descriptors
    double avg_mw(std::shared_ptr<ROMol> mol) {
        return Descriptors::calcAMW(*mol);
    };

    uint num_hbd(std::shared_ptr<ROMol> mol) {
        return Descriptors::calcNumHBD(*mol);
    };
    uint num_hba(std::shared_ptr<ROMol> mol) {
        return Descriptors::calcNumHBA(*mol);
    };

    uint num_hbd_lip(std::shared_ptr<ROMol> mol) {
        return Descriptors::calcLipinskiHBD(*mol);
    };
    uint num_hba_lip(std::shared_ptr<ROMol> mol) {
        return Descriptors::calcLipinskiHBA(*mol);
    };

    uint num_rotb(std::shared_ptr<ROMol> mol) {
        return Descriptors::calcNumRotatableBonds(*mol);
    };

    uint num_het_atoms(std::shared_ptr<ROMol> mol) {
        return Descriptors::calcNumHeteroatoms(*mol);
    };

    double fraction_csp3(std::shared_ptr<ROMol> mol) {
        return Descriptors::calcFractionCSP3(*mol);
    };
    double clogp(std::shared_ptr<ROMol> mol) {
        return Descriptors::calcClogP(*mol);
    };

    double tpsa(std::shared_ptr<ROMol> mol) {
        return Descriptors::calcTPSA(*mol);
    };



} // namespace RDKit