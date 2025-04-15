#include "ImportExport.hpp"
#include <toml++/toml.h>
#include <fstream>
#include <iostream>
#include <filesystem>
#include "../core/Color.hpp"

bool importEntriesFromToml(const std::string& filepath, IAuthDB& db) {
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

bool exportEntriesToToml(const std::string& filepath, const std::vector<SAuthEntry>& entries) {
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