import os

class SingleLetterCounter:
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
    
class DoubleLetterCounter:
    def __init__(self):
        self.letter_counts = {}
        for letter in "ABCDEFGHIJKLMNOPQRSTUVWXYZ":
            for letter2 in "ABCDEFGHIJKLMNOPQRSTUVWXYZ":
                self.letter_counts[letter+letter2] = 0

    def add_word(self, word, first_index):
        letter_pair = word[first_index:first_index+2]
        if letter_pair.isalpha() and self.letter_counts.get(letter_pair) is not None and self.letter_counts[letter_pair] == 0:
                self.letter_counts[letter_pair] = word.strip()

    def get_letter_counts(self):
        return self.letter_counts

# Get the path to the file in the same directory as the script
input_file = os.path.join(os.path.dirname(__file__), "final_word_list.txt")

def single_letter_analysis(input_file):
    letters = []
    for i in range(15):
        letters.append(SingleLetterCounter())

    with open(input_file, "r") as file:
        lines = file.readlines()
        for line in lines:
            assert len(line.strip()) == 15, f"Line does not have 15 letters: {line.strip()}"
            for i in range(15):
                letters[i].add_word(line, i)
    
    return letters

def double_letter_analysis(input_file):
    letters = []
    for i in range(14):
        letters.append(DoubleLetterCounter())

    with open(input_file, "r") as file:
        lines = file.readlines()
        for line in lines:
            assert len(line.strip()) == 15, f"Line does not have 15 letters: {line.strip()}"
            for i in range(14):
                letters[i].add_word(line, i)
    
    return letters

letters = double_letter_analysis(input_file)
zero_count = 0
for i in range(14):
    # print(f"{i}: {letters[i].get_letter_counts()}")
    for k, v in letters[i].get_letter_counts().items():
        if v == 0:
            zero_count += 1
print(f"Total zero %: {zero_count}/{26*26*14}")