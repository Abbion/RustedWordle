file = open("words.txt", "r")
lines = file.read().splitlines()
file.close()

amountOfLines = len(lines)
noDuplicates = True

for i in range(amountOfLines):
    for j in range(i + 1, amountOfLines):
        if lines[i] == lines[j]:
            print(i, " - ", j, " => ", lines[i], " - ", lines[j])
            noDuplicates = False            

if noDuplicates == True:
    print("There are no dupicates")
