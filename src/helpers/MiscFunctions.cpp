#include "MiscFunctions.hpp"
#include <algorithm>
#include <cstring>
#include <pwd.h>
#include <unistd.h>
#include <filesystem>
#include <iostream>

void StringToLowerInPlace(std::string& str) {
    std::transform(str.begin(), str.end(), str.begin(), [](unsigned char c) { return std::tolower(c); });
}

std::string StringToLower(const std::string& str) {
    std::string result = str;
    StringToLowerInPlace(result);
    return result;
}

std::vector<uint8_t> DecodeBase32(const std::string& input) {
    constexpr const char* BASE32_CHARS = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

    std::string           sanitized;
    for (char c : input) {
        if (c != ' ' && c != '-')
            sanitized += std::toupper(c);
    }

    std::vector<uint8_t> result;
    size_t               buffer   = 0;
    size_t               bitsLeft = 0;

    for (char c : sanitized) {
        const char* pos = strchr(BASE32_CHARS, c);
        if (pos == nullptr)
            continue;

        size_t val = pos - BASE32_CHARS;
        buffer <<= 5;
        buffer |= val;
        bitsLeft += 5;

        if (bitsLeft >= 8) {
            bitsLeft -= 8;
            result.push_back((buffer >> bitsLeft) & 0xFF);
        }
    }

    return result;
}

std::string GetHomeDir() {
    const char* homeDir = getenv("HOME");
    if (homeDir == nullptr)
        homeDir = getpwuid(getuid())->pw_dir;
    return homeDir ? homeDir : "";
}

std::optional<SAuthEntry> FindEntryByNameOrId(const std::vector<SAuthEntry>& entries, const std::string& nameOrId) {
    try {
        uint64_t num = std::stoull(nameOrId);

        auto     it = std::ranges::find_if(entries, [num](const SAuthEntry& e) { return e.id == num; });
        if (it != entries.end())
            return *it;

        if (num > 0 && num <= entries.size()) {
            std::vector<SAuthEntry> sortedEntries = entries;
            std::ranges::sort(sortedEntries, [](const SAuthEntry& a, const SAuthEntry& b) { return a.id < b.id; });

            return sortedEntries[num - 1];
        }
    } catch (const std::exception&) {
        // no-op
    }

    auto it = std::ranges::find_if(entries, [&nameOrId](const SAuthEntry& e) { return e.name == nameOrId; });
    if (it != entries.end())
        return *it;

    return std::nullopt;
}

bool ValidateDigits(uint32_t digits, std::string& errorMessage) {
    if (digits < 6 || digits > 8) {
        errorMessage = "Digits must be between 6 and 8";
        return false;
    }
    return true;
}

bool ValidatePeriod(uint32_t period, std::string& errorMessage) {
    if (period == 0) {
        errorMessage = "Period cannot be 0";
        return false;
    }
    return true;
}

bool IsSecretValid(const std::string& secret, std::string& errorMessage) {
    for (char c : secret) {
        if (c != ' ' && c != '-' && !std::isalnum(c)) {
            errorMessage = "Secret contains invalid characters";
            return false;
        }
    }
    return true;
}

std::string GetDatabasePath() {
    std::string dbPath;
    const char* dbDir = getenv("AUTH_DATABASE_DIR");

    if (dbDir)
        dbPath = std::string(dbDir) + "/auth.db";
    else {
        std::string homeDir = GetHomeDir();
        if (homeDir.empty())
            return "";

        dbPath = homeDir + "/.local/share/auth/auth.db";
    }

    return dbPath;
}
