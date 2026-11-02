#pragma once

#include "DataBase.h"
#include <algorithm>
#include <climits>
#include <cstdint>
#include <vector>

namespace tetris {

inline int64_t CalculateHPWL(const Cell &cell, int32_t pos_x, int32_t pos_y,
                             const DataBase &design) {
  int64_t total_hpwl = 0;

  for (size_t i = 0; i < cell.neighbors.size(); ++i) {
    int32_t current_net = cell.neighbors[i].number_net;

    // 1. Проверяем, был ли уже рассчитан BBox для этой цепи на предыдущих
    // итерациях
    bool already_processed = false;
    for (size_t j = 0; j < i; ++j) {
      if (cell.neighbors[j].number_net == current_net) {
        already_processed = true;
        break;
      }
    }
    // Внимание: если все неты были свернуты парсером с number_net = 0,
    // то проверка по number_net пропустит остальных соседей!
    // Поэтому пропускаем только если number_net != 0:
    if (current_net != 0 && already_processed) {
      continue;
    }

    // 2. Инициализируем BBox координатами оцениваемой позиции для текущей
    // ячейки
    int32_t min_x = pos_x;
    int32_t max_x = pos_x;
    int32_t min_y = pos_y;
    int32_t max_y = pos_y;

    int32_t weight = cell.neighbors[i].weight;
    int placed_pins_count = 1;

    if (current_net == 0) {
      // Попарный режим (когда неты свернуты в ребра графа)
      const auto &neighbor_cell =
          design.cells[static_cast<size_t>(cell.neighbors[i].ID_cell)];
      if (neighbor_cell.is_placed()) {
        min_x = std::min(min_x, neighbor_cell.x);
        max_x = std::max(max_x, neighbor_cell.x);
        min_y = std::min(min_y, neighbor_cell.y);
        max_y = std::max(max_y, neighbor_cell.y);
        placed_pins_count++;
      }
    } else {
      // 3. Обходим всех соседей и расширяем BBox за счет тех, кто принадлежит
      // текущей цепи
      for (size_t k = i; k < cell.neighbors.size(); ++k) {
        if (cell.neighbors[k].number_net == current_net) {
          const auto &neighbor_cell =
              design.cells[static_cast<size_t>(cell.neighbors[k].ID_cell)];
          if (neighbor_cell.is_placed()) {
            min_x = std::min(min_x, neighbor_cell.x);
            max_x = std::max(max_x, neighbor_cell.x);
            min_y = std::min(min_y, neighbor_cell.y);
            max_y = std::max(max_y, neighbor_cell.y);
            placed_pins_count++;
          }
        }
      }
    }

    // 4. Если в цепи есть хотя бы один закрепленный контакт помимо текущей
    // ячейки, плюсуем HPWL
    if (placed_pins_count > 1) {
      int64_t wirelength =
          static_cast<int64_t>(max_x - min_x) + (max_y - min_y);
      total_hpwl += wirelength * weight;
    }
  }

  return total_hpwl;
}

inline bool placeFallbackBottomLeft(Cell &cell, std::vector<RowDef> &rows,
                                    int32_t site_width) {
  for (size_t r = 0; r < rows.size(); ++r) {
    if (!rows[r].available)
      continue;

    for (const auto &interval : rows[r].free_intervals) {
      // Выравниваем x_start вверх кратно site_width
      int32_t rem = interval.x_start % site_width;
      int32_t aligned_x = interval.x_start;
      if (rem != 0) {
        aligned_x += (site_width - rem);
      }

      if (aligned_x + cell.width <= interval.x_end) {
        cell.x = aligned_x;
        cell.y = rows[r].y;
        cell.set_placed();
        rows[r].occupyInterval(aligned_x, aligned_x + cell.width);
        return true;
      }
    }
  }
  return false;
}

inline bool plasment(DataBase &design) {
  std::vector<RowDef> &rows = design.getRows();
  std::vector<Cell> &cels = design.getCells();
  const int32_t sw = std::max<int32_t>(1, design.params.site_width);

  for (auto &cell : cels) {
    // 1. Пропускаем фиксированные элементы (Blockages, IO-Pads) и уже
    // размещенные ячейки!
    if (!cell.isStdCell() || cell.is_placed()) {
      continue;
    }

    int64_t x_c = 0, y_c = 0, p_c = 0;

    for (size_t i = 0; i < cell.neighbors.size(); ++i) {
      const auto &neighbor =
          cels[static_cast<size_t>(cell.neighbors[i].ID_cell)];
      if (neighbor.is_placed()) {
        x_c += static_cast<int64_t>(neighbor.x) * cell.neighbors[i].weight;
        y_c += static_cast<int64_t>(neighbor.y) * cell.neighbors[i].weight;
        p_c += cell.neighbors[i].weight;
      }
    }

    // Если у ячейки вообще нет соседей ИЛИ ни один из её соседей еще не
    // размещен (p_c == 0), ставим её в самую нижнюю левую доступную позицию
    // (fallback)
    if (p_c <= 0) {
      if (!placeFallbackBottomLeft(cell, rows, sw)) {
        return false;
      }
      continue;
    }

    x_c /= p_c;
    y_c /= p_c;

    // Защита от выхода за границы кристалла
    x_c = std::clamp<int64_t>(x_c, design.design.left_boundary,
                              design.design.right_boundary);
    y_c = std::clamp<int64_t>(y_c, design.design.bottom_boundary,
                              design.design.top_boundary);

    // Целевой ряд и защита от выхода за пределы массива строк
    const int32_t row_pitch =
        design.design.row_height + design.design.row_spacing;
    int32_t base_row =
        static_cast<int32_t>((y_c - design.design.bottom_boundary) / row_pitch);
    base_row =
        std::clamp<int32_t>(base_row, 0, static_cast<int32_t>(rows.size()) - 1);

    const int MAX_EVALS = 30;
    int eval_count = 0;
    int same_hpwl_count = 0;
    bool stop_search = false;

    int64_t best_hpwl = INT64_MAX;
    int32_t best_x = -1, best_y = -1;
    size_t best_row_idx = 0;

    // Движение по Манхэттенской спирали (удаление от целевой строки: 0, +1, -1,
    // +2, -2...)
    for (size_t radius = 0; radius < rows.size() && !stop_search; ++radius) {
      const int32_t rad = static_cast<int32_t>(radius);
      int32_t rows_to_check[] = {base_row + rad, base_row - rad};
      int num_rows = (radius == 0) ? 1 : 2;

      for (int r = 0; r < num_rows && !stop_search; ++r) {
        int32_t target_row_signed = rows_to_check[r];
        if (target_row_signed < 0 ||
            static_cast<size_t>(target_row_signed) >= rows.size()) {
          continue;
        }

        size_t target_row = static_cast<size_t>(target_row_signed);
        if (!rows[target_row].available)
          continue;

        auto &current_intervals = rows[target_row].free_intervals;
        if (current_intervals.empty())
          continue;

        // Поиск ближайшего свободного интервала к x_c
        auto lower = std::lower_bound(
            current_intervals.begin(), current_intervals.end(), x_c,
            [](const FreeInterval &interval, int64_t val) {
              return interval.x_start < val;
            });

        size_t center_idx =
            (lower == current_intervals.end())
                ? current_intervals.size() - 1
                : static_cast<size_t>(lower - current_intervals.begin());

        size_t start_idx = (center_idx > 0) ? center_idx - 1 : 0;
        size_t end_idx = std::min(center_idx + 1, current_intervals.size() - 1);

        for (size_t idx = start_idx; idx <= end_idx && !stop_search; ++idx) {
          const auto &interval = current_intervals[idx];

          // Сначала выравниваем левую границу интервала вверх по сетке
          // site_width
          int32_t min_valid_x = interval.x_start;
          int32_t rem = min_valid_x % sw;
          if (rem != 0) {
            min_valid_x += (sw - rem);
          }
          int32_t max_valid_x = interval.x_end - cell.width;

          // Если интервал слишком узкий для данной ячейки — пропускаем!
          if (min_valid_x > max_valid_x) {
            continue;
          }

          // Проецируем x_c в допустимые границы интервала
          int32_t pos_x = static_cast<int32_t>(
              std::clamp<int64_t>(x_c, min_valid_x, max_valid_x));

          // Выравниваем по сетке кратно site_width
          pos_x = (pos_x / sw) * sw;
          if (pos_x < min_valid_x) {
            pos_x = min_valid_x;
          }

          if (pos_x >= interval.x_start &&
              (pos_x + cell.width) <= interval.x_end) {
            int32_t pos_y = rows[target_row].y;

            int64_t current_hpwl = CalculateHPWL(cell, pos_x, pos_y, design);
            eval_count++;

            if (current_hpwl < best_hpwl) {
              best_hpwl = current_hpwl;
              best_x = pos_x;
              best_y = pos_y;
              best_row_idx = target_row;
              same_hpwl_count = 1;
            } else if (current_hpwl == best_hpwl) {
              same_hpwl_count++;
            }

            if (same_hpwl_count >= 3 || eval_count >= MAX_EVALS) {
              stop_search = true;
            }
          }
        }
      }
    }

    // Фиксация лучшей позиции на кристалле и вырезание занятого интервала в
    // строке
    if (best_x != -1) {
      cell.x = best_x;
      cell.y = best_y;
      cell.set_placed();
      rows[best_row_idx].occupyInterval(best_x, best_x + cell.width);
    } else {
      // Если в проверенных окнах места не хватило — пробуем общий fallback
      if (!placeFallbackBottomLeft(cell, rows, sw)) {
        return false;
      }
    }
  }

  return true;
}

} // namespace tetris