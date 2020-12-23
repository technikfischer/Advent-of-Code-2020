if __name__ == '__main__':
    cups = list(map(int, '364289715'))

    pickup = cups[0]
    for i in range(1, 100 + 1):
        print("\nMove", i)
        print('Cups', cups)

        pickup_index = cups.index(pickup)
        print('Current cup', pickup)

        l = len(cups)
        if pickup_index == l - 1:
            c1, c2, c3 = cups[:3]
            cups = cups[3:]
        elif pickup_index == l - 2:
            c2, c3 = cups[:2]
            c1, = cups[-1:]
            cups = cups[2:-1]
        elif pickup_index == l - 3:
            c1, c2 = cups[-2:]
            c3, = cups[:1]
            cups = cups[1:-2]
        else:
            c1, c2, c3 = cups[pickup_index + 1: pickup_index + 4]
            del cups[pickup_index + 1: pickup_index + 4]

        pickup_cups = [c1, c2, c3]

        print('pick up', pickup_cups)

        destination_cup = pickup - 1  # current cup label - 1
        if destination_cup == 0:
            destination_cup = 9

        while destination_cup in pickup_cups:
            destination_cup -= 1
            if destination_cup == 0:
                destination_cup = 9

        destination_cup_index = cups.index(destination_cup)
        destination_cup_index += 1
        print('Destination', destination_cup)
        cups[destination_cup_index:destination_cup_index] = pickup_cups

        pickup_index = cups.index(pickup)
        pickup = cups[(pickup_index + 1) % len(cups)]

    print("Final", cups)
    index_one = cups.index(1)
    part1 = ''.join([str(cups[i % len(cups)]) for i in range(index_one + 1, index_one + len(cups))])
    print("Answer to part 1 is", part1)
