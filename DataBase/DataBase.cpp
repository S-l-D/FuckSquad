/**
 * @file DataBase.cpp
 * @brief Реализация базы данных, словарей, перенумерации связей и свободных
 * интервалов.
 * @author Dmitry
 */

#include "DataBase.h"

#include <algorithm>
#include <stdexcept>

namespace tetris {

// Методы RowDef (свободные интервалы и оконный запрос делением пополам)

void RowDef::initIntervals(int32_t left_bound, int32_t right_bound) {
  free_intervals.clear();
  if (!available) {
    return;
  }
  const int32_t start_x = std::max(0, left_bound);
  const int32_t end_x = std::min(width, right_bound);
  if (start_x < end_x) {
    free_intervals.push_back({start_x, end_x});
  }
}

void RowDef::occupyInterval(int32_t occ_x1, int32_t occ_x2) {
  if (occ_x1 >= occ_x2 || free_intervals.empty()) {
    return;
  }

  std::vector<FreeInterval> updated;
  updated.reserve(free_intervals.size() + 1);

  for (const auto &iv : free_intervals) {
    if (occ_x2 <= iv.x_start || occ_x1 >= iv.x_end) {
      updated.push_back(iv);
      continue;
    }
    if (occ_x1 > iv.x_start) {
      updated.push_back({iv.x_start, occ_x1});
    }
    if (occ_x2 < iv.x_end) {
      updated.push_back({occ_x2, iv.x_end});
    }
  }

  free_intervals = std::move(updated);
}

bool RowDef::canPlaceAt(int32_t x, int32_t cell_w) const noexcept {
  if (!available || free_intervals.empty() || cell_w <= 0) {
    return false;
  }
  const int32_t target_end = x + cell_w;

  // Поиск делением пополам O(log K)
  const FreeInterval key{x, x};
  auto it = std::upper_bound(free_intervals.begin(), free_intervals.end(), key);
  if (it == free_intervals.begin()) {
    return false;
  }
  --it;
  return (x >= it->x_start && target_end <= it->x_end);
}

// Методы DataBase

void DataBase::clear() {
  design = DesignParams{};
  params = TetrisParams{};
  cells.clear();
  rows.clear();
  id_to_name.clear();
  name_to_id.clear();
  type_id_to_name.clear();
  type_name_to_id.clear();
}

int16_t DataBase::registerCellType(const std::string &type_name) {
  auto it = type_name_to_id.find(type_name);
  if (it != type_name_to_id.end()) {
    return it->second;
  }
  if (type_id_to_name.size() > static_cast<size_t>(TypeFlags::DICT_ID_MASK)) {
    throw std::runtime_error(
        "DataBase error: Too many cell types (limit is 1024).");
  }
  const int16_t new_id = static_cast<int16_t>(type_id_to_name.size());
  type_id_to_name.push_back(type_name);
  type_name_to_id.emplace(type_name, new_id);
  return new_id;
}

int32_t DataBase::addCell(const std::string &text_id, Cell cell) {
  if (text_id.empty()) {
    throw std::runtime_error("DataBase error: Cell ID cannot be empty.");
  }
  if (name_to_id.find(text_id) != name_to_id.end()) {
    throw std::runtime_error("DataBase error: Duplicate object ID '" + text_id +
                             "'");
  }

  const int32_t new_id = static_cast<int32_t>(cells.size());
  name_to_id.emplace(text_id, new_id);
  id_to_name.push_back(text_id);
  cells.push_back(std::move(cell));
  return new_id;
}

int32_t DataBase::findCellId(const std::string &text_id) const {
  const auto it = name_to_id.find(text_id);
  if (it == name_to_id.end()) {
    return -1;
  }
  return it->second;
}

void DataBase::reorderCells(const std::vector<int32_t> &new_to_old_order) {
  const size_t n = cells.size();
  if (new_to_old_order.size() != n) {
    throw std::runtime_error(
        "DataBase error: Permutation size mismatch in reorderCells.");
  }

  std::vector<Cell> new_cells;
  new_cells.reserve(n);
  std::vector<std::string> new_id_to_name;
  new_id_to_name.reserve(n);
  std::vector<int32_t> old_to_new(n, -1);

  for (size_t new_id = 0; new_id < n; ++new_id) {
    const int32_t old_id = new_to_old_order[new_id];
    if (old_id < 0 || static_cast<size_t>(old_id) >= n ||
        old_to_new[static_cast<size_t>(old_id)] != -1) {
      throw std::runtime_error(
          "DataBase error: Invalid permutation index in reorderCells.");
    }
    old_to_new[static_cast<size_t>(old_id)] = static_cast<int32_t>(new_id);
    new_cells.push_back(std::move(cells[static_cast<size_t>(old_id)]));
    new_id_to_name.push_back(
        std::move(id_to_name[static_cast<size_t>(old_id)]));
  }

  cells = std::move(new_cells);
  id_to_name = std::move(new_id_to_name);

  // Обновляем словарь текстовое имя -> новый численный ID
  name_to_id.clear();
  for (size_t new_id = 0; new_id < n; ++new_id) {
    name_to_id.emplace(id_to_name[new_id], static_cast<int32_t>(new_id));
  }

  // Обновляем численные ID соседей в каждой ячейке и сортируем их по
  // возрастанию ID
  for (auto &cell : cells) {
    for (auto &link : cell.neighbors) {
      link.first = old_to_new[static_cast<size_t>(link.first)];
    }
    std::sort(cell.neighbors.begin(), cell.neighbors.end(),
              [](const NeighborLink &a, const NeighborLink &b) {
                return a.first < b.first;
              });
  }
}

void DataBase::initRowsAndBlockages() {
  if (rows.empty() && design.row_height > 0) {
    const int32_t step = design.row_height + design.row_spacing;
    int32_t row_idx = 1;
    for (int32_t y = design.bottom_boundary;
         y + design.row_height <= design.top_boundary; y += step) {
      RowDef r;
      r.id = "R" + std::to_string(row_idx++);
      r.y = y;
      r.height = design.row_height;
      r.width = design.chip_width;
      r.available = true;
      rows.push_back(std::move(r));
    }
  }

  for (auto &row : rows) {
    row.initIntervals(design.left_boundary, design.right_boundary);
  }

  for (const auto &cell : cells) {
    if (!cell.isBlockage()) {
      continue;
    }
    const int32_t blk_x1 = cell.x;
    const int32_t blk_y1 = cell.y;
    const int32_t blk_x2 = cell.x + cell.effectiveWidth();
    const int32_t blk_y2 = cell.y + cell.effectiveHeight();

    for (auto &row : rows) {
      const int32_t row_y1 = row.y;
      const int32_t row_y2 = row.y + row.height;
      if (blk_y1 < row_y2 && blk_y2 > row_y1) {
        row.occupyInterval(blk_x1, blk_x2);
      }
    }
  }
}

} // namespace tetris