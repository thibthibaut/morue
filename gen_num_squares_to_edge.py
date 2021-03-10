


#             N   S   W  E  NW SE  NE  SW
dirOffsets = [8, -8, -1, 1, 7, -7, 9, -9]


for square in range(64):
        fil = square % 8
        rank = square // 8
        n = 7 - rank
        s = rank
        w = fil
        e = 7 - fil
        nw = min(n, w)
        se = min(s, e)
        sw = min(s, w)
        ne = min(n, e)
        print(f'(({square}))[{n}, {s}, {w}, {e}, {nw}, {se}, {sw}, {ne}]')


# let directions: [[i32; 2]; 4] = [[-1, 0], [0, 1], [0, 1], [1, 0]]
