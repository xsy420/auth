#pragma once

#include <cstdint>
#include <string>
#include <vector>
#include <memory>

class CTOTP {
  public:
    CTOTP(const std::string& secret, uint32_t digits = 6, uint32_t period = 30);
    std::string generate() const;

  private:
    std::string m_secret;
    uint32_t    m_digits = 6;
    uint32_t    m_period = 30;
};

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
    virtual bool                    save()                               = 0;
    virtual std::vector<SAuthEntry> getEntries()                         = 0;
    virtual bool                    addEntry(const SAuthEntry& entry)    = 0;
    virtual bool                    removeEntry(uint64_t id)             = 0;
    virtual bool                    updateEntry(const SAuthEntry& entry) = 0;
};

class CFileAuthDB : public IAuthDB {
  public:
    CFileAuthDB(const std::string& path);

    bool                    load() override;
    bool                    save() override;
    std::vector<SAuthEntry> getEntries() override;
    bool                    addEntry(const SAuthEntry& entry) override;
    bool                    removeEntry(uint64_t id) override;
    bool                    updateEntry(const SAuthEntry& entry) override;

  private:
    std::string             m_path;
    std::vector<SAuthEntry> m_entries;
    uint64_t                m_nextId = 1;
};

bool importEntriesFromToml(const std::string& filepath, IAuthDB& db);
bool exportEntriesToToml(const std::string& filepath, const std::vector<SAuthEntry>& entries);

class CAuthCLI {
  public:
    CAuthCLI();

    bool processCommand(int argc, char* argv[]);

  private:
    void                     printUsage();
    bool                     commandAdd(const std::vector<std::string>& args);
    bool                     commandRemove(const std::vector<std::string>& args);
    bool                     commandList();
    bool                     commandGenerate(const std::vector<std::string>& args);
    bool                     commandInfo(const std::vector<std::string>& args);
    bool                     commandImport(const std::vector<std::string>& args);
    bool                     commandExport(const std::vector<std::string>& args);

    std::unique_ptr<IAuthDB> m_db;
};