from typing import List

# DICTIONARY = ["abc, def, ghi", "adg", "beh", "cfi"]
DICTIONARY = ["ABC", "DEF", "ZZT", "GHI", "BEH", "CFI", "ADG"]
NUM_ROWS = 3
NUM_COLS = 3

def print_grid(grid: List[str]):
    for row in grid:
        print("|", end="")
        for c in row:
            print(f"{c}|", end="")
        print("")

def print_grids(grids: List[List[str]]):
    num_dashes = 2 * NUM_COLS + 1
    for grid in grids:
        print("-" * num_dashes)
        print_grid(grid)
        print("-" * num_dashes)

class Solver:
    solution_grids: List[List[str]] = []
    
    def does_prefix_exist(self, prefix):
        # TODO using a trie or radix tree
        prefix_len = len(prefix)
        for word in DICTIONARY:
            if word[:prefix_len] == prefix:
                return True
        return False
    
    def are_cols_valid(self, cur_grid: List[str]):
        for col in range(NUM_COLS):
            prefix = ""
            for row in cur_grid:
                prefix += row[col]

            prefix_exists = self.does_prefix_exist(prefix)
            if not prefix_exists:
                return False
        return True

    def backtrack(self, cur_grid: List[str]):
        if len(cur_grid) == NUM_ROWS and self.are_cols_valid(cur_grid):
            self.solution_grids.append(cur_grid.copy())
            return
        
        for word in DICTIONARY:
            if not self.are_cols_valid(cur_grid):
                continue
            
            cur_grid.append(word)
            
            # TODO: remove word from dictionary
            self.backtrack(cur_grid)
            cur_grid.pop()
    
    
    def solve(self) -> List[List[str]]:
        grid = []
        self.backtrack(grid)
        return self.solution_grids


solver = Solver()
solution_grids = solver.solve()
print_grids(solution_grids)