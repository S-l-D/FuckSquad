#include "DataBase.h"
#include <algorithm>

namespace tetris {

int64_t CalculateHPWL(const Cell& cell, int32_t pos_x, int32_t pos_y, const DataBase& design) {
    int64_t total_hpwl = 0;

    for (size_t i = 0; i < cell.neighbors.size(); ++i) {
        int32_t current_net = cell.neighbors[i].number_net;

        // 1. Проверяем, был ли уже рассчитан BBox для этой цепи на предыдущих итерациях
        // Это надо будет заменить на мап или вектор
        bool already_processed = false; 
        for (size_t j = 0; j < i; ++j) {
            if (cell.neighbors[j].number_net == current_net) {
                already_processed = true;
                break;
            }
        }
        if (already_processed) continue; // Цепь уже учтена, пропускаем

        // 2. Инициализируем BBox координатами оцениваемой позиции для текущей ячейки
        int32_t min_x = pos_x;
        int32_t max_x = pos_x;
        int32_t min_y = pos_y;
        int32_t max_y = pos_y;

        int32_t weight = cell.neighbors[i].weight;
        int placed_pins_count = 1; // Сама целевая ячейка считается "размещенной" в (pos_x, pos_y)

        // 3. Обходим всех соседей и расширяем BBox за счет тех, кто принадлежит текущей цепи
        for (size_t k = i; k < cell.neighbors.size(); ++k) {
            if (cell.neighbors[k].number_net == current_net) {
                const auto& neighbor_cell = design.cells[cell.neighbors[k].ID_cell];

                // Расширяем прямоугольник только для уже установленных ячеек (и падов)
                if (neighbor_cell.is_placed()) {
                    min_x = std::min(min_x, neighbor_cell.x);
                    max_x = std::max(max_x, neighbor_cell.x);
                    min_y = std::min(min_y, neighbor_cell.y);
                    max_y = std::max(max_y, neighbor_cell.y);
                    placed_pins_count++;
                }
            }
        }

        // 4. Если в цепи есть хотя бы один закрепленный контакт помимо текущей ячейки, плюсуем HPWL
        if (placed_pins_count > 1) {
            int64_t wirelength = static_cast<int64_t>(max_x - min_x) + (max_y - min_y);
            total_hpwl += wirelength * weight;
        }
    }

    return total_hpwl;
}

bool plasment(DataBase& design) {

  std::vector<RowDef> rows = design.getRows();
  std::vector<Cell> cels = design.getCells();
  RowDef activRow;


   switch (design.params.strategy) { //Это надо перенести в парсер
        case ordering_strategy::connectivity:    std::sort(cels.begin(), cels.end(), [](const Cell& a, const Cell& b) {
   return a.neighbors.size() > b.neighbors.size(); }); break;
        case ordering_strategy::priority: std::sort(cels.begin(), cels.end(), [](const Cell& a, const Cell& b) {
  return a.priority > b.priority; }); break;
        case ordering_strategy::width_desc: std::sort(cels.begin(), cels.end(), [](const Cell& a, const Cell& b) {
              return a.width > b.width; }); break;
        case ordering_strategy::width_asc: std::sort(cels.begin(), cels.end(), [](const Cell& a, const Cell& b) {
  return a.width < b.width; }); break;
        case ordering_strategy::random:
        default:
            return false;
    }

int64_t x_c = 0, y_c = 0, p_c = 0; 
std::vector<NeighborLink> plasment_nei;
plasment_nei.reserve(20);

// Использование auto& предотвращает лишнее копирование ячеек
for(auto& cell : cels) {
    x_c = 0; y_c = 0; p_c = 0;
    if(cell.neighbors.size() != 0) {
    for(int i = 0; i < cell.neighbors.size(); i++) {
        const auto& neighbor = cels[cell.neighbors[i].ID_cell];
        // Явная проверка флага размещения вместо проверки на (0,0)
        if(neighbor.is_placed()) { 
            x_c += neighbor.x * cell.neighbors[i].weight;
            y_c += neighbor.y * cell.neighbors[i].weight;
            p_c += cell.neighbors[i].weight;
        }
    }

    if (p_c > 0) {
        x_c = x_c / p_c;
        y_c = y_c / p_c;
    } else {
        // Логику для ячеек без размещенных соседей ты реализуешь отдельно
        continue; 
    }

    // Защита от выхода за границы кристалла
    x_c = std::clamp<int64_t>(x_c, design.design.left_boundary, design.design.right_boundary);
    y_c = std::clamp<int64_t>(y_c, design.design.bottom_boundary, design.design.top_boundary);

    // Целевой ряд и защита от выхода за пределы массива строк
    int32_t base_row = (y_c - design.design.bottom_boundary) / (design.design.row_height + design.design.row_spacing);
    base_row = std::clamp<int32_t>(base_row, 0, static_cast<int32_t>(rows.size()) - 1);

    const int MAX_EVALS = 30; // Бюджет вычислений для локального минимума
    int eval_count = 0;
    int same_hpwl_count = 0; // Счетчик для выхода из плато
    bool stop_search = false;

    int64_t best_hpwl = INT64_MAX;
    int64_t best_x = -1, best_y = -1;

    // Движение по Манхэттенской спирали (удаление от целевой строки: 0, 1, -1, 2, -2...)
    for (int radius = 0; radius < rows.size() && !stop_search; ++radius) {
        
        int rows_to_check[] = {base_row + radius, base_row - radius};
        int num_rows = (radius == 0) ? 1 : 2;

        for (int r = 0; r < num_rows && !stop_search; ++r) {
            int target_row = rows_to_check[r];
            if (target_row < 0 || target_row >= rows.size()) continue;

            auto& current_intervals = rows[target_row].free_intervals;
            if (current_intervals.empty()) continue;

            // Поиск ближайшего свободного интервала к x_c
            auto lower = std::lower_bound(current_intervals.begin(), current_intervals.end(), x_c,
                [](const FreeInterval& interval, int64_t val) {
                    return interval.x_start < val;
                });

            size_t center_idx = (lower == current_intervals.end()) ? current_intervals.size() - 1 : (lower - current_intervals.begin());

            // Оцениваем сам интервал и ближайших соседей слева/справа
            size_t start_idx = (center_idx > 0) ? center_idx - 1 : 0;
            size_t end_idx = std::min(center_idx + 1, current_intervals.size() - 1);

            for (size_t idx = start_idx; idx <= end_idx && !stop_search; ++idx) {
                const auto& interval = current_intervals[idx];
                
                // Проецируем x_c в рамки интервала
                int64_t pos_x = std::clamp<int64_t>(x_c, interval.x_start, interval.x_end - cell.width);
                
                // Жесткое выравнивание по сетке кратно site_width
                pos_x = (pos_x / design.params.site_width) * design.params.site_width;

                // Проверяем, что после выравнивания ячейка всё еще легально влезает
                if (pos_x >= interval.x_start && (pos_x + cell.width) <= interval.x_end) {
                    int64_t pos_y = rows[target_row].y;
                    
                    // Функция-заглушка: здесь должен быть твой реальный пересчет Bounding Box для цепей
                    int64_t current_hpwl = CalculateHPWL(cell, pos_x, pos_y, design); 
                    eval_count++;

                    if (current_hpwl < best_hpwl) {
                        best_hpwl = current_hpwl;
                        best_x = pos_x;
                        best_y = pos_y;
                        same_hpwl_count = 1; // Сброс счетчика плато при нахождении нового абсолютного минимума
                    } 
                    else if (current_hpwl == best_hpwl) {
                        same_hpwl_count++; // Накапливаем нахождение на плато
                    }

                    // Триггер выхода: исчерпан бюджет позиций ИЛИ нашли 3 одинаково лучших результата
                    if (same_hpwl_count >= 3 || eval_count >= MAX_EVALS) {
                        stop_search = true;
                    }
                }
            }
        }
    }

    // Фиксация лучшей позиции на кристалле
    if (best_x != -1) {
        cell.x = best_x;
        cell.y = best_y;
        cell.set_placed();
        // Здесь должен быть вызов функции, которая разрезает FreeInterval на части 
        // и обновляет массив current_intervals для строки best_y
    }
}
else {
        // Цель: самая нижняя (min Y) и самая левая (min X) валидная позиция (потом надо заменить на уже размещенный дизайн)
        bool found_fallback = false;
        for (size_t r = 0; r < rows.size(); ++r) {
            for (const auto& interval : rows[r].free_intervals) {
                
                // Проецируем x_start на сетку кратно site_width с округлением вверх, 
                // чтобы не вылезти левее интервала
                int64_t rem = interval.x_start % design.params.site_width; // Подразумевается, что поля перенесены в params
                int64_t aligned_x = interval.x_start;
                if (rem != 0) {
                    aligned_x += (design.params.site_width - rem);
                }

                // Проверяем легальность размещения
                if (aligned_x + cell.width <= interval.x_end) {
                    x_c = aligned_x;
                    y_c = rows[r].y;
                    found_fallback = true;
                    break; // Нашли самый левый слот в этой строке
                }
            }
            if (found_fallback) break; // Нашли самую нижнюю строку
        }

        if (!found_fallback) {
            return false; 
        }
        cell.x = x_c;
        cell.y = y_c;
        cell.set_placed();
        //Здесь нужно вызвать метод обновления FreeInterval для строки y_c
        // UpdateIntervals(rows, y_c, x_c, cell.width);
       
}
}
}

}