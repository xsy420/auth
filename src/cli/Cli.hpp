#pragma once

#include "../db/Db.hpp"
#include <memory>
#include <string>
#include <vector>

class CAuthCLI {
  public:
    CAuthCLI();
    virtual ~CAuthCLI() = default;

    bool processCommand(int argc, char* argv[]);
    void printUsage();

  protected:
    std::unique_ptr<IAuthDB> m_db;
    virtual std::string      getHomeDir() const;

  private:
    bool commandAdd(const std::vector<std::string>& args);
    bool commandRemove(const std::vector<std::string>& args);
    bool commandList();
    bool commandGenerate(const std::vector<std::string>& args);
    bool commandInfo(const std::vector<std::string>& args);
    bool commandImport(const std::vector<std::string>& args);
    bool commandExport(const std::vector<std::string>& args);
    bool commandEdit(const std::vector<std::string>& args);
    bool commandWipe();
};