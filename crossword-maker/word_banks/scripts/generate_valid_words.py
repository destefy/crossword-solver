filename = "../fifteen_letter/test.txt"
with open(filename, "r") as file:
    lines = file.readlines()
    lines = [line.strip() for line in lines if line.strip()]
    new_words = []
    word_len = len(lines[0])
    print("Word length:", word_len)
    for col in range(word_len):
        new_word = ""
        for row_word in lines:
            new_word += (row_word[col])
        new_words.append(new_word)

for word in new_words:
    print(word)