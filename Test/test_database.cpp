/**
 * @file test_database.cpp
 * @brief Модульные тесты базы данных: интервалы, оконные запросы, CompactVec и
 * перенумерация.
 * @author Dmitry
 */

#include "DataBase.h"
#include "Parser.h"

#include <gtest/gtest.h>
#include <sstream>

namespace tetris::testing {

TEST(DatabaseTest, CutsBlockagesFromRowIntervalsAndBinarySearches) {
  const std::string json_str = R"({
      "design": {"chip_width": 1000, "chip_height": 200, "row_height": 50, "grid_size": 5},
      "rows": [
        {"id": "R1", "y": 0,   "height": 50, "width": 1000, "available": true},
        {"id": "R2", "y": 50,  "height": 50, "width": 1000, "available": true},
        {"id": "R3", "y": 100, "height": 50, "width": 1000, "available": false}
      ],
      "blockages": [
        {"id": "BLK1", "x": 300, "y": 0, "width": 200, "height": 50}
      ],
      "cells": [
        {"id": "C1", "width": 50, "height": 50}
      ]
    })";

  std::istringstream stream(json_str);
  DataBase db;
  Parser::parseStream(stream, db);

  // В строке R1 интервал [300, 500] вырезан: остаются [0, 300) и [500, 1000)
  ASSERT_EQ(db.rows[0].free_intervals.size(), 2u);
  EXPECT_EQ(db.rows[0].free_intervals[0].x_start, 0);
  EXPECT_EQ(db.rows[0].free_intervals[0].x_end, 300);
  EXPECT_EQ(db.rows[0].free_intervals[1].x_start, 500);
  EXPECT_EQ(db.rows[0].free_intervals[1].x_end, 1000);

  // Оконный запрос canPlaceAt через binary search
  EXPECT_TRUE(db.rows[0].canPlaceAt(0, 50));
  EXPECT_TRUE(db.rows[0].canPlaceAt(250, 50));
  EXPECT_FALSE(
      db.rows[0].canPlaceAt(260, 50)); // Вылезает на 10 единиц в зону BLK1
  EXPECT_FALSE(db.rows[0].canPlaceAt(350, 50)); // Полностью внутри BLK1
  EXPECT_TRUE(db.rows[0].canPlaceAt(500, 100)); // Начиная от правого края BLK1

  // R2 не задета блокировкой
  ASSERT_EQ(db.rows[1].free_intervals.size(), 1u);
  EXPECT_TRUE(db.rows[1].canPlaceAt(350, 50));

  // R3 помечена available = false
  EXPECT_TRUE(db.rows[2].free_intervals.empty());
  EXPECT_FALSE(db.rows[2].canPlaceAt(0, 50));
}

TEST(DatabaseTest, GeneratesRowsAutomaticallyWhenOmitted) {
  const std::string json_str = R"({
      "design": {
        "chip_width": 500, "chip_height": 200, "row_height": 40, "row_spacing": 10,
        "bottom_boundary": 0, "top_boundary": 200
      },
      "cells": [{"id": "C1", "width": 20, "height": 40}]
    })";

  std::istringstream stream(json_str);
  DataBase db;
  Parser::parseStream(stream, db);

  // Шаг: 40 + 10 = 50. Ряды на y: 0, 50, 100, 150 (итого 4)
  ASSERT_EQ(db.rows.size(), 4u);
  EXPECT_EQ(db.rows[0].y, 0);
  EXPECT_EQ(db.rows[1].y, 50);
  EXPECT_EQ(db.rows[2].y, 100);
  EXPECT_EQ(db.rows[3].y, 150);
}

TEST(DatabaseTest, ReorderCellsUpdatesDictionariesAndNeighbors) {
  const std::string json_str = R"({
      "design": {"chip_width": 500, "chip_height": 200, "row_height": 40},
      "cells": [
        {"id": "C_SMALL", "width": 10, "height": 40, "priority": 1},
        {"id": "C_MID",   "width": 30, "height": 40, "priority": 2},
        {"id": "C_LARGE", "width": 60, "height": 40, "priority": 3}
      ],
      "nets": [
        {"net_id": "N1", "cells": ["C_SMALL", "C_MID", "C_LARGE"], "weight": 2.0}
      ]
    })";

  std::istringstream stream(json_str);
  DataBase db;
  Parser::parseStream(stream, db);

  // Исходный порядок: 0: C_SMALL, 1: C_MID, 2: C_LARGE
  // Алгоритмическая перестановка: [2, 1, 0] (C_LARGE, C_MID, C_SMALL)
  db.reorderCells({2, 1, 0});

  EXPECT_EQ(db.findCellId("C_LARGE"), 0);
  EXPECT_EQ(db.findCellId("C_MID"), 1);
  EXPECT_EQ(db.findCellId("C_SMALL"), 2);

  EXPECT_EQ(db.id_to_name[0], "C_LARGE");
  EXPECT_EQ(db.id_to_name[1], "C_MID");
  EXPECT_EQ(db.id_to_name[2], "C_SMALL");

  // Для C_SMALL (ID=2) список соседей должен быть {0, 1}, отсортированный по
  // возрастанию ID
  const auto &small_links = db.cells[2].neighbors;
  ASSERT_EQ(small_links.size(), 2u);
  EXPECT_EQ(small_links[0].first, 0);
  EXPECT_EQ(small_links[1].first, 1);
}

TEST(DatabaseTest, CompactVecCopyAndMoveOperations) {
  CompactVec<NeighborLink> original;
  for (int32_t i = 0; i < 5; ++i) {
    original.push_back({i, i * 10});
  }
  EXPECT_EQ(original.size(), 5u);

  // Копирование
  CompactVec<NeighborLink> copy(original);
  ASSERT_EQ(copy.size(), 5u);
  EXPECT_EQ(copy[3].first, 3);
  EXPECT_EQ(copy[3].second, 30);

  // Перемещение
  CompactVec<NeighborLink> moved(std::move(original));
  EXPECT_EQ(moved.size(), 5u);
  EXPECT_TRUE(original.empty());
  EXPECT_EQ(moved[4].first, 4);
}

TEST(DatabaseTest, WriteStreamAndReparseRoundTrip) {
  const std::string json_str = R"({
      "design": {"chip_width": 500, "chip_height": 200, "row_height": 40},
      "cells": [
        {"id": "C1", "width": 30, "height": 40, "type": "NAND2"},
        {"id": "C2", "width": 20, "height": 40, "type": "INV"}
      ]
    })";

  std::istringstream in_stream(json_str);
  DataBase db1;
  Parser::parseStream(in_stream, db1);

  db1.cells[0].setPosition(100, 40);
  db1.cells[1].setPosition(140, 40);

  std::ostringstream out_stream;
  Parser::writeStream(out_stream, db1);

  std::istringstream re_stream(out_stream.str());
  DataBase db2;
  ASSERT_NO_THROW(Parser::parseStream(re_stream, db2));

  ASSERT_EQ(db2.cells.size(), 2u);
  EXPECT_EQ(db2.cells[0].x, 100);
  EXPECT_EQ(db2.cells[0].y, 40);
  EXPECT_TRUE(db2.cells[0].isPlaced());
  EXPECT_EQ(db2.cells[1].x, 140);
  EXPECT_EQ(db2.cells[1].y, 40);
  EXPECT_TRUE(db2.cells[1].isPlaced());
}

} // namespace tetris::testing