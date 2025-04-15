#pragma once

#include "../db/Db.hpp"
#include <string>
#include <vector>

bool importEntriesFromToml(const std::string& filepath, IAuthDB& db);
bool exportEntriesToToml(const std::string& filepath, const std::vector<SAuthEntry>& entries);