#pragma once

#include "../db/Db.hpp"
#include <string>
#include <vector>

enum class EFileFormat {
    TOML,
    JSON
};

bool importEntries(const std::string& filepath, IAuthDB& db, EFileFormat format);
bool exportEntries(const std::string& filepath, const std::vector<SAuthEntry>& entries, EFileFormat format);