import sys
import array

if __name__ == "__main__":
    if len(sys.argv) < 2:
        exit(1)
    
    x = int(sys.argv[1])

    if x < 0:
        exit(1)

    p = array.array('B', (0 for _ in range(0, x * 1024 * 1024)))

    for i in range(0, x * 1024 * 1024):
        p[i] = 42
    
    exit(0)
