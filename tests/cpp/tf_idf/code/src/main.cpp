#include "IdMap.h"
#include <algorithm>
#include <cmath>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

const static string usage = "tf_idf <query> <FILES...>";

static bool isSeparator(char c) { return !isalnum(c); }

static optional<unordered_map<uint32_t, uint32_t>> parseFile(const string& filename,
                                                   IdMap& ids) {
    unordered_map<uint32_t, uint32_t> termCount;

    string buffer;
    ifstream is(filename);

    if (!is) {
        return nullopt;
    }

    while (getline(is, buffer)) {

        string::const_iterator wordBegin = buffer.cbegin();
        string::const_iterator wordEnd;
        while (wordBegin < buffer.cend()) {
            wordEnd = find_if(wordBegin, buffer.cend(), isSeparator);

            if (wordBegin == wordEnd) {
                ++wordBegin;
                continue;
            }

            string word(wordBegin, wordEnd);
            transform(word.begin(), word.end(), word.begin(), ::tolower);
            uint32_t id = ids.get_or_register(word);

            termCount[id]++;

            wordBegin = wordEnd;
            ++wordBegin;
        }

        buffer.clear();
    }

    return termCount;
}

double
computeScore(const string& query,
             const unordered_map<uint32_t, uint32_t>& termCount,
             const unordered_map<uint32_t, uint32_t>& documentsOccurences,
             const IdMap& ids, uint32_t totalDocumentCount) {

    double score = 0.0;

    uint32_t totalTermCount = 0;
    for (auto [key, value] : termCount) {
        totalTermCount += value;
    }

    string::const_iterator wordBegin = query.cbegin();
    string::const_iterator wordEnd;

    while (wordBegin < query.cend()) {
        wordEnd = find_if(wordBegin, query.cend(), isSeparator);
        if (wordBegin == wordEnd) {
            ++wordBegin;
            continue;
        }

        string word(wordBegin, wordEnd);
        transform(word.begin(), word.end(), word.begin(), ::tolower);

        uint32_t id;
        if (optional<uint32_t> optionalId = ids.get(word)) {
            id = optionalId.value();
        }

        auto it = termCount.find(id);
        if (it == termCount.end()) {
            goto next_word;
        }

        {
            double tf = (double)it->second / totalTermCount;
            double idf =
                log2((double)totalDocumentCount / documentsOccurences.at(id));

            score += tf * idf;
        }

    next_word:
        wordBegin = wordEnd;
        ++wordBegin;
    }

    return score;
}

bool scoreCmp(const pair<double, string>& a, const pair<double, string>& b) {
    return a.first > b.first;
}

int main(int argc, char** argv) {

    IdMap ids;
    unordered_map<string, unordered_map<uint32_t, uint32_t>> filesTermCount;
    unordered_map<uint32_t, uint32_t> documentsFrequencies;
    if (argc < 3) {
        cerr << usage << endl;
        return 1;
    }

    string query = argv[1];

    for (int i = 2; i < argc; i++) {
        auto termCount = parseFile(argv[i], ids);
        if (termCount) {
            for (auto& [key, value] : termCount.value()) {
                documentsFrequencies[key]++;
            }
            filesTermCount.insert({argv[i], std::move(termCount.value())});
        } else {
            cerr << "Unable to open " << argv[i] << endl;
        }
    }

    vector<pair<double, string>> scores;
    scores.reserve(filesTermCount.size());

    for (auto [filename, termCount] : filesTermCount) {
        scores.push_back({computeScore(query, termCount, documentsFrequencies,
                                       ids, filesTermCount.size()),
                          filename});
    }

    sort(scores.begin(), scores.end(), scoreCmp);

    int toShow = min(scores.size(), (size_t)20);

    for (int i = 0; i < toShow; i++) {
        const auto& pair = scores[i];
        cout << pair.second << ": " << pair.first << endl;
    }

    return 0;
}