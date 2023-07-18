#include "IdMap.h"
#include <cstdint>
#include <optional>

using namespace std;

optional<uint32_t> IdMap::get(const string &key) const {
    auto iter = mInner.find(key);

    if (iter != mInner.end()) {
        return iter->second;
    }

    return {};
}

uint32_t IdMap::register_word(const string &key) {
    uint32_t id = mNextId;
    mNextId++;

    mInner.insert({key, id});
    return id;
}

uint32_t IdMap::get_or_register(const string &key) {
    if (auto id = get(key)) {
        return id.value();
    }

    return register_word(key);
}
