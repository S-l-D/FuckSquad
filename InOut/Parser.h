/**
 * @file Parser.h
 * @brief Чтение входного JSON-файла
 * @author Dmitry
 */

#pragma once

#include "DataBase.h"

#include <istream>
#include <ostream>
#include <string>

namespace tetris {

class Parser {
public:
  /**
   * @brief Считывает JSON-файл, заполняет плоский вектор cells, словари имён и
   * типов
   */
  static void parseFile(const std::string &filepath, DataBase &db);

  /**
   * @brief Считывает JSON из входного потока.
   */
  static void parseStream(std::istream &input_stream, DataBase &db);

  /**
   * @brief Записывает выходной JSON-файл с координатами размещения ячеек (без
   * секции nets).
   */
  static void writeFile(const std::string &filepath, const DataBase &db);

  /**
   * @brief Записывает выходной JSON в поток вывода.
   */
  static void writeStream(std::ostream &output_stream, const DataBase &db);
};

} // namespace tetris