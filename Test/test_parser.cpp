/**
 * @file test_parser.cpp
 * @brief Модульные тесты парсера: загрузка файлов из папки Data и негативные
 * сценарии.
 * @author Dmitry
 */

#include "DataBase.h"
#include "Parser.h"

#include <fstream>
#include <gtest/gtest.h>
#include <sstream>
#include <string>

#ifndef TEST_DATA_DIR
#define TEST_DATA_DIR "Data"
#endif

namespace tetris::testing {

// Позитивные тесты парсинга файлов

TEST(ParserFileTest, ParsesExampleFileFromDisk) {
  const std::string filepath = std::string(TEST_DATA_DIR) + "/example_1.json";

  std::ifstream check_file(filepath);
  if (!check_file.is_open()) {
    GTEST_SKIP() << "Файл " << filepath
                 << " не найден в папке Data. Пропускаем тест диска.";
  }
  check_file.close();

  DataBase db;
  ASSERT_NO_THROW(Parser::parseFile(filepath, db));

  EXPECT_GT(db.design.chip_width, 0);
  EXPECT_GT(db.design.chip_height, 0);
  EXPECT_GT(db.design.row_height, 0);
  EXPECT_FALSE(db.cells.empty());
  EXPECT_FALSE(db.rows.empty());
}

TEST(ParserFileTest, ParsesReferenceStringStream) {
  const std::string reference_json = R"({
      "design": {
        "chip_width": 2000, "chip_height": 1000, "row_height": 40,
        "row_spacing": 0, "grid_size": 5,
        "left_boundary": 0, "right_boundary": 2000,
        "top_boundary": 1000, "bottom_boundary": 0
      },
      "rows": [
        {"id": "R1", "y": 0,  "height": 40, "width": 2000, "available": true},
        {"id": "R2", "y": 40, "height": 40, "width": 2000, "available": true}
      ],
      "blockages": [
        {"id": "BLK1", "x": 800, "y": 0, "width": 200, "height": 40}
      ],
      "cells": [
        {"id": "C1", "width": 30, "height": 40, "type": "NAND2", "priority": 3},
        {"id": "C2", "width": 20, "height": 40, "type": "INV",   "priority": 1}
      ],
      "nets": [
        {"net_id": "N1", "cells": ["C1", "C2"], "weight": 1.5}
      ],
      "pad_locations": [
        {"id": "PAD_LEFT", "x": 0, "y": 500}
      ],
      "tetris_params": {
        "ordering_strategy": "connectivity",
        "cost_function": "skyline_height",
        "site_width": 5,
        "rotation_allowed": false,
        "allow_multistory": false,
        "gap_penalty": 0.1,
        "boundary_penalty": 0.05,
        "seed": 42
      }
    })";

  std::istringstream stream(reference_json);
  DataBase db;
  ASSERT_NO_THROW(Parser::parseStream(stream, db));

  EXPECT_EQ(db.design.chip_width, 2000);
  EXPECT_EQ(db.design.chip_height, 1000);
  EXPECT_EQ(db.design.row_height, 40);
  EXPECT_EQ(db.params.ordering_strategy, "connectivity");

  // Блокировка (1) + пад (1) + ячейки (2) = 4
  ASSERT_EQ(db.cells.size(), 4u);

  // Фиксированные элементы имеют меньшие численные ID
  EXPECT_TRUE(db.cells[0].isBlockage());
  EXPECT_TRUE(db.cells[0].isFixed());
  EXPECT_TRUE(db.cells[1].isIoPad());
  EXPECT_TRUE(db.cells[1].isFixed());

  // Перемещаемые ячейки не размещены
  EXPECT_TRUE(db.cells[2].isStdCell());
  EXPECT_FALSE(db.cells[2].isPlaced());
  EXPECT_TRUE(db.cells[3].isStdCell());
  EXPECT_FALSE(db.cells[3].isPlaced());

  // Вес цепи: 1.5 * 10 = 15
  const int32_t c1_id = db.findCellId("C1");
  const int32_t c2_id = db.findCellId("C2");
  ASSERT_GE(c1_id, 0);
  ASSERT_GE(c2_id, 0);

  bool link_found = false;
  for (const auto &[neighbor_id, weight] :
       db.cells[static_cast<size_t>(c1_id)].neighbors) {
    if (neighbor_id == c2_id) {
      EXPECT_EQ(weight, 15);
      link_found = true;
      break;
    }
  }
  EXPECT_TRUE(link_found);
}

// Негативные и граничные тесты

TEST(ParserNegativeTest, ThrowsOnEmptyStream) {
  DataBase db;
  std::istringstream stream("");
  EXPECT_THROW(Parser::parseStream(stream, db), std::runtime_error);
}

TEST(ParserNegativeTest, ThrowsOnMalformedJson) {
  DataBase db;
  std::istringstream stream(R"({ "design": { "chip_width": 1000 )");
  EXPECT_THROW(Parser::parseStream(stream, db), std::runtime_error);
}

TEST(ParserNegativeTest, ThrowsOnMissingDesignSection) {
  DataBase db;
  std::istringstream stream(
      R"({"cells": [{"id": "C1", "width": 10, "height": 10}]})");
  EXPECT_THROW(Parser::parseStream(stream, db), std::runtime_error);
}

TEST(ParserNegativeTest, ThrowsOnEmptyCellsArray) {
  DataBase db;
  std::istringstream stream(R"({
      "design": {"chip_width": 500, "chip_height": 500, "row_height": 40},
      "cells": []
    })");
  EXPECT_THROW(Parser::parseStream(stream, db), std::runtime_error);
}

TEST(ParserNegativeTest, ThrowsOnNonPositiveChipDimensions) {
  DataBase db;
  std::istringstream stream(R"({
      "design": {"chip_width": -500, "chip_height": 500, "row_height": 40},
      "cells": [{"id": "C1", "width": 10, "height": 40}]
    })");
  EXPECT_THROW(Parser::parseStream(stream, db), std::runtime_error);
}

TEST(ParserNegativeTest, ThrowsOnNonPositiveCellDimensions) {
  DataBase db;
  std::istringstream stream(R"({
      "design": {"chip_width": 500, "chip_height": 500, "row_height": 40},
      "cells": [{"id": "C1", "width": 0, "height": 40}]
    })");
  EXPECT_THROW(Parser::parseStream(stream, db), std::runtime_error);
}

TEST(ParserNegativeTest, ThrowsOnBlockageOutOfBounds) {
  DataBase db;
  std::istringstream stream(R"({
      "design": {"chip_width": 500, "chip_height": 500, "row_height": 40},
      "blockages": [
        {"id": "BLK1", "x": 450, "y": 0, "width": 100, "height": 40}
      ],
      "cells": [{"id": "C1", "width": 10, "height": 40}]
    })");
  // 450 + 100 = 550 > chip_width (500)
  EXPECT_THROW(Parser::parseStream(stream, db), std::runtime_error);
}

TEST(ParserNegativeTest, ThrowsOnPadOutOfBounds) {
  DataBase db;
  std::istringstream stream(R"({
      "design": {"chip_width": 500, "chip_height": 500, "row_height": 40},
      "pad_locations": [
        {"id": "PAD1", "x": 550, "y": 100, "width": 10, "height": 10}
      ],
      "cells": [{"id": "C1", "width": 10, "height": 40}]
    })");
  EXPECT_THROW(Parser::parseStream(stream, db), std::runtime_error);
}

TEST(ParserNegativeTest, ThrowsOnDuplicateCellIds) {
  DataBase db;
  std::istringstream stream(R"({
      "design": {"chip_width": 500, "chip_height": 500, "row_height": 40},
      "cells": [
        {"id": "C1", "width": 20, "height": 40},
        {"id": "C1", "width": 30, "height": 40}
      ]
    })");
  EXPECT_THROW(Parser::parseStream(stream, db), std::runtime_error);
}

TEST(ParserNegativeTest, ThrowsOnNetReferencingUnknownCell) {
  DataBase db;
  std::istringstream stream(R"({
      "design": {"chip_width": 500, "chip_height": 500, "row_height": 40},
      "cells": [{"id": "C1", "width": 20, "height": 40}],
      "nets": [
        {"net_id": "N1", "cells": ["C1", "MISSING_CELL"], "weight": 1.0}
      ]
    })");
  EXPECT_THROW(Parser::parseStream(stream, db), std::runtime_error);
}

TEST(ParserNegativeTest, ThrowsOnInt16Overflow) {
  DataBase db;
  std::istringstream stream(R"({
      "design": {"chip_width": 100000, "chip_height": 100000, "row_height": 40},
      "cells": [{"id": "C1", "width": 40000, "height": 40}]
    })");
  EXPECT_THROW(Parser::parseStream(stream, db), std::runtime_error);
}

} // namespace tetris::testing