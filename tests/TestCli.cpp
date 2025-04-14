#include "auth/tests/TestCli.hpp"
#include <iostream>
#include <filesystem>

CTestAuthCLI::CTestAuthCLI() : CAuthCLI() {
    m_db = std::make_unique<CMockAuthDB>();
}

CMockAuthDB* CTestAuthCLI::getMockDb() {
    return static_cast<CMockAuthDB*>(m_db.get());
}

std::string CTestAuthCLI::getHomeDir() const {
    return "/tmp/auth_test_home";
}

bool CTestAuthCLI::runCommand(const std::string& command, const std::vector<std::string>& args) {
    std::vector<std::string> fullArgs;

    if (command.empty())
        fullArgs = {"auth"};
    else {
        fullArgs = {"auth", command};
        fullArgs.insert(fullArgs.end(), args.begin(), args.end());
    }

    std::vector<char*> cArgs;
    for (auto& arg : fullArgs) {
        cArgs.push_back(const_cast<char*>(arg.c_str()));
    }

    std::streambuf* oldCoutBuf = std::cout.rdbuf();
    std::streambuf* oldCerrBuf = std::cerr.rdbuf();

    m_stdoutCapture.str("");
    m_stderrCapture.str("");

    std::cout.rdbuf(m_stdoutCapture.rdbuf());
    std::cerr.rdbuf(m_stderrCapture.rdbuf());

    bool result = processCommand(static_cast<int>(cArgs.size()), cArgs.data());

    std::cout.rdbuf(oldCoutBuf);
    std::cerr.rdbuf(oldCerrBuf);

    return result;
}

std::string CTestAuthCLI::getStdout() const {
    return m_stdoutCapture.str();
}

std::string CTestAuthCLI::getStderr() const {
    return m_stderrCapture.str();
}

void CTestAuthCLI::resetCapture() {
    m_stdoutCapture.str("");
    m_stderrCapture.str("");
}