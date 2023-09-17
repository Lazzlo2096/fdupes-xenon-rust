#include <iostream>
#include <fstream>
#include <filesystem>
#include <unordered_map>
#include <vector>
#include <string>
#include <cstring>
#include <iomanip>
#include <openssl/md5.h>
#include <getopt.h>

namespace fs = std::filesystem;

// Функция для вычисления MD5 хеша файла
std::string computeMD5(const std::string& filePath) {
    std::ifstream file(filePath, std::ios::binary);
    if (!file) {
        return "";
    }

    MD5_CTX md5Context;
    MD5_Init(&md5Context);

    char buffer[1024];
    while (file.good()) {
        file.read(buffer, sizeof(buffer));
        MD5_Update(&md5Context, buffer, file.gcount());
    }

    unsigned char hash[MD5_DIGEST_LENGTH];
    MD5_Final(hash, &md5Context);

    std::stringstream ss;
    ss << std::hex << std::setfill('0');
    for (int i = 0; i < MD5_DIGEST_LENGTH; ++i) {
        ss << std::setw(2) << static_cast<unsigned int>(hash[i]);
    }

    return ss.str();
}

// Функция для сканирования директории и поиска дубликатов
void findDuplicates(const fs::path& directory, bool recursive) {
    std::unordered_map<std::string, std::vector<fs::path>> hashToPaths;

    for (const auto& entry : fs::directory_iterator(directory)) {
        const fs::path& path = entry.path();

        if (fs::is_directory(path) && recursive) {
            findDuplicates(path, true);
        } else if (fs::is_regular_file(path)) {
            std::string hash = computeMD5(path.string());

            if (!hash.empty()) {
                hashToPaths[hash].push_back(path);
            }
        }
    }

    for (const auto& [hash, paths] : hashToPaths) {
        if (paths.size() > 1) {
            std::cout << "---- hash: " << hash << " ----" << std::endl;
            for (const auto& path : paths) {
                std::cout << path << std::endl;
            }
        }
    }
}

int main(int argc, char* argv[]) {
    bool recursive = false;
    std::vector<std::string> paths;

    // Обработка аргументов командной строки
    int option;
    while ((option = getopt(argc, argv, "r")) != -1) {
        switch (option) {
            case 'r':
                recursive = true;
                break;
            default:
                std::cerr << "Usage: " << argv[0] << " [-r] path1 path2 ..." << std::endl;
                return 1;
        }
    }

    for (int i = optind; i < argc; ++i) {
        paths.push_back(argv[i]);
    }

    for (const auto& path : paths) {
        findDuplicates(path, recursive);
    }

    return 0;
}
