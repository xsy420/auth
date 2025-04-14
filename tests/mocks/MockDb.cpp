#include "MockDb.hpp"
#include <algorithm>
#include <filesystem>

bool CMockAuthDB::load() {
    return true;
}

bool CMockAuthDB::save() {
    return true;
}

std::vector<SAuthEntry> CMockAuthDB::getEntries() {
    return m_entries;
}

bool CMockAuthDB::addEntry(const SAuthEntry& entry) {
    SAuthEntry newEntry = entry;
    newEntry.id         = m_nextId++;
    m_entries.push_back(newEntry);
    return true;
}

bool CMockAuthDB::removeEntry(uint64_t id) {
    auto it = std::ranges::find_if(m_entries, [id](const SAuthEntry& entry) { return entry.id == id; });

    if (it != m_entries.end()) {
        m_entries.erase(it);
        return true;
    }

    return false;
}

bool CMockAuthDB::updateEntry(const SAuthEntry& entry) {
    auto it = std::ranges::find_if(m_entries, [&entry](const SAuthEntry& e) { return e.id == entry.id; });

    if (it != m_entries.end()) {
        *it = entry;
        return true;
    }

    return false;
}

void CMockAuthDB::reset() {
    m_entries.clear();
    m_nextId = 1;
}

CTemporaryFileFixture::CTemporaryFileFixture() : m_tempDbPath("/tmp/auth_test_dir/auth_test_db.toml") {
    std::filesystem::create_directories("/tmp/auth_test_dir");
}

CTemporaryFileFixture::~CTemporaryFileFixture() {
    std::filesystem::remove_all("/tmp/auth_test_dir");
}

std::string CTemporaryFileFixture::getDbPath() const {
    return m_tempDbPath;
}