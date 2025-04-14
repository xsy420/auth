#include "Totp.hpp"
#include <openssl/hmac.h>
#include <openssl/evp.h>
#include <ctime>
#include <iomanip>
#include <sstream>
#include <vector>
#include <cstring>
#include <stdexcept>
#include <cmath>

constexpr const char* BASE32_CHARS = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

std::vector<uint8_t>  decodeBase32(const std::string& input) {
    std::string sanitized;
    for (char c : input) {
        if (c != ' ' && c != '-')
            sanitized += std::toupper(c);
    }

    std::vector<uint8_t> result;
    size_t               buffer   = 0;
    size_t               bitsLeft = 0;

    for (char c : sanitized) {
        const char* pos = strchr(BASE32_CHARS, c);
        if (pos == nullptr)
            continue;

        size_t val = pos - BASE32_CHARS;
        buffer <<= 5;
        buffer |= val;
        bitsLeft += 5;

        if (bitsLeft >= 8) {
            bitsLeft -= 8;
            result.push_back((buffer >> bitsLeft) & 0xFF);
        }
    }

    return result;
}

CTOTP::CTOTP(const std::string& secret, uint32_t digits, uint32_t period) : m_secret(secret), m_digits(digits), m_period(period) {
    if (m_period == 0)
        m_period = 30;
}

std::string CTOTP::generate() const {
    std::vector<uint8_t> key = decodeBase32(m_secret);
    if (key.empty())
        return "Invalid key";

    time_t   currentTime = time(nullptr);
    uint64_t counter     = static_cast<uint64_t>(currentTime) / m_period;

    uint8_t  counterBytes[8] = {0};
    for (int i = 7; i >= 0; i--) {
        counterBytes[i] = counter & 0xFF;
        counter >>= 8;
    }

    uint8_t      hash[EVP_MAX_MD_SIZE];
    unsigned int hashLen = 0;

    HMAC(EVP_sha1(), key.data(), static_cast<int>(key.size()), counterBytes, sizeof(counterBytes), hash, &hashLen);

    int               offset        = hash[hashLen - 1] & 0x0F;
    uint32_t          truncatedHash = ((hash[offset] & 0x7F) << 24) | ((hash[offset + 1] & 0xFF) << 16) | ((hash[offset + 2] & 0xFF) << 8) | (hash[offset + 3] & 0xFF);

    uint32_t          totpValue = truncatedHash % static_cast<uint32_t>(std::pow(10, m_digits));

    std::stringstream ss;
    ss << std::setw(m_digits) << std::setfill('0') << totpValue;
    return ss.str();
}