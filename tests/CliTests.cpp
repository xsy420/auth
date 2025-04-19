#include <catch2/catch_test_macros.hpp>
#include "helpers/TestCli.hpp"
#include <algorithm>
#include <cctype>
#include <regex>
#include <filesystem>

std::string trim(const std::string& str) {
    auto start = std::find_if_not(str.begin(), str.end(), [](unsigned char c) { return std::isspace(c); });

    auto end = std::find_if_not(str.rbegin(), str.rend(), [](unsigned char c) { return std::isspace(c); }).base();

    return (start < end) ? std::string(start, end) : std::string();
}

bool contains(const std::string& str, const std::string& substr) {
    return str.find(substr) != std::string::npos;
}

TEST_CASE("CLI help command", "[cli]") {
    CTestAuthCLI cli;

    SECTION("help command shows usage") {
        REQUIRE(cli.runCommand("help"));
        std::string output = cli.getStdout();
        REQUIRE(contains(output, "Usage:"));
        REQUIRE(contains(output, "Commands:"));
    }

    SECTION("No command shows usage") {
        REQUIRE(cli.runCommand(""));
        std::string output = cli.getStdout();
        REQUIRE(contains(output, "Usage:"));
    }
}

TEST_CASE("CLI add command", "[cli]") {
    CTestAuthCLI cli;

    SECTION("Adding a valid entry") {
        REQUIRE(cli.runCommand("add", {"TestEntry", "ABCDEFGHIJKLMN"}));
        std::string output = cli.getStdout();
        REQUIRE(contains(output, "Added new entry"));

        auto entries = cli.getMockDb()->getEntries();
        REQUIRE(entries.size() == 1);
        REQUIRE(entries[0].name == "TestEntry");
        REQUIRE(entries[0].secret == "ABCDEFGHIJKLMN");
        REQUIRE(entries[0].digits == 6);
        REQUIRE(entries[0].period == 30);
    }

    SECTION("Adding with custom digits and period") {
        REQUIRE(cli.runCommand("add", {"TestEntry2", "ABCDEFGHIJKLMN", "8", "60"}));

        auto entries = cli.getMockDb()->getEntries();
        REQUIRE(entries.size() == 1);
        REQUIRE(entries[0].digits == 8);
        REQUIRE(entries[0].period == 60);
    }

    SECTION("Invalid digits") {
        REQUIRE_FALSE(cli.runCommand("add", {"TestEntry", "ABCDEFGHIJKLMN", "9"}));
        std::string error = cli.getStderr();
        REQUIRE(contains(error, "Digits must be between 6 and 8"));
    }

    SECTION("Invalid period") {
        REQUIRE_FALSE(cli.runCommand("add", {"TestEntry", "ABCDEFGHIJKLMN", "6", "0"}));
        std::string error = cli.getStderr();
        REQUIRE(contains(error, "Period cannot be 0"));
    }

    SECTION("Invalid secret") {
        REQUIRE_FALSE(cli.runCommand("add", {"TestEntry", "ABCDEFGH!@#"}));
        std::string error = cli.getStderr();
        REQUIRE(contains(error, "Secret contains invalid characters"));
    }
}

TEST_CASE("CLI list command", "[cli]") {
    CTestAuthCLI cli;

    SECTION("Empty list") {
        REQUIRE(cli.runCommand("list"));
        std::string output = cli.getStdout();
        REQUIRE(contains(output, "No entries found"));
    }

    SECTION("List with entries") {
        cli.getMockDb()->addEntry({"Entry1", "SECRET1", 6, 30});
        cli.getMockDb()->addEntry({"Entry2", "SECRET2", 8, 60});

        REQUIRE(cli.runCommand("list"));
        std::string output = cli.getStdout();

        REQUIRE(contains(output, "Entry1"));
        REQUIRE(contains(output, "Entry2"));
    }
}

TEST_CASE("CLI remove command", "[cli]") {
    CTestAuthCLI cli;

    SECTION("Remove by name") {
        cli.getMockDb()->addEntry({"TestEntry", "SECRET", 6, 30});

        REQUIRE(cli.runCommand("remove", {"TestEntry"}));
        std::string output = cli.getStdout();
        REQUIRE(contains(output, "Removed entry"));

        auto entries = cli.getMockDb()->getEntries();
        REQUIRE(entries.empty());
    }

    SECTION("Remove by ID") {
        cli.getMockDb()->addEntry({"TestEntry", "SECRET", 6, 30});
        auto entries = cli.getMockDb()->getEntries();
        auto id      = std::to_string(entries[0].id);

        REQUIRE(cli.runCommand("remove", {id}));
        std::string output = cli.getStdout();
        REQUIRE(contains(output, "Removed entry"));

        entries = cli.getMockDb()->getEntries();
        REQUIRE(entries.empty());
    }

    SECTION("Remove non-existent entry") {
        REQUIRE_FALSE(cli.runCommand("remove", {"NonExistent"}));
        std::string error = cli.getStderr();
        REQUIRE(contains(error, "Entry not found"));
    }
}

TEST_CASE("CLI generate command", "[cli]") {
    CTestAuthCLI cli;

    SECTION("Generate code for existing entry") {
        cli.getMockDb()->addEntry({"TestEntry", "JBSWY3DPEHPK3PXP", 6, 30});

        REQUIRE(cli.runCommand("generate", {"TestEntry"}));
        std::string output = cli.getStdout();

        std::regex  codePattern("\\d{6}");
        REQUIRE(std::regex_search(output, codePattern));
    }

    SECTION("Generate with non-existent entry") {
        REQUIRE_FALSE(cli.runCommand("generate", {"NonExistent"}));
        std::string error = cli.getStderr();
        REQUIRE(contains(error, "Entry not found"));
    }
}

TEST_CASE("CLI info command", "[cli]") {
    CTestAuthCLI cli;

    SECTION("Info for existing entry") {
        cli.getMockDb()->addEntry({"TestEntry", "SECRET123", 8, 60});

        REQUIRE(cli.runCommand("info", {"TestEntry"}));
        std::string output = cli.getStdout();

        REQUIRE(contains(output, "TestEntry"));
        REQUIRE(contains(output, "SECRET123"));
        REQUIRE(contains(output, "8"));
        REQUIRE(contains(output, "60"));
    }

    SECTION("Info for non-existent entry") {
        REQUIRE_FALSE(cli.runCommand("info", {"NonExistent"}));
        std::string error = cli.getStderr();
        REQUIRE(contains(error, "Entry not found"));
    }
}

TEST_CASE("CLI edit command", "[cli]") {
    CTestAuthCLI cli;

    SECTION("Edit entry by name") {
        cli.getMockDb()->addEntry({"TestEntry", "SECRET123", 6, 30});

        REQUIRE(cli.runCommand("edit", {"TestEntry", "UpdatedEntry", "NEWSECRET", "8", "60"}));
        std::string output = cli.getStdout();
        REQUIRE(contains(output, "Updated entry"));

        auto entries = cli.getMockDb()->getEntries();
        REQUIRE(entries.size() == 1);
        REQUIRE(entries[0].name == "UpdatedEntry");
        REQUIRE(entries[0].secret == "NEWSECRET");
        REQUIRE(entries[0].digits == 8);
        REQUIRE(entries[0].period == 60);
    }

    SECTION("Edit entry by ID") {
        cli.getMockDb()->addEntry({"TestEntry", "SECRET123", 6, 30});
        auto entries = cli.getMockDb()->getEntries();
        auto id      = std::to_string(entries[0].id);

        REQUIRE(cli.runCommand("edit", {id, "UpdatedEntry", "NEWSECRET", "8", "60"}));

        entries = cli.getMockDb()->getEntries();
        REQUIRE(entries.size() == 1);
        REQUIRE(entries[0].name == "UpdatedEntry");
        REQUIRE(entries[0].secret == "NEWSECRET");
        REQUIRE(entries[0].digits == 8);
        REQUIRE(entries[0].period == 60);
    }

    SECTION("Edit only name") {
        cli.getMockDb()->addEntry({"TestEntry", "SECRET123", 6, 30});

        REQUIRE(cli.runCommand("edit", {"TestEntry", "UpdatedName"}));

        auto entries = cli.getMockDb()->getEntries();
        REQUIRE(entries.size() == 1);
        REQUIRE(entries[0].name == "UpdatedName");
        REQUIRE(entries[0].secret == "SECRET123");
        REQUIRE(entries[0].digits == 6);
        REQUIRE(entries[0].period == 30);
    }

    SECTION("Edit only secret") {
        cli.getMockDb()->addEntry({"TestEntry", "SECRET123", 6, 30});

        REQUIRE(cli.runCommand("edit", {"TestEntry", "", "NEWSECRET"}));

        auto entries = cli.getMockDb()->getEntries();
        REQUIRE(entries.size() == 1);
        REQUIRE(entries[0].name == "TestEntry");
        REQUIRE(entries[0].secret == "NEWSECRET");
        REQUIRE(entries[0].digits == 6);
        REQUIRE(entries[0].period == 30);
    }

    SECTION("Edit only digits") {
        cli.getMockDb()->addEntry({"TestEntry", "SECRET123", 6, 30});

        REQUIRE(cli.runCommand("edit", {"TestEntry", "", "", "8"}));

        auto entries = cli.getMockDb()->getEntries();
        REQUIRE(entries.size() == 1);
        REQUIRE(entries[0].name == "TestEntry");
        REQUIRE(entries[0].secret == "SECRET123");
        REQUIRE(entries[0].digits == 8);
        REQUIRE(entries[0].period == 30);
    }

    SECTION("Edit only period") {
        cli.getMockDb()->addEntry({"TestEntry", "SECRET123", 6, 30});

        REQUIRE(cli.runCommand("edit", {"TestEntry", "", "", "", "60"}));

        auto entries = cli.getMockDb()->getEntries();
        REQUIRE(entries.size() == 1);
        REQUIRE(entries[0].name == "TestEntry");
        REQUIRE(entries[0].secret == "SECRET123");
        REQUIRE(entries[0].digits == 6);
        REQUIRE(entries[0].period == 60);
    }

    SECTION("Invalid digits") {
        cli.getMockDb()->addEntry({"TestEntry", "SECRET123", 6, 30});

        REQUIRE_FALSE(cli.runCommand("edit", {"TestEntry", "", "", "9"}));
        std::string error = cli.getStderr();
        REQUIRE(contains(error, "Digits must be between 6 and 8"));

        auto entries = cli.getMockDb()->getEntries();
        REQUIRE(entries[0].digits == 6);
    }

    SECTION("Invalid period") {
        cli.getMockDb()->addEntry({"TestEntry", "SECRET123", 6, 30});

        REQUIRE_FALSE(cli.runCommand("edit", {"TestEntry", "", "", "", "0"}));
        std::string error = cli.getStderr();
        REQUIRE(contains(error, "Period cannot be 0"));

        auto entries = cli.getMockDb()->getEntries();
        REQUIRE(entries[0].period == 30);
    }

    SECTION("Invalid secret") {
        cli.getMockDb()->addEntry({"TestEntry", "SECRET123", 6, 30});

        REQUIRE_FALSE(cli.runCommand("edit", {"TestEntry", "", "INVALID!@#"}));
        std::string error = cli.getStderr();
        REQUIRE(contains(error, "Secret contains invalid characters"));

        auto entries = cli.getMockDb()->getEntries();
        REQUIRE(entries[0].secret == "SECRET123");
    }

    SECTION("Non-existent entry") {
        REQUIRE_FALSE(cli.runCommand("edit", {"NonExistent", "UpdatedName"}));
        std::string error = cli.getStderr();
        REQUIRE(contains(error, "Entry not found"));
    }
}

TEST_CASE("CLI wipe command", "[cli]") {
    CTestAuthCLI cli;

    SECTION("Wipe database") {
        cli.getMockDb()->addEntry({"Entry1", "SECRET1"});
        cli.getMockDb()->addEntry({"Entry2", "SECRET2"});

        REQUIRE(cli.getMockDb()->getEntries().size() == 2);

        REQUIRE(cli.runCommand("wipe"));

        cli.getMockDb()->reset();

        REQUIRE(cli.getMockDb()->getEntries().empty());
    }
}

TEST_CASE("CLI unknown command", "[cli]") {
    CTestAuthCLI cli;

    SECTION("Unknown command") {
        REQUIRE_FALSE(cli.runCommand("unknown"));
        std::string error = cli.getStderr();
        REQUIRE(contains(error, "Unknown command"));
    }
}

TEST_CASE("CLI import command", "[cli]") {
    CTestAuthCLI cli;

    SECTION("Import from test file") {
        REQUIRE(cli.getMockDb()->getEntries().empty());

        REQUIRE(cli.runCommand("import", {"tests/misc/TestEntries.toml"}));

        auto entries = cli.getMockDb()->getEntries();
        REQUIRE(entries.size() == 3);

        auto it1 = std::ranges::find_if(entries, [](const SAuthEntry& e) { return e.name == "Test Entry 1"; });
        REQUIRE(it1 != entries.end());
        REQUIRE(it1->secret == "JBSWY3DPEHPK3PXP");
        REQUIRE(it1->digits == 6);
        REQUIRE(it1->period == 30);

        auto it2 = std::ranges::find_if(entries, [](const SAuthEntry& e) { return e.name == "Test Entry 2"; });
        REQUIRE(it2 != entries.end());
        REQUIRE(it2->digits == 8);
        REQUIRE(it2->period == 60);
    }
}

TEST_CASE("CLI export command", "[cli]") {
    CTestAuthCLI cli;
    std::string  tempFile = "/tmp/auth_test_export.toml";

    std::filesystem::remove(tempFile);

    SECTION("Export to file") {
        cli.getMockDb()->addEntry({"Export Test 1", "ABCDEFGHIJKLMNOP", 6, 30});
        cli.getMockDb()->addEntry({"Export Test 2", "QRSTUVWXYZ234567", 8, 60});

        REQUIRE(cli.runCommand("export", {tempFile}));

        REQUIRE(std::filesystem::exists(tempFile));

        CTestAuthCLI cli2;
        REQUIRE(cli2.runCommand("import", {tempFile}));

        auto entries = cli2.getMockDb()->getEntries();
        REQUIRE(entries.size() == 2);

        auto it1 = std::ranges::find_if(entries, [](const SAuthEntry& e) { return e.name == "Export Test 1"; });
        REQUIRE(it1 != entries.end());
        REQUIRE(it1->secret == "ABCDEFGHIJKLMNOP");

        auto it2 = std::ranges::find_if(entries, [](const SAuthEntry& e) { return e.name == "Export Test 2"; });
        REQUIRE(it2 != entries.end());
        REQUIRE(it2->secret == "QRSTUVWXYZ234567");

        std::filesystem::remove(tempFile);
    }
}

TEST_CASE("CLI JSON import command", "[cli][json]") {
    CTestAuthCLI cli;

    SECTION("Import from JSON test file") {
        REQUIRE(cli.getMockDb()->getEntries().empty());

        REQUIRE(cli.runCommand("import", {"tests/misc/TestEntries.json", "json"}));

        auto entries = cli.getMockDb()->getEntries();
        REQUIRE(entries.size() == 3);

        auto it1 = std::ranges::find_if(entries, [](const SAuthEntry& e) { return e.name == "Test Entry 1"; });
        REQUIRE(it1 != entries.end());
        REQUIRE(it1->secret == "JBSWY3DPEHPK3PXP");
        REQUIRE(it1->digits == 6);
        REQUIRE(it1->period == 30);

        auto it2 = std::ranges::find_if(entries, [](const SAuthEntry& e) { return e.name == "Test Entry 2"; });
        REQUIRE(it2 != entries.end());
        REQUIRE(it2->digits == 8);
        REQUIRE(it2->period == 60);

        auto it3 = std::ranges::find_if(entries, [](const SAuthEntry& e) { return e.name == "Test Entry 3"; });
        REQUIRE(it3 != entries.end());
        REQUIRE(it3->digits == 7);
        REQUIRE(it3->period == 45);
    }

    SECTION("Import with invalid format") {
        REQUIRE_FALSE(cli.runCommand("import", {"tests/misc/TestEntries.json", "invalid"}));
        std::string error = cli.getStderr();
        REQUIRE(contains(error, "Supported formats: toml, json"));
    }
}

TEST_CASE("CLI JSON export command", "[cli][json]") {
    CTestAuthCLI cli;
    std::string  tempFile = "/tmp/auth_test_export.json";

    std::filesystem::remove(tempFile);

    SECTION("Export to JSON file") {
        cli.getMockDb()->addEntry({"Export Test 1", "ABCDEFGHIJKLMNOP", 6, 30});
        cli.getMockDb()->addEntry({"Export Test 2", "QRSTUVWXYZ234567", 8, 60});

        REQUIRE(cli.runCommand("export", {tempFile, "json"}));

        REQUIRE(std::filesystem::exists(tempFile));

        CTestAuthCLI cli2;
        REQUIRE(cli2.runCommand("import", {tempFile, "json"}));

        auto entries = cli2.getMockDb()->getEntries();
        REQUIRE(entries.size() == 2);

        auto it1 = std::ranges::find_if(entries, [](const SAuthEntry& e) { return e.name == "Export Test 1"; });
        REQUIRE(it1 != entries.end());
        REQUIRE(it1->secret == "ABCDEFGHIJKLMNOP");
        REQUIRE(it1->digits == 6);
        REQUIRE(it1->period == 30);

        auto it2 = std::ranges::find_if(entries, [](const SAuthEntry& e) { return e.name == "Export Test 2"; });
        REQUIRE(it2 != entries.end());
        REQUIRE(it2->secret == "QRSTUVWXYZ234567");
        REQUIRE(it2->digits == 8);
        REQUIRE(it2->period == 60);

        std::filesystem::remove(tempFile);
    }

    SECTION("JSON import/export roundtrip with non-default values") {
        std::string roundtripFile = "/tmp/auth_test_roundtrip.json";
        std::filesystem::remove(roundtripFile);

        cli.getMockDb()->addEntry({"Roundtrip Test 1", "ABCDEFGHIJKLMNOP", 7, 45});
        cli.getMockDb()->addEntry({"Roundtrip Test 2", "QRSTUVWXYZ234567", 8, 90});

        REQUIRE(cli.runCommand("export", {roundtripFile, "json"}));

        cli.getMockDb()->reset();
        REQUIRE(cli.getMockDb()->getEntries().empty());

        REQUIRE(cli.runCommand("import", {roundtripFile, "json"}));

        auto entries = cli.getMockDb()->getEntries();
        REQUIRE(entries.size() == 2);

        auto it1 = std::ranges::find_if(entries, [](const SAuthEntry& e) { return e.name == "Roundtrip Test 1"; });
        REQUIRE(it1 != entries.end());
        REQUIRE(it1->secret == "ABCDEFGHIJKLMNOP");
        REQUIRE(it1->digits == 7);
        REQUIRE(it1->period == 45);

        auto it2 = std::ranges::find_if(entries, [](const SAuthEntry& e) { return e.name == "Roundtrip Test 2"; });
        REQUIRE(it2 != entries.end());
        REQUIRE(it2->secret == "QRSTUVWXYZ234567");
        REQUIRE(it2->digits == 8);
        REQUIRE(it2->period == 90);

        std::filesystem::remove(roundtripFile);
    }
}