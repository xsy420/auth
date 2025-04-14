#include "auth/Cli.hpp"
#include "auth/Color.hpp"
#include "auth/Totp.hpp"
#include "auth/Import.hpp"
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
    std::cout << CColor::BOLD << "Usage: " << CColor::CYAN << "auth" << CColor::RESET << " [command] [options]\n\n";
    std::cout << CColor::BOLD << "Commands:" << CColor::RESET << "\n";
    std::cout << "  " << CColor::GREEN << "add" << CColor::RESET << "      <n> <secret> [digits] [period]     Add a new TOTP entry\n";
    std::cout << "  " << CColor::GREEN << "list" << CColor::RESET << "                                        List all entries\n";
    std::cout << "  " << CColor::GREEN << "generate" << CColor::RESET << " <n>                                Generate TOTP code for specific entry\n";
    std::cout << "  " << CColor::GREEN << "remove" << CColor::RESET << "   <name or id>                       Remove an entry\n";
    std::cout << "  " << CColor::GREEN << "info" << CColor::RESET << "     <n>                                Show details for an entry\n";
    std::cout << "  " << CColor::GREEN << "import" << CColor::RESET << "   <file>                             Import entries from TOML file\n";
    std::cout << "  " << CColor::GREEN << "export" << CColor::RESET << "   <file>                             Export entries to TOML file\n";
    std::cout << "  " << CColor::GREEN << "wipe" << CColor::RESET << "                                        Wipe database\n";
    std::cout << "  " << CColor::GREEN << "help" << CColor::RESET << "                                        Show this help message\n";
    std::cout << "\n" << CColor::BOLD << "Options:" << CColor::RESET << "\n";
    std::cout << "  " << CColor::YELLOW << "digits" << CColor::RESET << "   Number of digits in the code (default: 6)\n";
    std::cout << "  " << CColor::YELLOW << "period" << CColor::RESET << "   Time period in seconds (default: 30)\n";
}

bool CAuthCLI::processCommand(int argc, char* argv[]) {
    if (argc < 2) {
        printUsage();
        return true;
    }

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
    else if (command == "wipe")
        return commandWipe();
    else if (command == "help") {
        printUsage();
        return true;
    } else {
        std::cerr << "Unknown command: " << command << "\n";
        return false;
    }
}

bool CAuthCLI::commandAdd(const std::vector<std::string>& args) {
    if (args.size() < 2) {
        std::cerr << CColor::RED << "Not enough arguments for add command" << CColor::RESET << "\n";
        std::cerr << "Usage: auth add <n> <secret> [digits] [period]\n";
        return false;
    }

    std::string name   = args[0];
    std::string secret = args[1];
    uint32_t    digits = 6;
    uint32_t    period = 30;

    if (args.size() >= 3) {
        try {
            digits = std::stoi(args[2]);
            if (digits < 6 || digits > 8) {
                std::cerr << CColor::RED << "Digits must be between 6 and 8" << CColor::RESET << "\n";
                return false;
            }
        } catch (const std::exception& e) {
            std::cerr << CColor::RED << "Invalid digits value" << CColor::RESET << "\n";
            return false;
        }
    }

    if (args.size() >= 4) {
        try {
            period = std::stoi(args[3]);
            if (period == 0) {
                std::cerr << CColor::RED << "Period cannot be 0" << CColor::RESET << "\n";
                return false;
            }
        } catch (const std::exception& e) {
            std::cerr << CColor::RED << "Invalid period value" << CColor::RESET << "\n";
            return false;
        }
    }

    for (char c : secret) {
        if (c != ' ' && c != '-' && !std::isalnum(c)) {
            std::cerr << CColor::RED << "Secret contains invalid characters" << CColor::RESET << "\n";
            return false;
        }
    }

    SAuthEntry entry;
    entry.name   = name;
    entry.secret = secret;
    entry.digits = digits;
    entry.period = period;

    if (m_db->addEntry(entry)) {
        std::cout << CColor::GREEN << "Added new entry: " << name << CColor::RESET << "\n";
        return true;
    } else {
        std::cerr << CColor::RED << "Failed to add entry" << CColor::RESET << "\n";
        return false;
    }
}

bool CAuthCLI::commandRemove(const std::vector<std::string>& args) {
    if (args.empty()) {
        std::cerr << CColor::RED << "Missing argument for remove command" << CColor::RESET << "\n";
        std::cerr << "Usage: auth remove <name or id>\n";
        return false;
    }

    std::string nameOrId = args[0];
    auto        entries  = m_db->getEntries();

    bool        found = false;
    try {
        uint64_t id = std::stoull(nameOrId);
        if (m_db->removeEntry(id)) {
            std::cout << CColor::GREEN << "Removed entry with ID: " << id << CColor::RESET << "\n";
            return true;
        }
    } catch (const std::exception&) {
        for (const auto& entry : entries) {
            if (entry.name == nameOrId) {
                if (m_db->removeEntry(entry.id)) {
                    std::cout << CColor::GREEN << "Removed entry: " << nameOrId << CColor::RESET << "\n";
                    return true;
                }
                found = true;
                break;
            }
        }
    }

    if (!found)
        std::cerr << CColor::RED << "Entry not found: " << nameOrId << CColor::RESET << "\n";
    else
        std::cerr << CColor::RED << "Failed to remove entry" << CColor::RESET << "\n";

    return false;
}

bool CAuthCLI::commandList() {
    auto entries = m_db->getEntries();

    if (entries.empty()) {
        std::cout << CColor::YELLOW << "No entries found" << CColor::RESET << "\n";
        return true;
    }

    std::ranges::sort(entries, [](const SAuthEntry& a, const SAuthEntry& b) { return a.id < b.id; });

    size_t maxNameLength = 0;
    for (const auto& entry : entries) {
        maxNameLength = std::max(maxNameLength, entry.name.length());
    }

    time_t now             = time(nullptr);
    int    periodRemaining = 0;
    if (!entries.empty() && entries[0].period > 0)
        periodRemaining = entries[0].period - (now % entries[0].period);

    std::cout << CColor::BOLD << std::left << std::setw(5) << "ID" << std::setw(maxNameLength + 2) << "NAME" << "CODE" << CColor::RESET << "\n";

    std::cout << std::string(5 + maxNameLength + 8, '-') << "\n";

    for (const auto& entry : entries) {
        CTOTP       totp(entry.secret, entry.digits, entry.period);
        std::string code = totp.generate();

        std::cout << CColor::CYAN << std::left << std::setw(5) << entry.id << CColor::RESET << CColor::GREEN << std::setw(maxNameLength + 2) << entry.name << CColor::RESET
                  << CColor::BOLD << CColor::YELLOW << code << CColor::RESET << "\n";
    }

    std::cout << "\n" << CColor::MAGENTA << "Expires in " << periodRemaining << "s" << CColor::RESET << "\n";

    return true;
}

bool CAuthCLI::commandGenerate(const std::vector<std::string>& args) {
    if (args.empty()) {
        std::cerr << CColor::RED << "Missing argument for generate command" << CColor::RESET << "\n";
        std::cerr << "Usage: auth generate <n>\n";
        return false;
    }

    std::string name    = args[0];
    auto        entries = m_db->getEntries();

    for (const auto& entry : entries) {
        if (entry.name == name) {
            CTOTP       totp(entry.secret, entry.digits, entry.period);
            std::string code = totp.generate();

            std::cout << CColor::YELLOW << code << CColor::RESET << std::endl;
            return true;
        }
    }

    std::cerr << CColor::RED << "Entry not found: " << name << CColor::RESET << "\n";
    return false;
}

bool CAuthCLI::commandInfo(const std::vector<std::string>& args) {
    if (args.empty()) {
        std::cerr << CColor::RED << "Missing argument for info command" << CColor::RESET << "\n";
        std::cerr << "Usage: auth info <n>\n";
        return false;
    }

    std::string name    = args[0];
    auto        entries = m_db->getEntries();

    for (const auto& entry : entries) {
        if (entry.name == name) {
            std::cout << CColor::BOLD << "Name:   " << CColor::RESET << CColor::GREEN << entry.name << CColor::RESET << "\n";
            std::cout << CColor::BOLD << "ID:     " << CColor::RESET << CColor::CYAN << entry.id << CColor::RESET << "\n";
            std::cout << CColor::BOLD << "Secret: " << CColor::RESET << entry.secret << "\n";
            std::cout << CColor::BOLD << "Digits: " << CColor::RESET << entry.digits << "\n";
            std::cout << CColor::BOLD << "Period: " << CColor::RESET << entry.period << "s\n";

            CTOTP       totp(entry.secret, entry.digits, entry.period);
            std::string code = totp.generate();

            time_t      now             = time(nullptr);
            int         periodRemaining = entry.period - (now % entry.period);

            std::cout << CColor::BOLD << "Code:   " << CColor::RESET << CColor::YELLOW << code << CColor::RESET << " (expires in " << CColor::MAGENTA << periodRemaining << "s"
                      << CColor::RESET << ")\n";
            return true;
        }
    }

    std::cerr << CColor::RED << "Entry not found: " << name << CColor::RESET << "\n";
    return false;
}

bool CAuthCLI::commandImport(const std::vector<std::string>& args) {
    if (args.empty()) {
        std::cerr << CColor::RED << "Missing argument for import command" << CColor::RESET << "\n";
        std::cerr << "Usage: auth import <file>\n";
        return false;
    }

    std::string filepath = args[0];
    if (importEntriesFromToml(filepath, *m_db)) {
        std::cout << CColor::GREEN << "Successfully imported entries from " << filepath << CColor::RESET << "\n";
        return true;
    } else {
        std::cerr << CColor::RED << "Failed to import entries from " << filepath << CColor::RESET << "\n";
        return false;
    }
}

bool CAuthCLI::commandExport(const std::vector<std::string>& args) {
    if (args.empty()) {
        std::cerr << CColor::RED << "Missing argument for export command" << CColor::RESET << "\n";
        std::cerr << "Usage: auth export <file>\n";
        return false;
    }

    std::string filepath = args[0];
    auto        entries  = m_db->getEntries();

    if (entries.empty()) {
        std::cerr << CColor::RED << "No entries to export" << CColor::RESET << "\n";
        return false;
    }

    if (exportEntriesToToml(filepath, entries)) {
        std::cout << CColor::GREEN << "Successfully exported " << entries.size() << " entries to " << filepath << CColor::RESET << "\n";
        return true;
    } else {
        std::cerr << CColor::RED << "Failed to export entries to " << filepath << CColor::RESET << "\n";
        return false;
    }
}

bool CAuthCLI::commandWipe() {
    std::string homeDir = getHomeDir();
    if (homeDir.empty()) {
        std::cerr << CColor::RED << "Could not find home directory" << CColor::RESET << "\n";
        return false;
    }

    std::string dbPath = homeDir + "/.local/share/auth/db.toml";

    try {
        std::filesystem::remove(dbPath);
        std::cout << CColor::GREEN << "Database wiped successfully" << CColor::RESET << "\n";
        return true;
    } catch (const std::exception& e) {
        std::cerr << CColor::RED << "Error wiping database: " << e.what() << CColor::RESET << "\n";
        return false;
    }
}