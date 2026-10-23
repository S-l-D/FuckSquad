/**
 * @file Parser.cpp
 * @brief Реализация JSON-парсера
 * @author Dmitry
 */

#include "Parser.h"

#include <algorithm>
#include <cmath>
#include <fstream>
#include <limits>
#include <nlohmann/json.hpp>
#include <stdexcept>
#include <vector>

namespace tetris {

using json = nlohmann::json;

namespace {

int16_t toInt16Checked(int32_t val, const std::string &field_name) {
  if (val < std::numeric_limits<int16_t>::min() ||
      val > std::numeric_limits<int16_t>::max()) {
    throw std::runtime_error("Parser error: Value out of int16_t range for '" +
                             field_name + "'");
  }
  return static_cast<int16_t>(val);
}

} // namespace

void Parser::parseFile(const std::string &filepath, DataBase &db) {
  std::ifstream file(filepath);
  if (!file.is_open()) {
    throw std::runtime_error("Parser error: Cannot open input file '" +
                             filepath + "'");
  }
  parseStream(file, db);
}

void Parser::parseStream(std::istream &input_stream, DataBase &db) {
  db.clear();

  json j;
  try {
    input_stream >> j;
  } catch (const json::parse_error &e) {
    throw std::runtime_error(
        std::string("Parser error: Invalid JSON syntax: ") + e.what());
  }

  if (!j.is_object() || j.empty()) {
    throw std::runtime_error(
        "Parser error: Root JSON must be a non-empty object.");
  }

  // 1. Чтение секции "design"
  if (!j.contains("design") || !j["design"].is_object()) {
    throw std::runtime_error(
        "Parser error: Missing or invalid 'design' section.");
  }
  const auto &j_design = j["design"];
  db.design.chip_width = j_design.at("chip_width").get<int32_t>();
  db.design.chip_height = j_design.at("chip_height").get<int32_t>();
  db.design.row_height = j_design.at("row_height").get<int32_t>();
  db.design.row_spacing = j_design.value("row_spacing", 0);
  db.design.grid_size = j_design.value("grid_size", 1);
  db.design.left_boundary = j_design.value("left_boundary", 0);
  db.design.right_boundary =
      j_design.value("right_boundary", db.design.chip_width);
  db.design.bottom_boundary = j_design.value("bottom_boundary", 0);
  db.design.top_boundary =
      j_design.value("top_boundary", db.design.chip_height);

  if (db.design.chip_width <= 0 || db.design.chip_height <= 0 ||
      db.design.row_height <= 0 || db.design.grid_size <= 0) {
    throw std::runtime_error("Parser error: Chip dimensions, row_height, and "
                             "grid_size must be > 0.");
  }

  // 2. Чтение секции "rows"
  if (j.contains("rows") && j["rows"].is_array()) {
    db.rows.reserve(j["rows"].size());
    for (const auto &j_row : j["rows"]) {
      RowDef row;
      row.id = j_row.at("id").get<std::string>();
      row.y = j_row.at("y").get<int32_t>();
      row.height = j_row.value("height", db.design.row_height);
      row.width = j_row.value("width", db.design.chip_width);
      row.available = j_row.value("available", true);
      db.rows.push_back(std::move(row));
    }
  }

  // Фиксированные объекты (blockages и pad_locations) добавляются в массив
  // cells первыми, чтобы их численные ID всегда были меньше ID перемещаемых
  // стандартных ячеек.

  // 3. Чтение секции "blockages" (IP-блоки / препятствия)
  if (j.contains("blockages") && j["blockages"].is_array()) {
    const int16_t blk_dict_id = db.registerCellType("BLOCKAGE");
    for (const auto &j_blk : j["blockages"]) {
      const std::string id_str = j_blk.at("id").get<std::string>();
      Cell blk;
      blk.x = j_blk.at("x").get<int32_t>();
      blk.y = j_blk.at("y").get<int32_t>();
      blk.width =
          toInt16Checked(j_blk.at("width").get<int32_t>(), "blockage.width");
      blk.height =
          toInt16Checked(j_blk.at("height").get<int32_t>(), "blockage.height");
      blk.priority = 0;
      blk.type =
          static_cast<int16_t>(blk_dict_id | TypeFlags::KIND_BLOCKAGE |
                               TypeFlags::IS_FIXED | TypeFlags::IS_PLACED);

      if (blk.width <= 0 || blk.height <= 0) {
        throw std::runtime_error("Parser error: Blockage '" + id_str +
                                 "' has non-positive dimensions.");
      }

      if (blk.x < 0 || blk.y < 0 || blk.x + blk.width > db.design.chip_width ||
          blk.y + blk.height > db.design.chip_height) {
        throw std::runtime_error("Parser error: Blockage '" + id_str +
                                 "' is out of chip bounds.");
      }

      db.addCell(id_str, std::move(blk));
    }
  }

  // 4. Чтение секции "pad_locations" (внешние контакты / IO-пады)
  if (j.contains("pad_locations") && j["pad_locations"].is_array()) {
    const int16_t pad_dict_id = db.registerCellType("IO_PAD");
    for (const auto &j_pad : j["pad_locations"]) {
      const std::string id_str = j_pad.at("id").get<std::string>();
      Cell pad;
      pad.x = j_pad.at("x").get<int32_t>();
      pad.y = j_pad.at("y").get<int32_t>();
      pad.width = toInt16Checked(j_pad.value("width", 0), "pad.width");
      pad.height = toInt16Checked(j_pad.value("height", 0), "pad.height");
      pad.priority = 0;
      pad.type =
          static_cast<int16_t>(pad_dict_id | TypeFlags::KIND_IO_PAD |
                               TypeFlags::IS_FIXED | TypeFlags::IS_PLACED);

      if (pad.x < 0 || pad.y < 0 || pad.x + pad.width > db.design.chip_width ||
          pad.y + pad.height > db.design.chip_height) {
        throw std::runtime_error("Parser error: Pad '" + id_str +
                                 "' is out of chip bounds.");
      }

      db.addCell(id_str, std::move(pad));
    }
  }

  // 5. Чтение секции "cells" (стандартные перемещаемые ячейки)
  if (!j.contains("cells") || !j["cells"].is_array() || j["cells"].empty()) {
    throw std::runtime_error("Parser error: Missing or empty 'cells' array.");
  }
  for (const auto &j_cell : j["cells"]) {
    const std::string id_str = j_cell.at("id").get<std::string>();
    const std::string type_str = j_cell.value("type", "STD");
    const int16_t type_dict_id = db.registerCellType(type_str);

    Cell cell;
    cell.x = j_cell.value("x", 0);
    cell.y = j_cell.value("y", 0);
    cell.width =
        toInt16Checked(j_cell.at("width").get<int32_t>(), "cell.width");
    cell.height =
        toInt16Checked(j_cell.at("height").get<int32_t>(), "cell.height");
    cell.priority =
        toInt16Checked(j_cell.value("priority", 0), "cell.priority");
    cell.type = static_cast<int16_t>(type_dict_id | TypeFlags::KIND_STD_CELL);

    if (j_cell.contains("x") && j_cell.contains("y") &&
        j_cell.value("placed", false)) {
      cell.setPlaced(true);
    }

    if (cell.width <= 0 || cell.height <= 0) {
      throw std::runtime_error("Parser error: Invalid dimensions for cell '" +
                               id_str + "'.");
    }
    db.addCell(id_str, std::move(cell));
  }

  // 6. Чтение секции "nets", формирование пар <ID связанной ячейки, weight *
  // 10> и немедленное устранение нетов (в DataBase вектор нетов не хранится).
  if (j.contains("nets") && j["nets"].is_array()) {
    const size_t total_cells = db.cells.size();
    std::vector<std::vector<NeighborLink>> temp_adj(total_cells);

    for (const auto &j_net : j["nets"]) {
      const std::string net_name = j_net.at("net_id").get<std::string>();
      const double raw_weight = j_net.value("weight", 1.0);
      // Умножаем входной вес на 10 и записываем в int32_t
      const int32_t scaled_weight =
          static_cast<int32_t>(std::llround(raw_weight * WEIGHT_SCALE));

      const auto &j_net_cells = j_net.at("cells");
      if (!j_net_cells.is_array()) {
        throw std::runtime_error("Parser error: 'cells' in net '" + net_name +
                                 "' must be an array.");
      }

      std::vector<int32_t> net_cids;
      net_cids.reserve(j_net_cells.size());
      for (const auto &cell_name_val : j_net_cells) {
        const std::string cell_name = cell_name_val.get<std::string>();
        const int32_t cid = db.findCellId(cell_name);
        if (cid < 0) {
          throw std::runtime_error("Parser error: Net '" + net_name +
                                   "' references unknown object '" + cell_name +
                                   "'.");
        }
        net_cids.push_back(cid);
      }

      // Формируем парные связи с чистым весом цепи без деления
      const size_t k = net_cids.size();
      for (size_t u_idx = 0; u_idx < k; ++u_idx) {
        const int32_t u = net_cids[u_idx];
        for (size_t v_idx = u_idx + 1; v_idx < k; ++v_idx) {
          const int32_t v = net_cids[v_idx];
          if (u == v) {
            continue;
          }
          temp_adj[static_cast<size_t>(u)].push_back({v, scaled_weight});
          temp_adj[static_cast<size_t>(v)].push_back({u, scaled_weight});
        }
      }
    }

    // Укладываем связи в 16-байтовый CompactVec каждой ячейки (сортируя по ID и
    // суммируя кратные связи)
    for (size_t i = 0; i < total_cells; ++i) {
      auto &links = temp_adj[i];
      if (links.empty()) {
        continue;
      }

      std::sort(links.begin(), links.end(),
                [](const NeighborLink &a, const NeighborLink &b) {
                  return a.first < b.first;
                });

      size_t unique_cnt = 0;
      for (size_t j_idx = 0; j_idx < links.size(); ++j_idx) {
        if (unique_cnt > 0 &&
            links[unique_cnt - 1].first == links[j_idx].first) {
          links[unique_cnt - 1].second += links[j_idx].second;
        } else {
          links[unique_cnt++] = links[j_idx];
        }
      }
      links.resize(unique_cnt);

      db.cells[i].neighbors.reserve(static_cast<uint32_t>(links.size()));
      for (const auto &link : links) {
        db.cells[i].neighbors.push_back(link);
      }
    }
  }

  // 7. Чтение параметров алгоритма "tetris_params"
  if (j.contains("tetris_params") && j["tetris_params"].is_object()) {
    const auto &j_tp = j["tetris_params"];
    db.params.ordering_strategy =
        j_tp.value("ordering_strategy", std::string("connectivity"));
    db.params.cost_function =
        j_tp.value("cost_function", std::string("skyline_height"));
    db.params.site_width = j_tp.value("site_width", db.design.grid_size);
    db.params.rotation_allowed = j_tp.value("rotation_allowed", false);
    db.params.allow_multistory = j_tp.value("allow_multistory", false);
    db.params.gap_penalty = j_tp.value("gap_penalty", 0.1);
    db.params.boundary_penalty = j_tp.value("boundary_penalty", 0.05);
    db.params.seed = j_tp.value("seed", 42u);
  } else {
    db.params.site_width = db.design.grid_size;
  }

  // 8. Инициализация свободных интервалов в строках и вырезание IP-блоков
  // (blockages)
  db.initRowsAndBlockages();
}

void Parser::writeFile(const std::string &filepath, const DataBase &db) {
  std::ofstream file(filepath);
  if (!file.is_open()) {
    throw std::runtime_error("Parser error: Cannot open output file '" +
                             filepath + "' for writing.");
  }
  writeStream(file, db);
}

void Parser::writeStream(std::ostream &output_stream, const DataBase &db) {
  json out;

  out["design"] = {{"chip_width", db.design.chip_width},
                   {"chip_height", db.design.chip_height},
                   {"row_height", db.design.row_height},
                   {"row_spacing", db.design.row_spacing},
                   {"grid_size", db.design.grid_size},
                   {"left_boundary", db.design.left_boundary},
                   {"right_boundary", db.design.right_boundary},
                   {"top_boundary", db.design.top_boundary},
                   {"bottom_boundary", db.design.bottom_boundary}};

  json j_rows = json::array();
  for (const auto &r : db.rows) {
    j_rows.push_back({{"id", r.id},
                      {"y", r.y},
                      {"height", r.height},
                      {"width", r.width},
                      {"available", r.available}});
  }
  out["rows"] = std::move(j_rows);

  json j_cells = json::array();
  json j_blockages = json::array();
  json j_pads = json::array();

  for (size_t id = 0; id < db.cells.size(); ++id) {
    const auto &c = db.cells[id];
    const std::string &cid = db.id_to_name[id];

    if (c.isStdCell()) {
      const std::string &type_str =
          db.type_id_to_name[static_cast<size_t>(c.getTypeDictId())];
      j_cells.push_back({{"id", cid},
                         {"x", c.x},
                         {"y", c.y},
                         {"width", c.effectiveWidth()},
                         {"height", c.effectiveHeight()},
                         {"type", type_str},
                         {"priority", c.priority},
                         {"placed", c.isPlaced()}});
    } else if (c.isBlockage()) {
      j_blockages.push_back({{"id", cid},
                             {"x", c.x},
                             {"y", c.y},
                             {"width", c.effectiveWidth()},
                             {"height", c.effectiveHeight()}});
    } else if (c.isIoPad()) {
      j_pads.push_back({{"id", cid}, {"x", c.x}, {"y", c.y}});
    }
  }

  out["blockages"] = std::move(j_blockages);
  out["cells"] = std::move(j_cells);
  out["pad_locations"] = std::move(j_pads);

  output_stream << out.dump(2) << '\n';
}

} // namespace tetris