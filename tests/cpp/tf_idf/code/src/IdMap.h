#include <cstdint>
#include <optional>
#include <stdint.h>
#include <string>
#include <unordered_map>
#include <ostream>

class IdMap {
  public:
    std::optional<uint32_t> get(const std::string& key) const;
    uint32_t register_word(const std::string& key);
    uint32_t get_or_register(const std::string& key);

  private:
    uint32_t mNextId = 0;
    std::unordered_map<std::string, uint32_t> mInner;
};
