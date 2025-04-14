#include "Auth.hpp"
#include <iostream>
#include <filesystem>
#include <pwd.h>
#include <unistd.h>
#include <algorithm>
#include <iomanip>
#include <sstream>

std::string getHomeDir() {
    const char* homeDir = getenv("HOME");
    if (homeDir == nullptr)
        homeDir = getpwuid(getuid())->pw_dir;
    return homeDir ? homeDir : "";
}

CAuthCLI::CAuthCLI() {
    std::string homeDir = getHomeDir();
    if (!homeDir.empty()) {
        std::string dbPath = homeDir + "/.local/share/auth/db.toml";
        m_db               = std::make_unique<CFileAuthDB>(dbPath);

        if (!m_db->load())
            m_db->save();
    }
}

void CAuthCLI::printUsage() {
    std::cout << "Usage: auth [command] [options]\n\n";
    std::cout << "Commands:\n";
    std::cout << "  add <name> <secret> [digits] [period]  Add a new TOTP entry\n";
    std::cout << "  list                                   List all entries\n";
    std::cout << "  generate <name>                        Generate TOTP code for specific entry\n";
    std::cout << "  remove <name or id>                    Remove an entry\n";
    std::cout << "  info <name>                            Show details for an entry\n";
    std::cout << "  import <file>                          Import entries from TOML file\n";
    std::cout << "  export <file>                          Export entries to TOML file\n";
    std::cout << "  help                                   Show this help message\n";
    std::cout << "\nOptions:\n";
    std::cout << "  digits   Number of digits in the code (default: 6)\n";
    std::cout << "  period   Time period in seconds (default: 30)\n";
}

bool CAuthCLI::processCommand(int argc, char* argv[]) {
    if (argc < 2)
        return commandList();

    std::string              command = argv[1];
    std::vector<std::string> args;

    for (int i = 2; i < argc; i++) {
        args.push_back(argv[i]);
    }

    if (command == "add")
        return commandAdd(args);
    else if (command == "remove")
        return commandRemove(args);
    else if (command == "list")
        return commandList();
    else if (command == "generate")
        return commandGenerate(args);
    else if (command == "info")
        return commandInfo(args);
    else if (command == "import")
        return commandImport(args);
    else if (command == "export")
        return commandExport(args);
    else if (command == "help") {
        printUsage();
        return true;
    } else {
        std::cerr << "Unknown command: " << command << "\n";
        printUsage();
        return false;
    }
}

bool CAuthCLI::commandAdd(const std::vector<std::string>& args) {
    if (args.size() < 2) {
        std::cerr << "Error: Not enough arguments for add command\n";
        std::cerr << "Usage: auth add <name> <secret> [digits] [period]\n";
        return false;
    }

    std::string name   = args[0];
    std::string secret = args[1];
    uint32_t    digits = 6;
    uint32_t    period = 30;

    if (args.size() >= 3) {
        try {
            digits = std::stoi(args[2]);
        } catch (const std::exception& e) {
            std::cerr << "Error: Invalid digits value\n";
            return false;
        }
    }

    if (args.size() >= 4) {
        try {
            period = std::stoi(args[3]);
        } catch (const std::exception& e) {
            std::cerr << "Error: Invalid period value\n";
            return false;
        }
    }

    SAuthEntry entry;
    entry.name   = name;
    entry.secret = secret;
    entry.digits = digits;
    entry.period = period;

    if (m_db->addEntry(entry)) {
        std::cout << "Added new entry: " << name << "\n";
        return true;
    } else {
        std::cerr << "Error: Failed to add entry\n";
        return false;
    }
}

bool CAuthCLI::commandRemove(const std::vector<std::string>& args) {
    if (args.empty()) {
        std::cerr << "Error: Missing argument for remove command\n";
        std::cerr << "Usage: auth remove <name or id>\n";
        return false;
    }

    std::string nameOrId = args[0];
    auto        entries  = m_db->getEntries();

    bool        found = false;
    try {
        uint64_t id = std::stoull(nameOrId);
        if (m_db->removeEntry(id)) {
            std::cout << "Removed entry with ID: " << id << "\n";
            return true;
        }
    } catch (const std::exception&) {
        for (const auto& entry : entries) {
            if (entry.name == nameOrId) {
                if (m_db->removeEntry(entry.id)) {
                    std::cout << "Removed entry: " << nameOrId << "\n";
                    return true;
                }
                found = true;
                break;
            }
        }
    }

    if (!found)
        std::cerr << "Error: Entry not found: " << nameOrId << "\n";
    else
        std::cerr << "Error: Failed to remove entry\n";

    return false;
}

bool CAuthCLI::commandList() {
    auto entries = m_db->getEntries();

    if (entries.empty()) {
        std::cout << "No entries found\n";
        return true;
    }

    std::ranges::sort(entries, [](const SAuthEntry& a, const SAuthEntry& b) { return a.id < b.id; });

    size_t maxNameLength = 0;
    for (const auto& entry : entries) {
        maxNameLength = std::max(maxNameLength, entry.name.length());
    }

    time_t now             = time(nullptr);
    int    periodRemaining = 30 - (now % 30);

    for (const auto& entry : entries) {
        CTOTP       totp(entry.secret, entry.digits, entry.period);
        std::string code = totp.generate();

        std::cout << std::left << std::setw(5) << entry.id << std::setw(maxNameLength + 2) << entry.name << code << "\n";
    }

    std::cout << "\nExpires in " << periodRemaining << "s\n";

    return true;
}

bool CAuthCLI::commandGenerate(const std::vector<std::string>& args) {
    if (args.empty()) {
        std::cerr << "Error: Missing argument for generate command\n";
        std::cerr << "Usage: auth generate <name>\n";
        return false;
    }

    std::string name    = args[0];
    auto        entries = m_db->getEntries();

    for (const auto& entry : entries) {
        if (entry.name == name) {
            CTOTP       totp(entry.secret, entry.digits, entry.period);
            std::string code = totp.generate();

            std::cout << code << std::endl;
            return true;
        }
    }

    std::cerr << "Error: Entry not found: " << name << "\n";
    return false;
}

bool CAuthCLI::commandInfo(const std::vector<std::string>& args) {
    if (args.empty()) {
        std::cerr << "Error: Missing argument for info command\n";
        std::cerr << "Usage: auth info <name>\n";
        return false;
    }

    std::string name    = args[0];
    auto        entries = m_db->getEntries();

    for (const auto& entry : entries) {
        if (entry.name == name) {
            std::cout << "Name:   " << entry.name << "\n";
            std::cout << "ID:     " << entry.id << "\n";
            std::cout << "Secret: " << entry.secret << "\n";
            std::cout << "Digits: " << entry.digits << "\n";
            std::cout << "Period: " << entry.period << "s\n";

            CTOTP       totp(entry.secret, entry.digits, entry.period);
            std::string code = totp.generate();

            time_t      now             = time(nullptr);
            int         periodRemaining = entry.period - (now % entry.period);

            std::cout << "Code:   " << code << " (expires in " << periodRemaining << "s)\n";
            return true;
        }
    }

    std::cerr << "Error: Entry not found: " << name << "\n";
    return false;
}

bool CAuthCLI::commandImport(const std::vector<std::string>& args) {
    if (args.empty()) {
        std::cerr << "Error: Missing file path for import command\n";
        std::cerr << "Usage: auth import <file>\n";
        return false;
    }

    std::string filepath = args[0];
    if (!std::filesystem::exists(filepath)) {
        std::cerr << "Error: File not found: " << filepath << "\n";
        return false;
    }

    if (importEntriesFromToml(filepath, *m_db)) {
        std::cout << "Entries imported successfully\n";
        return true;
    } else {
        std::cerr << "Error: Failed to import entries\n";
        return false;
    }
}

bool CAuthCLI::commandExport(const std::vector<std::string>& args) {
    if (args.empty()) {
        std::cerr << "Error: Missing file path for export command\n";
        std::cerr << "Usage: auth export <file>\n";
        return false;
    }

    std::string filepath = args[0];
    auto        entries  = m_db->getEntries();

    if (exportEntriesToToml(filepath, entries)) {
        std::cout << "Entries exported to " << filepath << "\n";
        return true;
    } else {
        std::cerr << "Error: Failed to export entries\n";
        return false;
    }
}