#include <cstdint>
#include <iostream>
#include <string>
#include <stdint.h>
#include <sstream>
#include <unordered_map>
#include <utility>

using namespace std;
int main(int argc, char **argv) {
    if (argc < 2) {
        cerr << "hashmap <count>" << endl;
        return 1;
    }
    uint32_t count;
    stringstream ss;
    ss << argv[1];
    ss >> count;

    if(!ss) {
        cerr << "hashmap <count>" << endl;
        return 1;
    }

    unordered_map<string, uint32_t> map;
    for (uint32_t i = 0; i < count; i++) {
        string key = to_string(i);
        map.insert(make_pair(std::move(key), i));
    }

    return 0;

}



