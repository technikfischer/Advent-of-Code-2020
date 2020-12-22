from collections import deque
from functools import reduce
from operator import add
from typing import Deque, Tuple, Optional

if __name__ == '__main__':
    with open('input') as file:
        def read_cards(s):
            return list(map(int, s.splitlines()[1:]))


        player1, player2 = map(read_cards, file.read().split('\n\n'))

    # part 1
    p1, p2 = deque(player1), deque(player2)
    while p1 and p2:
        c1, c2 = p1.popleft(), p2.popleft()
        if c1 > c2:  # 9 > 5
            p1.append(c1)
            p1.append(c2)
        elif c2 > c1:
            p2.append(c2)
            p2.append(c1)
        else:
            raise ValueError('Cards have the same value')

    winner = p1 if p1 else p2
    score = reduce(add, map(lambda m: m[0] * m[1], zip(range(len(winner), 0, -1), winner)))
    print(score)


    # part 2
    def rec_combat(p1, p2) -> Tuple[int, Optional[int]]:
        played1, played2 = set(), set()
        p1: Deque[int] = deque(p1)
        p2: Deque[int] = deque(p2)

        while p1 and p2:
            # print()
            t1, t2 = tuple(p1), tuple(p2)
            # print("Deck 1:", t1, 'Deck 2:', t2)
            if t1 in played1 and t2 in played2:
                # print('Case 1', p1, p2)
                return 1, None

            played1.add(t1)
            played2.add(t2)

            c1, c2 = p1.popleft(), p2.popleft()
            # print("Player1:", c1, 'Player2:', c2)

            if len(p1) >= c1 and len(p2) >= c2:
                # print("Subgame")
                (winner, _) = rec_combat(list(p1)[:c1], list(p2)[:c2])
                # print("End subgame")
                if winner == 1:
                    p1.append(c1)
                    p1.append(c2)
                    # print("Player 1 wins")
                else:
                    p2.append(c2)
                    p2.append(c1)
                    # print("Player 2 wins")

                continue

            if c1 > c2:  # 9 > 5
                p1.append(c1)
                p1.append(c2)
                # print("Player 1 wins")
            else:
                p2.append(c2)
                p2.append(c1)
                # print("Player 2 wins")

        (w, winner) = (1, p1) if p1 else (2, p2)
        score = reduce(add, map(lambda m: m[0] * m[1], zip(range(len(winner), 0, -1), winner)))
        return w, score


    print(rec_combat(player1, player2))
