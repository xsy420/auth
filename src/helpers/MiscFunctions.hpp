#pragma once

#include <string>
#include <vector>
#include <cstdint>
#include <optional>
#include "../db/Db.hpp"

void                      StringToLowerInPlace(std::string& str);

std::string               StringToLower(const std::string& str);

std::vector<uint8_t>      DecodeBase32(const std::string& input);

std::string               GetHomeDir();

std::optional<SAuthEntry> FindEntryByNameOrId(const std::vector<SAuthEntry>& entries, const std::string& nameOrId);

bool                      ValidateDigits(uint32_t digits, std::string& errorMessage);

bool                      ValidatePeriod(uint32_t period, std::string& errorMessage);

bool                      IsSecretValid(const std::string& secret, std::string& errorMessage);

std::string               GetDatabasePath();
