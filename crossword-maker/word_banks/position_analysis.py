import os

class LetterCounter:
    def __init__(self):
        self.letter_counts = {}
        for letter in "ABCDEFGHIJKLMNOPQRSTUVWXYZ":
            self.letter_counts[letter] = 0

    def add_word(self, word, position):
        letter = word[position]
        if letter.isalpha() and self.letter_counts.get(letter) is not None and self.letter_counts[letter] == 0:
                self.letter_counts[letter] = word.strip()

    def get_letter_counts(self):
        return self.letter_counts

# Get the path to the file in the same directory as the script
input_file = os.path.join(os.path.dirname(__file__), "final_word_list.txt")

letters = []
for i in range(15):
    letters.append(LetterCounter())

with open(input_file, "r") as file:
    lines = file.readlines()
    for line in lines:
        assert len(line.strip()) == 15, f"Line does not have 15 letters: {line.strip()}"
        for i in range(15):
            letters[i].add_word(line, i)

for i in range(15):
    print(f"{i}: {letters[i].get_letter_counts()}")