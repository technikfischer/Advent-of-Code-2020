from typing import List

from tqdm import tqdm


class LLNode:
    next: 'LLNode'
    val: int

    def __init__(self, val):
        self.next = None
        self.val = val


if __name__ == '__main__':
    cups = list(map(int, '364289715')) + list(range(10, 1_000_000 + 1))
    max_cup = max(cups)

    head = None
    tail = None
    nodes: List[LLNode] = [None] * (max_cup + 1)
    for c in cups:
        node = LLNode(c)

        if not head:
            head = node

        if tail:
            tail.next = node
        nodes[c] = node
        tail = node

    tail.next = head

    current: LLNode = head
    for i in tqdm(range(10_000_000)):
        c1 = current.next
        c2 = c1.next
        c3 = c2.next

        # cut out values
        current.next = c3.next

        # find insertion point
        dest_label = current.val - 1  # current cup label - 1
        if dest_label == 0:
            dest_label = max_cup

        while dest_label in (c1.val, c2.val, c3.val):
            dest_label -= 1
            if dest_label == 0:
                dest_label = max_cup

        destination_node = nodes[dest_label]
        c3.next = destination_node.next  # successor of destination node becomes successor of c3
        destination_node.next = c1  # c1 becomes successor of destination node

        # next cup
        current = current.next

    index_one = cups.index(1)

    star1 = nodes[1].next.val
    star2 = nodes[1].next.next.val
    print(star1, star2, star1 * star2)
