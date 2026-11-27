#include "DataBase.h"
#include "Parser.h"
#include "algoritm.hpp"
#include <iostream>

int main(int argc, char *argv[]) {
  if (argc < 2) {
    std::cerr << "Usage: " << argv[0] << " <input.json> [output.json]\n";
    return 1;
  }

  try {
    tetris::DataBase db;
    tetris::Parser::parseFile(argv[1], db);
    tetris::plasment(db);

    // Считаем общее количество попарных связей в графе
    size_t total_edges = 0;
    for (const auto &cell : db.cells) {
      total_edges += cell.neighbors.size();
    }
    total_edges /= 2; // Каждая связь учтена дважды (у обеих ячеек)

    std::cout << "[TetrIC] Successfully parsed JSON!\n"
              << "  Chip size:   " << db.design.chip_width << " x "
              << db.design.chip_height << "\n"
              << "  Rows count:  " << db.rows.size() << "\n"
              << "  Total cells: " << db.cells.size()
              << " (StdCells + Blockages + Pads)\n"
              << "  Total edges: " << total_edges
              << " (after net elimination)\n";
    //<< "  Strategy:    " << db.params.ordering_strategy << "\n";

    if (argc >= 3) {
      tetris::Parser::writeFile(argv[2], db);
      std::cout << "[TetrIC] Saved output JSON to: " << argv[2] << "\n";
      std::cout << "hui";
    }
  } catch (const std::exception &e) {
    std::cerr << "Error: " << e.what() << "\n";
    return 1;
  }

  return 0;
}
