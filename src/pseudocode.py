from typing import Any


class X:
    def __init__(self):
        self.string_reprentation = "X"

    def __repr__(self):
        return self.string_reprentation
    
class O:
    def __init__(self):
        self.string_reprentation = "O"

    def __repr__(self):
        return self.string_reprentation

class N:
    def __init__(self):
        self.string_reprentation = " "

    def __repr__(self):
        return self.string_reprentation

player = O

# # Al wat jy hoef te doen
# X = "X"
# O = "O"
# N = " "



tictactoe = [N, N, N, N, N, N, N, N, N]

def board_printable(copy_of_board):
    return [x().string_reprentation for x in copy_of_board]

def add_x_to_board_with_8_open(board_to_update, location_to_check):
    if board_to_update[location_to_check] == N:     # if square is open
        board_to_update[location_to_check] = X      # add X to square
        print(" >", board_printable(board_to_update))
        # copy_of_board[possible_location] = N
    else:
        pass

"""
[' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'O']
 > ['X', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'O']
 > [' ', 'X', ' ', ' ', ' ', ' ', ' ', ' ', 'O']
 > [' ', ' ', 'X', ' ', ' ', ' ', ' ', ' ', 'O']
 > ['X', 'X', 'X', 'X', ' ', ' ', ' ', ' ', 'O']
 > ['X', 'X', 'X', 'X', 'X', ' ', ' ', ' ', 'O']
 > ['X', 'X', 'X', 'X', 'X', 'X', ' ', ' ', 'O']
 > ['X', 'X', 'X', 'X', 'X', 'X', 'X', ' ', 'O']
 > ['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'O']
"""
for possible_location in range(9):
    copy_of_board = tictactoe.copy()
    copy_of_board[possible_location] = player
    print(board_printable(copy_of_board))
    for i in range(9):
        add_x_to_board_with_8_open(copy_of_board, i)



# depth = tictactoe.count(N)
# for current_depth in range(depth):
#     tictactoe[current_depth] = player
#     print(tictactoe)
