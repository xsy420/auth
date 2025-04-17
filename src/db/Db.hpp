#pragma once

#include <cstdint>
#include <string>
#include <vector>
#include <memory>
#include <sqlite3.h>

struct SAuthEntry {
    std::string name;
    std::string secret;
    uint32_t    digits = 6;
    uint32_t    period = 30;
    uint64_t    id     = 0;
};

class IAuthDB {
  public:
    virtual ~IAuthDB() = default;

    virtual bool                    load()                               = 0;
    virtual std::vector<SAuthEntry> getEntries()                         = 0;
    virtual bool                    addEntry(const SAuthEntry& entry)    = 0;
    virtual bool                    removeEntry(uint64_t id)             = 0;
    virtual bool                    updateEntry(const SAuthEntry& entry) = 0;
};

class CFileAuthDB : public IAuthDB {
  public:
    CFileAuthDB(const std::string& path);
    ~CFileAuthDB() override;

    bool                    load() override;
    std::vector<SAuthEntry> getEntries() override;
    bool                    addEntry(const SAuthEntry& entry) override;
    bool                    removeEntry(uint64_t id) override;
    bool                    updateEntry(const SAuthEntry& entry) override;

  private:
    bool        initializeDb();
    void        closeDb();

    std::string m_path;
    sqlite3*    m_db     = nullptr;
    uint64_t    m_nextId = 1;
};