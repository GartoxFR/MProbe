import re
import sys
from math import log2
from typing import Dict

class IdMap:
    def __init__(self) -> None:
        self.next_id: int = 0
        self.inner: Dict[str, int] = {}

    def get(self, word: str) -> int:
        return self.inner[word]

    def register(self, word: str) -> int:
        id = self.next_id
        self.next_id += 1
        self.inner[word] = id
        return id

    def contains(self, word: str) -> bool:
        return word in self.inner
        

    def get_or_register(self, word: str) -> int:
        try:
            return self.get(word)
        except KeyError:
            return self.register(word)

def parse_file(filename: str, ids: IdMap) -> Dict[int, int]:
    termCount: Dict[int, int] = {}
    with open(filename, "r") as file:
        for line in file.readlines():
            # Filtering empty strings because they are falsy
            for word in filter(lambda word: word, re.split(r"\W+", line)):
                id = ids.get_or_register(word.lower())
                termCount[id] = termCount.get(id, 0) + 1

    return termCount

def compute_score(query: str,
                  term_count: Dict[int, int], 
                  documents_term_count: Dict[int, int],
                  document_count: int,
                  ids: IdMap) -> float:
    score: float = 0.0

    total_term_count = sum(term_count.values())

    # Filtering empty strings because they are falsy
    for word in filter(lambda word: word, re.split(r"\W+", query)):
        if(not ids.contains(word.lower())):
            continue

        id = ids.get(word.lower())
        if(id not in term_count):
            continue

        tf = term_count[id] / total_term_count
        idf = log2(document_count / documents_term_count[id])
        score += tf * idf
        
    return score

if __name__ == "__main__":

    if(len(sys.argv) < 3):
        print("tf_idf <query> <FILES...>", file=sys.stderr)
        sys.exit(1)

    query = sys.argv[1]
    ids = IdMap()
    files_term_count: Dict[str, Dict[int, int]] = {}
    documents_term_count: Dict[int, int] = {}

    for filename in sys.argv[2:]: 
        term_count = parse_file(filename, ids)

        for key in term_count.keys():
            documents_term_count[key] = documents_term_count.get(key, 0) + 1

        files_term_count[filename] = term_count

    total_document_count = len(files_term_count)
    scores = map(lambda entry: (entry[0], compute_score(query, entry[1], 
                                                        documents_term_count,
                                                        total_document_count,
                                                        ids)), files_term_count.items())

    scores = sorted(scores, key=lambda score_tuple: score_tuple[1], reverse=True)

    for index, [filename, score] in zip(range(20), scores):
        print(f"{filename}: {score}")


