#pragma once

#include "../../src/db/Db.hpp"
#include <vector>
#include <string>
#include <random>

class CMockAuthDB : public IAuthDB {
  public:
    CMockAuthDB() = default;

    bool                    load() override;
    std::vector<SAuthEntry> getEntries() override;
    bool                    addEntry(const SAuthEntry& entry) override;
    bool                    removeEntry(uint64_t id) override;
    bool                    updateEntry(const SAuthEntry& entry) override;

    void                    reset();

  private:
    uint64_t                                generateRandomId();

    std::vector<SAuthEntry>                 m_entries;
    std::mt19937_64                         m_rng{std::random_device{}()};
    std::uniform_int_distribution<uint64_t> m_dist{1000, 5000};
    std::vector<uint64_t>                   m_usedIds;
};

class CTemporaryFileFixture {
  public:
    CTemporaryFileFixture();
    ~CTemporaryFileFixture();

    std::string getDbPath() const;

  private:
    std::string m_tempDbPath;
};