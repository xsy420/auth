#pragma once

#include "auth/Cli.hpp"
#include "auth/tests/MockDb.hpp"
#include <sstream>
#include <memory>
#include <string>
#include <vector>
#include <filesystem>

class CTestEnvSetup {
  public:
    CTestEnvSetup();
    ~CTestEnvSetup();

  private:
    std::string m_origDbDir;
};

class CTestAuthCLI : public CAuthCLI {
  public:
    CTestAuthCLI();

    CMockAuthDB* getMockDb();
    bool         runCommand(const std::string& command, const std::vector<std::string>& args = {});
    std::string  getStdout() const;
    std::string  getStderr() const;
    void         resetCapture();

  private:
    std::stringstream m_stdoutCapture;
    std::stringstream m_stderrCapture;
};