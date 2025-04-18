#include <catch2/catch_test_macros.hpp>
#include "../src/db/SecretStorage.hpp"
#include <string>

TEST_CASE("SecretStorage availability check", "[secretstorage]") {
    REQUIRE_NOTHROW(CSecretStorage::isAvailable());
}

TEST_CASE("SecretStorage construction and destruction", "[secretstorage]") {
    REQUIRE_NOTHROW([]() { CSecretStorage storage; }());
}

TEST_CASE("SecretStorage store and retrieve secret", "[secretstorage]") {
    CSecretStorage    storage;

    const std::string name   = "TestEntry";
    const uint64_t    id     = 1234;
    const std::string secret = "TestSecret123";

    std::string       secretId = storage.storeSecret(name, id, secret);
    if (CSecretStorage::isAvailable()) {
        REQUIRE(!secretId.empty());
        if (!secretId.empty()) {
            std::string retrievedSecret = storage.getSecret(secretId);
            REQUIRE(retrievedSecret == secret);
        }
    } else {
        REQUIRE(secretId == secret);
        std::string retrievedSecret = storage.getSecret(secretId);
        REQUIRE(retrievedSecret == secret);
    }
}

TEST_CASE("SecretStorage delete secret", "[secretstorage]") {
    CSecretStorage    storage;

    const std::string name   = "DeleteTest";
    const uint64_t    id     = 2345;
    const std::string secret = "SecretToDelete";

    std::string       secretId = storage.storeSecret(name, id, secret);

    bool              deleteResult = storage.deleteSecret(secretId);

    REQUIRE((deleteResult || !CSecretStorage::isAvailable()));

    if (CSecretStorage::isAvailable() && !secretId.empty()) {
        std::string retrievedSecret = storage.getSecret(secretId);
        REQUIRE(retrievedSecret.empty());
    }
}

TEST_CASE("SecretStorage update secret", "[secretstorage]") {
    CSecretStorage    storage;

    const std::string name           = "UpdateTest";
    const uint64_t    id             = 3456;
    const std::string originalSecret = "OriginalSecret";
    const std::string updatedSecret  = "UpdatedSecret";

    std::string       secretId        = storage.storeSecret(name, id, originalSecret);
    std::string       updatedSecretId = storage.updateSecret(secretId, name, id, updatedSecret);

    if (CSecretStorage::isAvailable()) {
        REQUIRE(!updatedSecretId.empty());
        if (!updatedSecretId.empty()) {
            std::string retrievedSecret = storage.getSecret(updatedSecretId);
            REQUIRE(retrievedSecret == updatedSecret);
        }
    } else
        REQUIRE(updatedSecretId == updatedSecret);
}

TEST_CASE("SecretStorage handles empty input", "[secretstorage]") {
    CSecretStorage storage;

    std::string    secretId = storage.storeSecret("", 0, "");

    if (CSecretStorage::isAvailable()) {
        if (!secretId.empty()) {
            std::string retrievedSecret = storage.getSecret(secretId);
            REQUIRE(retrievedSecret.empty());

            REQUIRE(storage.deleteSecret(secretId));
        }
    } else
        REQUIRE(secretId.empty());
}

TEST_CASE("SecretStorage handles invalid secret IDs", "[secretstorage]") {
    CSecretStorage storage;

    std::string    retrievedSecret = storage.getSecret("InvalidID");
    REQUIRE(retrievedSecret == "InvalidID");

    bool deleteResult = storage.deleteSecret("InvalidID");
    REQUIRE((!deleteResult || !CSecretStorage::isAvailable()));

    std::string updatedSecretId = storage.updateSecret("InvalidID", "Test", 1, "NewSecret");
    if (CSecretStorage::isAvailable())
        REQUIRE(!updatedSecretId.empty());
    else
        REQUIRE(updatedSecretId == "NewSecret");
}

TEST_CASE("SecretStorage handles valid SecretStorage ID format", "[secretstorage]") {
    CSecretStorage storage;

    std::string    validFormatId = "SecretStorage:TestName:4567";

    if (CSecretStorage::isAvailable()) {
        std::string retrievedSecret = storage.getSecret(validFormatId);
        REQUIRE(retrievedSecret.empty());

        bool deleteResult = storage.deleteSecret(validFormatId);
        REQUIRE(!deleteResult);
    } else {
        std::string retrievedSecret = storage.getSecret(validFormatId);
        REQUIRE(retrievedSecret == validFormatId);
    }
}