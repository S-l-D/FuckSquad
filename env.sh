#!/usr/bin/env bash

# Прерывать выполнение при любой ошибке
set -euo pipefail

# Цвета для вывода
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info()    { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[OK]${NC} $1"; }
log_warn()    { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error()   { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

# Проверка прав суперпользователя
SUDO=""
if [ "$EUID" -ne 0 ]; then
    if command -v sudo >/dev/null 2>&1; then
        SUDO="sudo"
    else
        log_error "Скрипт требует прав root или установленной утилиты sudo."
    fi
fi

# Определение дистрибутива Linux
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS_ID=$ID
    OS_LIKE=${ID_LIKE:-$ID}
else
    log_error "Не удалось определить дистрибутив Linux (/etc/os-release не найден)."
fi

log_info "Обнаружена система: ${PRETTY_NAME:-$OS_ID}"

install_debian_ubuntu() {
    log_info "Обновление списков пакетов (apt)..."
    $SUDO apt-get update -qq

    log_info "1/4. Установка тулчейна Clang, Ninja, CMake и анализаторов кода..."
    $SUDO apt-get install -y \
        build-essential \
        clang \
        lld \
        lldb \
        clang-format \
        clang-tidy \
        cmake \
        ninja-build \
        git \
        pkg-config

    log_info "2/4. Установка C++ библиотек (JSON, GTest, Benchmark, OpenMP, CLI11)..."
    $SUDO apt-get install -y \
        nlohmann-json3-dev \
        libgtest-dev \
        libgmock-dev \
        libbenchmark-dev \
        libomp-dev \
        libcli11-dev || log_warn "Опциональные пакеты при необходимости будут подтянуты через CMake FetchContent."

    log_info "3/4. Установка графических библиотек для C++ визуализации (SFML, Gnuplot)..."
    $SUDO apt-get install -y \
        libsfml-dev \
        gnuplot-nox

    log_info "4/4. Установка инструментов профилирования, документации и Bash-автоматизации..."
    $SUDO apt-get install -y \
        valgrind \
        doxygen \
        graphviz \
        jq \
        time
}

install_fedora() {
    log_info "Установка пакетов через dnf..."
    $SUDO dnf install -y \
        clang \
        clang-tools-extra \
        lld \
        lldb \
        cmake \
        ninja-build \
        git \
        gcc-c++ \
        json-devel \
        gtest-devel \
        gmock-devel \
        google-benchmark-devel \
        libomp-devel \
        cli11-devel \
        SFML-devel \
        gnuplot \
        valgrind \
        doxygen \
        graphviz \
        jq \
        time
}

install_arch() {
    log_info "Установка пакетов через pacman..."
    $SUDO pacman -Syu --needed --noconfirm \
        base-devel \
        clang \
        lld \
        lldb \
        cmake \
        ninja \
        git \
        nlohmann-json \
        gtest \
        benchmark \
        openmp \
        cli11 \
        sfml \
        gnuplot \
        valgrind \
        doxygen \
        graphviz \
        jq \
        time
}

# Маршрутизация по типу ОС
if [[ "$OS_ID" == "ubuntu" || "$OS_ID" == "debian" || "$OS_LIKE" == *"debian"* || "$OS_LIKE" == *"ubuntu"* ]]; then
    install_debian_ubuntu
elif [[ "$OS_ID" == "fedora" || "$OS_LIKE" == *"fedora"* || "$OS_LIKE" == *"rhel"* ]]; then
    install_fedora
elif [[ "$OS_ID" == "arch" || "$OS_LIKE" == *"arch"* ]]; then
    install_arch
else
    log_error "Дистрибутив '$OS_ID' не поддерживается автоматически. Установите пакеты вручную."
fi

echo ""
log_success "Нативное C++17 окружение для проекта 'Тетрис (ЭН-45 Команда 1)' успешно установлено!"
echo "-------------------------------------------------------------------------"
echo -e "Проверка версий установленного стека:"
echo -e "  * Компилятор:   $(clang++ --version | head -n 1)"
echo -e "  * Линкер LLD:   $(ld.lld --version | head -n 1)"
echo -e "  * CMake:        $(cmake --version | head -n 1)"
echo -e "  * Ninja:        $(ninja --version)"
echo -e "  * Clang-Tidy:   $(clang-tidy --version | head -n 1 | xargs)"
echo -e "  * Valgrind:     $(valgrind --version)"
echo "-------------------------------------------------------------------------"
echo -e "Команды для инициализации и сборки проекта:"
echo -e "  ${YELLOW}cmake -B build -G Ninja -DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++ -DCMAKE_BUILD_TYPE=Release${NC}"
echo -e "  ${YELLOW}cmake --build build${NC}"
echo -e "  ${YELLOW}ctest --test-dir build --output-on-failure${NC}"