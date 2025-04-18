#include "ImportExport.hpp"
#include <toml++/toml.h>
#include <nlohmann/json.hpp>
#include <fstream>
#include <iostream>
#include <filesystem>
#include "Color.hpp"

static bool importEntriesFromToml(const std::string& filepath, IAuthDB& db) {
    toml::table tbl;
    try {
        tbl = toml::parse_file(filepath);
    } catch (const toml::parse_error& err) {
        std::cerr << CColor::RED << "Error parsing TOML: " << err << CColor::RESET << std::endl;
        return false;
    } catch (const std::exception& e) {
        std::cerr << CColor::RED << "Error importing entries: " << e.what() << CColor::RESET << std::endl;
        return false;
    }

    auto entriesArray = tbl["entries"].as_array();
    if (!entriesArray) {
        std::cerr << CColor::RED << "Missing 'entries' array in TOML file" << CColor::RESET << std::endl;
        return false;
    }

    int importCount = 0;

    for (const auto& entry : *entriesArray) {
        auto entryTable = entry.as_table();
        if (!entryTable)
            continue;

        SAuthEntry authEntry;

        auto       name = entryTable->get("name");
        if (!name)
            continue;
        authEntry.name = name->as_string()->get();

        auto secret = entryTable->get("secret");
        if (!secret)
            continue;
        authEntry.secret = secret->as_string()->get();

        if (auto digits = entryTable->get("digits"))
            authEntry.digits = static_cast<uint32_t>(digits->as_integer()->get());

        if (auto period = entryTable->get("period"))
            authEntry.period = static_cast<uint32_t>(period->as_integer()->get());

        if (db.addEntry(authEntry))
            importCount++;
    }

    return importCount > 0;
}

static bool exportEntriesToToml(const std::string& filepath, const std::vector<SAuthEntry>& entries) {
    if (entries.empty())
        return false;

    toml::table root;
    toml::array entriesArray;

    try {
        for (const auto& entry : entries) {
            toml::table entryTable;
            entryTable.insert("name", entry.name);
            entryTable.insert("secret", entry.secret);

            if (entry.digits != 6)
                entryTable.insert("digits", static_cast<int64_t>(entry.digits));

            if (entry.period != 30)
                entryTable.insert("period", static_cast<int64_t>(entry.period));

            entriesArray.push_back(entryTable);
        }

        root.insert("entries", entriesArray);
    } catch (const std::exception& e) {
        std::cerr << CColor::RED << "Error creating TOML data: " << e.what() << CColor::RESET << std::endl;
        return false;
    }

    std::ofstream file(filepath);
    if (!file.is_open()) {
        std::cerr << CColor::RED << "Error opening file for writing: " << filepath << CColor::RESET << std::endl;
        return false;
    }

    try {
        file << root;
        return true;
    } catch (const std::exception& e) {
        std::cerr << CColor::RED << "Error writing to file: " << e.what() << CColor::RESET << std::endl;
        return false;
    }
}

static bool importEntriesFromJson(const std::string& filepath, IAuthDB& db) {
    std::ifstream file(filepath);
    if (!file.is_open()) {
        std::cerr << CColor::RED << "Error opening file: " << filepath << CColor::RESET << std::endl;
        return false;
    }

    nlohmann::json root;
    try {
        file >> root;
    } catch (const nlohmann::json::parse_error& err) {
        std::cerr << CColor::RED << "Error parsing JSON: " << err.what() << CColor::RESET << std::endl;
        return false;
    } catch (const std::exception& e) {
        std::cerr << CColor::RED << "Error importing entries: " << e.what() << CColor::RESET << std::endl;
        return false;
    }

    if (!root.contains("entries") || !root["entries"].is_array()) {
        std::cerr << CColor::RED << "Missing 'entries' array in JSON file" << CColor::RESET << std::endl;
        return false;
    }

    const auto& entries     = root["entries"];
    int         importCount = 0;

    for (const auto& entry : entries) {
        if (!entry.is_object())
            continue;

        if (!entry.contains("name") || !entry["name"].is_string())
            continue;

        if (!entry.contains("secret") || !entry["secret"].is_string())
            continue;

        SAuthEntry authEntry;
        authEntry.name   = entry["name"].get<std::string>();
        authEntry.secret = entry["secret"].get<std::string>();

        if (entry.contains("digits") && entry["digits"].is_number())
            authEntry.digits = entry["digits"].get<uint32_t>();

        if (entry.contains("period") && entry["period"].is_number())
            authEntry.period = entry["period"].get<uint32_t>();

        if (db.addEntry(authEntry))
            importCount++;
    }

    return importCount > 0;
}

static bool exportEntriesToJson(const std::string& filepath, const std::vector<SAuthEntry>& entries) {
    if (entries.empty())
        return false;

    nlohmann::json root;
    nlohmann::json entriesArray = nlohmann::json::array();

    try {
        for (const auto& entry : entries) {
            nlohmann::json entryObj;
            entryObj["name"]   = entry.name;
            entryObj["secret"] = entry.secret;

            if (entry.digits != 6)
                entryObj["digits"] = entry.digits;

            if (entry.period != 30)
                entryObj["period"] = entry.period;

            entriesArray.push_back(entryObj);
        }

        root["entries"] = entriesArray;
    } catch (const std::exception& e) {
        std::cerr << CColor::RED << "Error creating JSON data: " << e.what() << CColor::RESET << std::endl;
        return false;
    }

    std::ofstream file(filepath);
    if (!file.is_open()) {
        std::cerr << CColor::RED << "Error opening file for writing: " << filepath << CColor::RESET << std::endl;
        return false;
    }

    try {
        file << root.dump(4);
        return true;
    } catch (const std::exception& e) {
        std::cerr << CColor::RED << "Error writing to file: " << e.what() << CColor::RESET << std::endl;
        return false;
    }
}

bool importEntries(const std::string& filepath, IAuthDB& db, EFileFormat format) {
    switch (format) {
        case EFileFormat::TOML: return importEntriesFromToml(filepath, db);
        case EFileFormat::JSON: return importEntriesFromJson(filepath, db);
        default: std::cerr << CColor::RED << "Unsupported file format" << CColor::RESET << std::endl; return false;
    }
}

bool exportEntries(const std::string& filepath, const std::vector<SAuthEntry>& entries, EFileFormat format) {
    switch (format) {
        case EFileFormat::TOML: return exportEntriesToToml(filepath, entries);
        case EFileFormat::JSON: return exportEntriesToJson(filepath, entries);
        default: std::cerr << CColor::RED << "Unsupported file format" << CColor::RESET << std::endl; return false;
    }
}