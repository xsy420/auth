#include "MockDb.hpp"
#include <algorithm>
#include <filesystem>

bool CMockAuthDB::load() {
    return true;
}

std::vector<SAuthEntry> CMockAuthDB::getEntries() {
    return m_entries;
}

uint64_t CMockAuthDB::generateRandomId() {
    if (m_usedIds.size() >= 4001) {
        if (!m_usedIds.empty()) {
            auto minId = *std::min_element(m_usedIds.begin(), m_usedIds.end());
            return minId;
        }
        return 1000;
    }

    uint64_t newId;
    do {
        newId = m_dist(m_rng);
    } while (std::ranges::find(m_usedIds, newId) != m_usedIds.end());

    m_usedIds.push_back(newId);
    return newId;
}

bool CMockAuthDB::addEntry(const SAuthEntry& entry) {
    SAuthEntry newEntry = entry;
    newEntry.id         = generateRandomId();
    m_entries.push_back(newEntry);
    return true;
}

bool CMockAuthDB::removeEntry(uint64_t id) {
    auto it = std::ranges::find_if(m_entries, [id](const SAuthEntry& entry) { return entry.id == id; });

    if (it != m_entries.end()) {
        m_entries.erase(it);

        auto idIt = std::ranges::find(m_usedIds, id);
        if (idIt != m_usedIds.end())
            m_usedIds.erase(idIt);

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
    m_usedIds.clear();
}

CTemporaryFileFixture::CTemporaryFileFixture() : m_tempDbPath("/tmp/auth_test_dir/auth_test_db.db") {
    std::filesystem::create_directories("/tmp/auth_test_dir");
}

CTemporaryFileFixture::~CTemporaryFileFixture() {
    std::filesystem::remove_all("/tmp/auth_test_dir");
}

std::string CTemporaryFileFixture::getDbPath() const {
    return m_tempDbPath;
}