#pragma once

#include "../../src/db/Db.hpp"
#include <vector>
#include <string>

class CMockAuthDB : public IAuthDB {
  public:
    CMockAuthDB() = default;

    bool                    load() override;
    bool                    save() override;
    std::vector<SAuthEntry> getEntries() override;
    bool                    addEntry(const SAuthEntry& entry) override;
    bool                    removeEntry(uint64_t id) override;
    bool                    updateEntry(const SAuthEntry& entry) override;

    void                    reset();

  private:
    std::vector<SAuthEntry> m_entries;
    uint64_t                m_nextId = 1;
};

class CTemporaryFileFixture {
  public:
    CTemporaryFileFixture();
    ~CTemporaryFileFixture();

    std::string getDbPath() const;

  private:
    std::string m_tempDbPath;
};