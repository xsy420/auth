#include "auth/Import.hpp"
#include <toml++/toml.h>
#include <fstream>
#include <iostream>
#include <filesystem>
#include "auth/Color.hpp"

bool importEntriesFromToml(const std::string& filepath, IAuthDB& db) {
    try {
        toml::table tbl;
        try {
            tbl = toml::parse_file(filepath);
        } catch (const toml::parse_error& err) {
            std::cerr << CColor::RED << "Error parsing TOML: " << err << CColor::RESET << std::endl;
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

            if (auto name = entryTable->get("name"))
                authEntry.name = name->as_string()->get();
            else
                continue;

            if (auto secret = entryTable->get("secret"))
                authEntry.secret = secret->as_string()->get();
            else
                continue;

            if (auto digits = entryTable->get("digits"))
                authEntry.digits = static_cast<uint32_t>(digits->as_integer()->get());

            if (auto period = entryTable->get("period"))
                authEntry.period = static_cast<uint32_t>(period->as_integer()->get());

            if (db.addEntry(authEntry))
                importCount++;
        }

        return importCount > 0;
    } catch (const std::exception& e) {
        std::cerr << CColor::RED << "Error importing entries: " << e.what() << CColor::RESET << std::endl;
        return false;
    }
}

bool exportEntriesToToml(const std::string& filepath, const std::vector<SAuthEntry>& entries) {
    if (entries.empty())
        return false;

    try {
        toml::table root;
        toml::array entriesArray;

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

        std::ofstream file(filepath);
        if (!file.is_open())
            return false;

        file << root;
        return true;
    } catch (const std::exception& e) {
        std::cerr << CColor::RED << "Error exporting entries: " << e.what() << CColor::RESET << std::endl;
        return false;
    }
}