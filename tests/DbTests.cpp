#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_vector.hpp>
#include "auth/Db.hpp"
#include "auth/tests/MockDb.hpp"
#include <filesystem>
#include <fstream>

TEST_CASE_METHOD(CTemporaryFileFixture, "Database can be created", "[db]") {
    CFileAuthDB db(getDbPath());
    REQUIRE_NOTHROW(db.load());
}

TEST_CASE_METHOD(CTemporaryFileFixture, "Database can add and retrieve entries", "[db]") {
    CFileAuthDB db(getDbPath());

    SAuthEntry  entry;
    entry.name   = "Test Entry";
    entry.secret = "ABCDEFGHIJKLMNOP";
    entry.digits = 6;
    entry.period = 30;

    REQUIRE(db.addEntry(entry));

    auto entries = db.getEntries();
    REQUIRE(entries.size() == 1);
    REQUIRE(entries[0].name == "Test Entry");
    REQUIRE(entries[0].secret == "ABCDEFGHIJKLMNOP");
    REQUIRE(entries[0].digits == 6);
    REQUIRE(entries[0].period == 30);
    REQUIRE(entries[0].id > 0);
}

TEST_CASE_METHOD(CTemporaryFileFixture, "Database can remove entries", "[db]") {
    CFileAuthDB db(getDbPath());

    SAuthEntry  entry;
    entry.name   = "Test Entry";
    entry.secret = "ABCDEFGHIJKLMNOP";

    REQUIRE(db.addEntry(entry));

    auto entries = db.getEntries();
    REQUIRE(entries.size() == 1);

    REQUIRE(db.removeEntry(entries[0].id));

    entries = db.getEntries();
    REQUIRE(entries.empty());
}

TEST_CASE_METHOD(CTemporaryFileFixture, "Database can update entries", "[db]") {
    CFileAuthDB db(getDbPath());

    SAuthEntry  entry;
    entry.name   = "Test Entry";
    entry.secret = "ABCDEFGHIJKLMNOP";

    REQUIRE(db.addEntry(entry));

    auto entries = db.getEntries();
    REQUIRE(entries.size() == 1);

    SAuthEntry updatedEntry = entries[0];
    updatedEntry.name       = "Updated Entry";
    updatedEntry.digits     = 8;

    REQUIRE(db.updateEntry(updatedEntry));

    entries = db.getEntries();
    REQUIRE(entries.size() == 1);
    REQUIRE(entries[0].name == "Updated Entry");
    REQUIRE(entries[0].digits == 8);
}

TEST_CASE_METHOD(CTemporaryFileFixture, "Database persists between instances", "[db]") {
    {
        CFileAuthDB db(getDbPath());

        SAuthEntry  entry;
        entry.name   = "Test Entry";
        entry.secret = "ABCDEFGHIJKLMNOP";

        REQUIRE(db.addEntry(entry));
    }

    CFileAuthDB db2(getDbPath());
    REQUIRE(db2.load());

    auto entries = db2.getEntries();
    REQUIRE(entries.size() == 1);
    REQUIRE(entries[0].name == "Test Entry");
    REQUIRE(entries[0].secret == "ABCDEFGHIJKLMNOP");
}

TEST_CASE_METHOD(CTemporaryFileFixture, "Database handles missing file", "[db]") {
    CFileAuthDB db(getDbPath());

    bool        loadResult = db.load();
    REQUIRE_FALSE(loadResult);

    auto entries = db.getEntries();
    REQUIRE(entries.empty());
}

TEST_CASE_METHOD(CTemporaryFileFixture, "Database handles corrupted file", "[db]") {
    {
        std::ofstream file(getDbPath());
        file << "This is not valid TOML syntax";
    }

    CFileAuthDB db(getDbPath());

    bool        loadResult = db.load();
    REQUIRE_FALSE(loadResult);

    auto entries = db.getEntries();
    REQUIRE(entries.empty());
}

TEST_CASE_METHOD(CTemporaryFileFixture, "Database generates incremental IDs", "[db]") {
    CFileAuthDB db(getDbPath());

    SAuthEntry  entry1;
    entry1.name   = "Entry 1";
    entry1.secret = "SECRET1";

    SAuthEntry entry2;
    entry2.name   = "Entry 2";
    entry2.secret = "SECRET2";

    SAuthEntry entry3;
    entry3.name   = "Entry 3";
    entry3.secret = "SECRET3";

    REQUIRE(db.addEntry(entry1));
    REQUIRE(db.addEntry(entry2));
    REQUIRE(db.addEntry(entry3));

    auto entries = db.getEntries();
    REQUIRE(entries.size() == 3);

    REQUIRE(entries[0].id + 1 == entries[1].id);
    REQUIRE(entries[1].id + 1 == entries[2].id);
}