#include "Db.hpp"
#include <iostream>
#include <filesystem>
#include <algorithm>

CFileAuthDB::CFileAuthDB(const std::string& path) : m_path(path) {
    const std::filesystem::path configPath = std::filesystem::path(path).parent_path();
    if (!std::filesystem::exists(configPath))
        std::filesystem::create_directories(configPath);
}

CFileAuthDB::~CFileAuthDB() {
    closeDb();
}

bool CFileAuthDB::initializeDb() {
    if (m_db)
        return true;

    int rc = sqlite3_open(m_path.c_str(), &m_db);
    if (rc != SQLITE_OK) {
        std::cerr << "Cannot open database: " << sqlite3_errmsg(m_db) << std::endl;
        closeDb();
        return false;
    }

    const char* createTableSQL = "CREATE TABLE IF NOT EXISTS auth_entries ("
                                 "id INTEGER PRIMARY KEY,"
                                 "name TEXT NOT NULL,"
                                 "secret TEXT NOT NULL,"
                                 "digits INTEGER DEFAULT 6,"
                                 "period INTEGER DEFAULT 30);";

    char*       errMsg = nullptr;
    rc                 = sqlite3_exec(m_db, createTableSQL, nullptr, nullptr, &errMsg);

    if (rc != SQLITE_OK) {
        std::cerr << "SQL error: " << errMsg << std::endl;
        sqlite3_free(errMsg);
        closeDb();
        return false;
    }

    return true;
}

void CFileAuthDB::closeDb() {
    if (m_db) {
        sqlite3_close(m_db);
        m_db = nullptr;
    }
}

bool CFileAuthDB::load() {
    if (!initializeDb())
        return false;

    m_usedIds.clear();

    const char*   sql  = "SELECT id FROM auth_entries;";
    sqlite3_stmt* stmt = nullptr;

    int           rc = sqlite3_prepare_v2(m_db, sql, -1, &stmt, nullptr);
    if (rc != SQLITE_OK) {
        std::cerr << "Failed to prepare statement: " << sqlite3_errmsg(m_db) << std::endl;
        return false;
    }

    while (sqlite3_step(stmt) == SQLITE_ROW) {
        uint64_t id = sqlite3_column_int64(stmt, 0);
        m_usedIds.push_back(id);
    }

    sqlite3_finalize(stmt);
    return true;
}

uint64_t CFileAuthDB::generateRandomId() {
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

std::vector<SAuthEntry> CFileAuthDB::getEntries() {
    std::vector<SAuthEntry> entries;

    if (!initializeDb())
        return entries;

    const char*   sql  = "SELECT id, name, secret, digits, period FROM auth_entries;";
    sqlite3_stmt* stmt = nullptr;

    int           rc = sqlite3_prepare_v2(m_db, sql, -1, &stmt, nullptr);
    if (rc != SQLITE_OK) {
        std::cerr << "Failed to prepare statement: " << sqlite3_errmsg(m_db) << std::endl;
        return entries;
    }

    while (sqlite3_step(stmt) == SQLITE_ROW) {
        SAuthEntry entry;
        entry.id = sqlite3_column_int64(stmt, 0);

        const char* name   = reinterpret_cast<const char*>(sqlite3_column_text(stmt, 1));
        const char* secret = reinterpret_cast<const char*>(sqlite3_column_text(stmt, 2));

        entry.name   = name ? name : "";
        entry.secret = secret ? secret : "";
        entry.digits = sqlite3_column_int(stmt, 3);
        entry.period = sqlite3_column_int(stmt, 4);

        entries.push_back(entry);
    }

    sqlite3_finalize(stmt);
    return entries;
}

bool CFileAuthDB::addEntry(const SAuthEntry& entry) {
    if (!initializeDb())
        return false;

    const char*   sql  = "INSERT INTO auth_entries (id, name, secret, digits, period) VALUES (?, ?, ?, ?, ?);";
    sqlite3_stmt* stmt = nullptr;

    int           rc = sqlite3_prepare_v2(m_db, sql, -1, &stmt, nullptr);
    if (rc != SQLITE_OK) {
        std::cerr << "Failed to prepare statement: " << sqlite3_errmsg(m_db) << std::endl;
        return false;
    }

    uint64_t newId = generateRandomId();

    sqlite3_bind_int64(stmt, 1, newId);
    sqlite3_bind_text(stmt, 2, entry.name.c_str(), -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 3, entry.secret.c_str(), -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 4, entry.digits);
    sqlite3_bind_int(stmt, 5, entry.period);

    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);

    return rc == SQLITE_DONE;
}

bool CFileAuthDB::removeEntry(uint64_t id) {
    if (!initializeDb())
        return false;

    const char*   sql  = "DELETE FROM auth_entries WHERE id = ?;";
    sqlite3_stmt* stmt = nullptr;

    int           rc = sqlite3_prepare_v2(m_db, sql, -1, &stmt, nullptr);
    if (rc != SQLITE_OK) {
        std::cerr << "Failed to prepare statement: " << sqlite3_errmsg(m_db) << std::endl;
        return false;
    }

    sqlite3_bind_int64(stmt, 1, id);

    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);

    if (rc == SQLITE_DONE) {
        auto it = std::ranges::find(m_usedIds, id);
        if (it != m_usedIds.end())
            m_usedIds.erase(it);
    }

    return rc == SQLITE_DONE;
}

bool CFileAuthDB::updateEntry(const SAuthEntry& entry) {
    if (!initializeDb())
        return false;

    const char*   sql  = "UPDATE auth_entries SET name = ?, secret = ?, digits = ?, period = ? WHERE id = ?;";
    sqlite3_stmt* stmt = nullptr;

    int           rc = sqlite3_prepare_v2(m_db, sql, -1, &stmt, nullptr);
    if (rc != SQLITE_OK) {
        std::cerr << "Failed to prepare statement: " << sqlite3_errmsg(m_db) << std::endl;
        return false;
    }

    sqlite3_bind_text(stmt, 1, entry.name.c_str(), -1, SQLITE_STATIC);
    sqlite3_bind_text(stmt, 2, entry.secret.c_str(), -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 3, entry.digits);
    sqlite3_bind_int(stmt, 4, entry.period);
    sqlite3_bind_int64(stmt, 5, entry.id);

    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);

    return rc == SQLITE_DONE;
}