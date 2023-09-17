#!/bin/bash

# Путь к директории, из которой нужно скопировать файлы
source_directory="$1"

# Получаем список файлов в исходной директории
#files=("$source_directory"/*)
#files=$(find "$source_directory" -type f)
mapfile -t files < <(find "$source_directory" -type f)
#echo ${files[@]}


# Проверяем, есть ли файлы в исходной директории
if [ ${#files[@]} -eq 0 ]; then
  echo "В исходной директории нет файлов."
  exit 1
fi

# Выбираем случайный файл из списка
random_file="${files[RANDOM % ${#files[@]}]}"
#echo "lol = "${#files[@]}


# Получаем список поддиректорий в исходной директории
#subdirectories=("$source_directory"/*/)
#subdirectories=$(find "$source_directory" -type d)
mapfile -t subdirectories < <(find "$source_directory" -type d)
#echo ${subdirectories[@]}


# Проверяем, есть ли поддиректории
if [ ${#subdirectories[@]} -eq 0 ]; then
  echo "В исходной директории нет поддиректорий."
  exit 1
fi

# Выбираем случайную поддиректорию из списка
random_subdirectory="${subdirectories[RANDOM % ${#subdirectories[@]}]}"

# Генерируем случайное имя для файла
new_filename=$(head /dev/urandom | tr -dc A-Za-z0-9 | head -c 10)

# Полный путь для нового файла
new_filepath="$random_subdirectory/$new_filename".bin

# Копируем случайный файл в случайную поддиректорию и переименовываем
cp "$random_file" "$new_filepath"
echo "Скопирован и переименован файл $random_file в $new_filepath"
