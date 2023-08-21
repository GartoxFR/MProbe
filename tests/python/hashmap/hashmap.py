import sys

if __name__ == "__main__":
    if len(sys.argv) < 2:
        exit(1)
    
    x = int(sys.argv[1])

    if x < 0:
        exit(1)

    map = {}

    for i in range(0, x):
        map[str(i)] = i
    
    exit(0)
