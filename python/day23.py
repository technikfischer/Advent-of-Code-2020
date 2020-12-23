from tqdm import tqdm

if __name__ == '__main__':
    cups = list(map(int, '364289715')) + list(range(10, 1_000_000 + 1))
    max_cup = max(cups)

    head = None
    tail = None
    nodes = [None] * (max_cup + 1)
    for c in cups:
        node = [c, None]

        if not head:
            head = node

        if tail:
            tail[1] = node
        nodes[c] = node
        tail = node

    tail[1] = head

    current = head
    for i in tqdm(range(10_000_000)):
        c1 = current[1]
        c2 = current[1][1]
        c3 = current[1][1][1]

        # cut out values
        current[1] = c3[1]

        # find insertion point
        dest_label = current[0] - 1  # current cup label - 1
        if dest_label == 0:
            dest_label = max_cup

        while dest_label in (c1[0], c2[0], c3[0]):
            dest_label -= 1
            if dest_label == 0:
                dest_label = max_cup

        destination_node = nodes[dest_label]
        c3[1] = destination_node[1]  # successor of destination node becomes successor of c3
        destination_node[1] = c1  # c1 becomes successor of destination node

        # next cup
        current = current[1]

    index_one = cups.index(1)

    star1 = nodes[1][1][0]
    star2 = nodes[1][1][1][0]
    print(star1, star2, star1 * star2)
