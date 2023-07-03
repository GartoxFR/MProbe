#include <iostream>
#include <string>
#include <stdint.h>
#include <sstream>

using namespace std;
int main(int argc, char **argv) {
    if (argc < 3) {
        cerr << "axwy <alloc MB> <write MB>" << endl;
        return 1;
    }
    size_t alloc;
    size_t write;

    int r = sscanf(argv[1], "%ld", &alloc);
    if (r == 0 || r == EOF) {
        cerr << "axwy <alloc MB> <write MB>" << endl;
        return 1;
    }
    stringstream ss;

    ss << argv[1] << " " << argv[2];
    ss >> alloc;
    ss >> write;

    if(!ss) {
        cerr << "axwy <alloc MB> <write MB>\n" << endl;
        return 1;
    }

    if (write > alloc) {
        cerr << "Error:  write should be less or equal than alloc" << endl;
        return 1;
    }

    alloc *= 1024 * 1024;
    write *= 1024 * 1024;

    // volatile here to prevent compiler optimization for benchmark purposes
    volatile uint8_t* arr = new uint8_t[alloc];
    if (arr == NULL) {
        cerr << "Error:  Failed to malloc" << endl;
        return 1;
    }
    for (int i = 0; i < write ; i++) {
        arr[i] = 42;
    }

    delete [] arr;

    return 0;

}



