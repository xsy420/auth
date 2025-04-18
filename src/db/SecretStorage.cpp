#include "SecretStorage.hpp"
#include "../helpers/MiscFunctions.hpp"
#include "../helpers/Color.hpp"
#include <sstream>
#include <iostream>

#ifdef HAVE_LIBSECRET
#include <libsecret/secret.h>
#endif

const char* CSecretStorage::schemaName = "com.github.nnyyxxxx.auth.totp";

CSecretStorage::CSecretStorage() {
#ifdef HAVE_LIBSECRET
    initSchema();
#else
    m_schema = nullptr;
#endif
}

CSecretStorage::~CSecretStorage() {
#ifdef HAVE_LIBSECRET
    if (m_schema) {
        secret_schema_unref(static_cast<SecretSchema*>(m_schema));
        m_schema = nullptr;
    }
#endif
}

void CSecretStorage::initSchema() {
#ifdef HAVE_LIBSECRET
    SecretSchema* schema = secret_schema_new(schemaName, SECRET_SCHEMA_NONE, "name", SECRET_SCHEMA_ATTRIBUTE_STRING, "id", SECRET_SCHEMA_ATTRIBUTE_STRING, NULL);

    m_schema = schema;
#endif
}

std::string CSecretStorage::storeSecret(const std::string& name, uint64_t id, const std::string& secret) {
#ifdef HAVE_LIBSECRET
    if (!m_schema)
        return "";

    std::string idStr = std::to_string(id);
    GError*     error = nullptr;

    gboolean    result = secret_password_store_sync(static_cast<SecretSchema*>(m_schema), SECRET_COLLECTION_DEFAULT, (name + ":" + idStr).c_str(), secret.c_str(), NULL, &error,
                                                    "name", name.c_str(), "id", idStr.c_str(), NULL);

    if (!result) {
        if (error) {
            std::cerr << CColor::RED << "Failed to store secret: " << error->message << CColor::RESET << std::endl;
            g_error_free(error);
        }
        return "";
    }

    std::stringstream ss;
    ss << "SecretStorage:" << name << ":" << id;
    return ss.str();
#else
    return secret;
#endif
}

std::string CSecretStorage::getSecret(const std::string& secretId) {
#ifdef HAVE_LIBSECRET
    if (!m_schema || secretId.empty() || !secretId.starts_with("SecretStorage:"))
        return secretId;

    auto parts = SplitString(secretId, ":");
    if (parts.size() != 3)
        return "";

    std::string name  = parts[1];
    std::string idStr = parts[2];

    GError*     error    = nullptr;
    gchar*      password = secret_password_lookup_sync(static_cast<SecretSchema*>(m_schema), NULL, &error, "name", name.c_str(), "id", idStr.c_str(), NULL);

    if (error) {
        std::cerr << CColor::RED << "Failed to retrieve secret: " << error->message << CColor::RESET << std::endl;
        g_error_free(error);
        return "";
    }

    if (!password)
        return "";

    std::string result(password);
    secret_password_free(password);
    return result;
#else
    return secretId;
#endif
}

bool CSecretStorage::deleteSecret(const std::string& secretId) {
#ifdef HAVE_LIBSECRET
    if (!m_schema || secretId.empty() || !secretId.starts_with("SecretStorage:"))
        return false;

    auto parts = SplitString(secretId, ":");
    if (parts.size() != 3)
        return false;

    std::string name  = parts[1];
    std::string idStr = parts[2];

    GError*     error  = nullptr;
    gboolean    result = secret_password_clear_sync(static_cast<SecretSchema*>(m_schema), NULL, &error, "name", name.c_str(), "id", idStr.c_str(), NULL);

    if (error) {
        std::cerr << CColor::RED << "Failed to delete secret: " << error->message << CColor::RESET << std::endl;
        g_error_free(error);
        return false;
    }

    return result;
#else
    return true;
#endif
}

std::string CSecretStorage::updateSecret(const std::string& secretId, const std::string& name, uint64_t id, const std::string& newSecret) {
#ifdef HAVE_LIBSECRET
    if (!secretId.empty() && secretId.starts_with("SecretStorage:"))
        deleteSecret(secretId);

    return storeSecret(name, id, newSecret);
#else
    return newSecret;
#endif
}

bool CSecretStorage::isAvailable() {
#ifdef HAVE_LIBSECRET
    SecretService* service = secret_service_get_sync(SECRET_SERVICE_NONE, NULL, NULL);

    if (service) {
        g_object_unref(service);
        return true;
    }
    return false;
#else
    return false;
#endif
}