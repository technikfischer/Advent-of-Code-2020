M = 20201227


def transform(subject_number: int, loop_size: int) -> int:
    val = 1
    for _ in range(loop_size):
        val *= subject_number
        val %= M

    return val
    # return (subject_number ** loop_size) % M


def find_loop_size(pk: int, subject_number):
    val, loop_size = 1, 0
    while val != pk:
        loop_size += 1
        val *= subject_number
        val %= M

    return loop_size


# input
CARD = 5099500
DOOR = 7648211

d, c = find_loop_size(DOOR, 7), find_loop_size(CARD, 7)
print(d, c)

print(transform(DOOR, c), transform(CARD, d))
