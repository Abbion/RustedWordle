fileName = "words.txt"

with open(fileName, "r") as file:
    lines = file.read().splitlines()

print("processing...");

wordsSet = set(lines)

duplicatesDeleted = len(lines) - len(wordsSet)
print(duplicatesDeleted, "duplicates deleted")

if duplicatesDeleted > 0:
    with open(fileName, "w") as file:
            file.write('\n'.join(wordsSet))

print("duplicates deleted")
