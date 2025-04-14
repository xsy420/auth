#include <catch2/catch_test_macros.hpp>
#include "auth/Totp.hpp"
#include <string>

TEST_CASE("TOTP constructor sets correct default values", "[totp]") {
    CTOTP totp("ABCDEFGHIJKLMNOP");

    REQUIRE_NOTHROW(totp.generate());
}

TEST_CASE("TOTP constructor with custom parameters", "[totp]") {
    CTOTP totp("ABCDEFGHIJKLMNOP", 8, 60);
    REQUIRE_NOTHROW(totp.generate());
}

TEST_CASE("TOTP handles empty secret", "[totp]") {
    CTOTP totp("");
    REQUIRE(totp.generate() == "Invalid key");
}

TEST_CASE("TOTP handles invalid Base32 characters in secret", "[totp]") {
    CTOTP totp("!@#$%^&*()");
    REQUIRE(totp.generate() == "Invalid key");
}

TEST_CASE("TOTP handles valid Base32 secret", "[totp]") {
    CTOTP             totp("JBSWY3DPEHPK3PXP");
    const std::string code = totp.generate();

    REQUIRE(code.length() == 6);
    REQUIRE(std::all_of(code.begin(), code.end(), [](char c) { return std::isdigit(c); }));
}

TEST_CASE("TOTP with 8 digit output", "[totp]") {
    CTOTP             totp("JBSWY3DPEHPK3PXP", 8);
    const std::string code = totp.generate();

    REQUIRE(code.length() == 8);
    REQUIRE(std::all_of(code.begin(), code.end(), [](char c) { return std::isdigit(c); }));
}

TEST_CASE("TOTP handles spaces and dashes in secret", "[totp]") {
    CTOTP totp1("JBSWY3DPEHPK3PXP");
    CTOTP totp2("JBSW Y3DP-EHPK 3PXP");

    REQUIRE(totp1.generate() == totp2.generate());
}