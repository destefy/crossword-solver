import os

# Get the path to the file in the same directory as the script
filename = os.path.join(os.path.dirname(__file__), "saul_pwanson/clues.tsv")

with open(filename, "r") as file:
    lines = file.readlines()
    fifteen_letter_answers = []
    for line in lines:
        parts = line.strip().split("\t")
        if len(parts) > 2 and len(parts[2]) == 15 and parts[2].isalpha():
            fifteen_letter_answers.append(parts[2])

fifteen_letter_answers = set(fifteen_letter_answers)

output_file = os.path.join(os.path.dirname(__file__), "clues_answers.txt")
with open(output_file, "w") as output_file:
    for answer in fifteen_letter_answers:
        output_file.write(answer + "\n")