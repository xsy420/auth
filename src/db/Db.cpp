#include "Db.hpp"
#include <fstream>
#include <iostream>
#include <filesystem>
#include <toml++/toml.h>

CFileAuthDB::CFileAuthDB(const std::string& path) : m_path(path) {
    const std::filesystem::path configPath = std::filesystem::path(path).parent_path();
    if (!std::filesystem::exists(configPath))
        std::filesystem::create_directories(configPath);
}

bool CFileAuthDB::load() {
    if (!std::filesystem::exists(m_path))
        return false;

    toml::table tbl;
    try {
        tbl = toml::parse_file(m_path);
    } catch (const toml::parse_error& err) {
        std::cerr << "Error parsing TOML: " << err << std::endl;
        return false;
    } catch (const std::exception& e) {
        std::cerr << "Error loading database: " << e.what() << std::endl;
        return false;
    }

    m_entries.clear();
    m_nextId = 1;

    auto entries = tbl["entries"].as_array();
    if (!entries)
        return false;

    for (const auto& entry : *entries) {
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

        if (auto id = entryTable->get("id"))
            authEntry.id = static_cast<uint64_t>(id->as_integer()->get());
        else
            authEntry.id = m_nextId++;

        m_nextId = std::max(m_nextId, authEntry.id + 1);
        m_entries.push_back(authEntry);
    }

    return true;
}

bool CFileAuthDB::save() {
    try {
        toml::table root;
        toml::array entriesArray;

        for (const auto& entry : m_entries) {
            toml::table entryTable;
            entryTable.insert("name", entry.name);
            entryTable.insert("secret", entry.secret);
            entryTable.insert("digits", static_cast<int64_t>(entry.digits));
            entryTable.insert("period", static_cast<int64_t>(entry.period));
            entryTable.insert("id", static_cast<int64_t>(entry.id));

            entriesArray.push_back(entryTable);
        }

        root.insert("entries", entriesArray);

        std::ofstream file(m_path);
        if (!file.is_open())
            return false;

        file << root;
        return true;
    } catch (const std::exception& e) {
        std::cerr << "Error saving database: " << e.what() << std::endl;
        return false;
    }
}

std::vector<SAuthEntry> CFileAuthDB::getEntries() {
    return m_entries;
}

bool CFileAuthDB::addEntry(const SAuthEntry& entry) {
    SAuthEntry newEntry = entry;
    newEntry.id         = m_nextId++;
    m_entries.push_back(newEntry);
    return save();
}

bool CFileAuthDB::removeEntry(uint64_t id) {
    auto it = std::ranges::find_if(m_entries, [id](const SAuthEntry& entry) { return entry.id == id; });

    if (it != m_entries.end()) {
        m_entries.erase(it);
        return save();
    }

    return false;
}

bool CFileAuthDB::updateEntry(const SAuthEntry& entry) {
    auto it = std::ranges::find_if(m_entries, [&entry](const SAuthEntry& e) { return e.id == entry.id; });

    if (it != m_entries.end()) {
        *it = entry;
        return save();
    }

    return false;
}