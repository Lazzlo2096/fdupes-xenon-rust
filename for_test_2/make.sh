#!/bin/bash

# Функция для генерации случайного имени файла
generate_random_name() {
  cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w "$1" | head -n 1
}

# Функция для генерации случайных данных
generate_random_data() {
  dd if=/dev/random bs=1M count="$1" 2>/dev/null | base64
}

# Функция для создания случайной иерархии директорий
create_random_directory_structure() {
  local depth="$1"
  local max_depth="$2"
  local max_files="$3"

  if [ "$depth" -ge "$max_depth" ]; then
    return
  fi

  local current_directory="$(generate_random_name 8)"
  mkdir "$current_directory"
  cd "$current_directory"

  local num_files=$((RANDOM % max_files))
  for ((i = 0; i < num_files; i++)); do
    local file_name="$(generate_random_name 10)"
    #local file_size=$((RANDOM % 1024 + 1)) # Размер файла от 1 до 1024 байт
    local file_size=$((RANDOM % 64 + 1)) # Размер файла от 8 до 64 мегабайт
    generate_random_data "$file_size" >"$file_name".bin
  done

  local num_subdirectories=$((RANDOM % 3)) # Максимум 2 поддиректории
  for ((i = 0; i < num_subdirectories; i++)); do
    create_random_directory_structure "$((depth + 1))" "$max_depth" "$max_files"
    cd ..
  done
}

# Использование: ./create_random_hierarchy.sh <глубина> <макс_глубина> <макс_файлы_в_директории>
# Пример: ./create_random_hierarchy.sh 0 4 5

if [ "$#" -ne 3 ]; then
  echo "Использование: $0 <текущ. глубина. 0> <макс_глубина> <макс_файлы_в_директории>"
  echo "Например: $0 0 64 5"
  exit 1
fi

create_random_directory_structure "$1" "$2" "$3"
