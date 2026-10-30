/**
 * @file DataBase.h
 * @brief База данных для стандартных ячеек
 * @author Dmitry
 */

#pragma once

#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <new>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

namespace tetris {

/**
 * @brief 16-байтовая альтернатива std::vector для хранения связей внутри Cell.
 * Содержит указатель data_ (8 байт), size_ (4 байта) и capacity_ (4 байта) = 16
 * байт.
 */
template <typename T> class CompactVec {
public:
  CompactVec() noexcept = default;

  ~CompactVec() noexcept { std::free(data_); }

  CompactVec(const CompactVec &other) {
    if (other.size_ > 0) {
      reserve(other.size_);
      std::memcpy(data_, other.data_, other.size_ * sizeof(T));
      size_ = other.size_;
    }
  }

  CompactVec &operator=(const CompactVec &other) {
    if (this != &other) {
      clear();
      if (other.size_ > 0) {
        reserve(other.size_);
        std::memcpy(data_, other.data_, other.size_ * sizeof(T));
        size_ = other.size_;
      }
    }
    return *this;
  }

  CompactVec(CompactVec &&other) noexcept
      : data_(other.data_), size_(other.size_), capacity_(other.capacity_) {
    other.data_ = nullptr;
    other.size_ = 0;
    other.capacity_ = 0;
  }

  CompactVec &operator=(CompactVec &&other) noexcept {
    if (this != &other) {
      std::free(data_);
      data_ = other.data_;
      size_ = other.size_;
      capacity_ = other.capacity_;
      other.data_ = nullptr;
      other.size_ = 0;
      other.capacity_ = 0;
    }
    return *this;
  }

  void reserve(uint32_t new_cap) {
    if (new_cap <= capacity_) {
      return;
    }
    T *new_mem = static_cast<T *>(std::realloc(data_, new_cap * sizeof(T)));
    if (!new_mem) {
      throw std::bad_alloc();
    }
    data_ = new_mem;
    capacity_ = new_cap;
  }

  void push_back(const T &val) {
    if (size_ == capacity_) {
      reserve(capacity_ == 0 ? 4 : capacity_ * 2);
    }
    data_[size_++] = val;
  }

  void clear() noexcept { size_ = 0; }

  uint32_t size() const noexcept { return size_; }
  bool empty() const noexcept { return size_ == 0; }

  T *begin() noexcept { return data_; }
  T *end() noexcept { return data_ + size_; }
  const T *begin() const noexcept { return data_; }
  const T *end() const noexcept { return data_ + size_; }

  T &operator[](uint32_t idx) noexcept { return data_[idx]; }
  const T &operator[](uint32_t idx) const noexcept { return data_[idx]; }

private:
  T *data_{nullptr};
  uint32_t size_{0};
  uint32_t capacity_{0};
};

static_assert(sizeof(CompactVec<std::pair<int32_t, int32_t>>) == 16,
              "CompactVec must be exactly 16 bytes!");

/**
 * @brief Кодирование поля int16_t Cell::type (числовой ID типа из словаря +
 * флаги падов и IP-блоков).
 */
namespace TypeFlags {
constexpr int16_t DICT_ID_MASK =
    0x03FF; ///< Младшие 10 бит (0..9): ID текстового типа в словаре

constexpr int16_t KIND_MASK = (0b11 << 10); ///< Биты 10..11: категория объекта
constexpr int16_t KIND_STD_CELL =
    (0b00 << 10); ///< Стандартная перемещаемая ячейка
constexpr int16_t KIND_BLOCKAGE =
    (0b01 << 10); ///< IP-блок / препятствие (blockage)
constexpr int16_t KIND_IO_PAD =
    (0b10 << 10); ///< Внешний контакт (pad_location)

constexpr int16_t IS_FIXED = (1 << 12);  ///< Бит 12: координаты зафиксированы
constexpr int16_t IS_PLACED = (1 << 13); ///< Бит 13: ячейка размещена
constexpr int16_t IS_ROTATED_90 =
    (1 << 14); ///< Бит 14: повернута на 90/270 градусов
} // namespace TypeFlags

/// Множитель для входных весов цепей (умножаем на 10 и записываем в int32_t)
constexpr int32_t WEIGHT_SCALE = 10;

struct nei {
int32_t ID_cell;
int32_t number_net;
int32_t weight;
};
/// Связь с соседней ячейкой: pair<int32_t ID, int32_t weight>
using NeighborLink = nei;

/**
 * @brief Класс ячейки (строго по класс.md).
 * ID ячейки — её номер в массиве DataBase::cells.
 */
struct Cell {
  int32_t x{0}; ///< Координата X левого нижнего угла
  int32_t y{0}; ///< Координата Y левого нижнего угла
  CompactVec<NeighborLink>
      neighbors;       ///< 16-байтовый вектор 
  int16_t width{0};    ///< Ширина ячейки
  int16_t height{0};   ///< Высота ячейки
  int16_t priority{0}; ///< Приоритет ячейки
  int16_t type{0};     ///< Числовой тип из словаря + флаги падов и IP-блоков + флаг is_placed (1 бит)

  // Вспомогательные геттеры и сеттеры для поля type
  int16_t getTypeDictId() const noexcept {
    return type & TypeFlags::DICT_ID_MASK;
  }
 
  void set_placed() noexcept {type = type ^ 0x0001;}
  bool is_placed() const noexcept {return type & 0x0001;}
  int16_t getKind() const noexcept { return type & TypeFlags::KIND_MASK; }
  bool isStdCell() const noexcept {
    return getKind() == TypeFlags::KIND_STD_CELL;
  }
  bool isBlockage() const noexcept {
    return getKind() == TypeFlags::KIND_BLOCKAGE;
  }
  bool isIoPad() const noexcept { return getKind() == TypeFlags::KIND_IO_PAD; }

  bool isFixed() const noexcept { return (type & TypeFlags::IS_FIXED) != 0; }
  bool isPlaced() const noexcept { return (type & TypeFlags::IS_PLACED) != 0; }
  bool isRotated() const noexcept {
    return (type & TypeFlags::IS_ROTATED_90) != 0;
  }

  void setPlaced(bool placed) noexcept {
    if (placed) {
      type |= TypeFlags::IS_PLACED;
    } else {
      type &= static_cast<int16_t>(~TypeFlags::IS_PLACED);
    }
  }

  void setPosition(int32_t new_x, int32_t new_y) noexcept {
    x = new_x;
    y = new_y;
    setPlaced(true);
  }

  int16_t effectiveWidth() const noexcept {
    return isRotated() ? height : width;
  }
  int16_t effectiveHeight() const noexcept {
    return isRotated() ? width : height;
  }
};

static_assert(sizeof(Cell) == 32, "Cell size must be exactly 32 bytes!");

/**
 * @brief Свободный интервал [x_start, x_end) в строке для оконных запросов и
 * поиска позиций.
 */
struct FreeInterval {
  int32_t x_start{0};
  int32_t x_end{0};

  constexpr int32_t width() const noexcept { return x_end - x_start; }
  constexpr bool operator<(const FreeInterval &other) const noexcept {
    return x_start < other.x_start;
  }
};

/**
 * @brief Строка размещения (Row) с упорядоченным списком свободных интервалов.
 */
struct RowDef {
  std::string id;
  int32_t y{0};
  int32_t height{0};
  int32_t width{0};
  bool available{true};

  /// Доступные для размещения интервалы в строке (всегда отсортированы по
  /// возрастанию x_start)
  std::vector<FreeInterval> free_intervals;

  /// Инициализирует свободный интервал на ширину строки в пределах границ
  /// кристалла
  void initIntervals(int32_t left_bound, int32_t right_bound);

  /// Вырезает занятый отрезок [occ_x1, occ_x2] из свободных интервалов
  void occupyInterval(int32_t occ_x1, int32_t occ_x2);

  /// Оконный запрос поиском делением пополам O(log K): поместится ли ячейка
  /// ширины cell_w в точку x?
  bool canPlaceAt(int32_t x, int32_t cell_w) const noexcept;
};

/**
 * @brief Глобальные параметры кристалла и сетки размещения (секция "design").
 */
struct DesignParams {
  int32_t chip_width{0};
  int32_t chip_height{0};
  int32_t row_height{0};
  int32_t row_spacing{0};
  int32_t grid_size{1};
  int32_t left_boundary{0};
  int32_t right_boundary{0};
  int32_t top_boundary{0};
  int32_t bottom_boundary{0};
};


enum class ordering_strategy : int16_t {
connectivity = 0,
priority,
width_desc,
width_asc,
random
};
/**
 * @brief Параметры алгоритма Тетрис (секция "tetris_params").
 */
struct TetrisParams {
  ordering_strategy strategy{ordering_strategy::connectivity};
  std::string cost_function{"skyline_height"};
  int32_t site_width{1};
  bool rotation_allowed{false};
  bool allow_multistory{false};
  double gap_penalty{0.1};
  double boundary_penalty{0.05};
  uint32_t seed{42};
};

/**
 * @brief Центральная база данных топологии.
 * В единый момент времени топология представляет собой плоский вектор объектов
 * ячеек (cells).
 */
class DataBase {
public:
  DataBase() = default;

  DesignParams design{};
  TetrisParams params{};

  /// Плоский вектор объектов ячеек (ID ячейки — её индекс в этом массиве)
  std::vector<Cell> cells;

  /// Строки размещения со свободными интервалами
  std::vector<RowDef> rows;

  // --- Словари: число <-> текстовый ID из файла ---
  std::vector<std::string> id_to_name;
  std::unordered_map<std::string, int32_t> name_to_id;

  // --- Словари: число <-> текстовый тип ячейки ---
  std::vector<std::string> type_id_to_name;
  std::unordered_map<std::string, int16_t> type_name_to_id;

  /**
   * @brief Полностью очищает базу данных.
   */
  void clear();

  /**
   * @brief Регистрирует строковый тип ячейки ("NAND2", "INV", "BLOCKAGE",
   * "IO_PAD") в словаре.
   * @return Числовой ID типа (0..1023).
   */
  int16_t registerCellType(const std::string &type_name);

  std::vector<RowDef>& getRows() {return rows;};
  std::vector<Cell>& getCells() { return cells;};

  /**
   * @brief Добавляет ячейку в вектор cells и регистрирует её текстовый ID в
   * словаре.
   * @return Численный ID ячейки (её индекс в массиве cells).
   */
  int32_t addCell(const std::string &text_id, Cell cell);

  /**
   * @brief Возвращает численный ID ячейки по текстовому имени за O(1) (или -1,
   * если не найдена).
   */
  int32_t findCellId(const std::string &text_id) const;

  /**
   * @brief Перестраивает вектор cells согласно новому порядку индексов (после
   * сортировки по стратегии), автоматически обновляя словари id_to_name /
   * name_to_id и перенумеровывая ID внутри Cell::neighbors.
   * @param new_to_old_order Массив старых ID в новом порядке (размер равен
   * cells.size()).
   */
  void reorderCells(const std::vector<int32_t> &new_to_old_order);

  /**
   * @brief Инициализирует свободные интервалы в строках и вырезает из них
   * фиксированные IP-блоки (BLOCKAGE).
   */
  void initRowsAndBlockages();
};

} // namespace tetris