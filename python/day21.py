import re
from collections import defaultdict
from itertools import product
from typing import Set, Dict, Tuple, List

PATTERN = re.compile(r'(.+) \(contains (.+)\)')


def parse_line(line: str) -> (Set[str], Set[str]):
    ingredients, allergens = PATTERN.match(line).groups()
    ingredients = ingredients.split()
    allergens = allergens.split(', ')
    return set(ingredients), set(allergens)


if __name__ == '__main__':
    with open('input', 'r') as file:
        lines = file.read().strip().splitlines()
        foods = list(map(parse_line, lines))

    allergen_to_food: Dict[str, List[Tuple[Set[str], Set[str]]]] = defaultdict(list)
    for f in foods:
        for a in f[1]:
            allergen_to_food[a].append(f)

    print(allergen_to_food)

    # val for sublist in matrix for val in sublist
    clean_ingredients = [ing for food in foods for ing in food[0]]

    # check for each allergen of each ingredient if it always has that ingredient
    may_contain = defaultdict(set)
    for food in foods:
        for (ingredient, allergen) in product(*food):
            foods_containing_allergen = allergen_to_food[allergen]
            if all(ingredient in f[0] for f in foods_containing_allergen):
                print(allergen, 'may be in', ingredient)
                may_contain[allergen].add(ingredient)
                clean_ingredients = [ing for ing in clean_ingredients if ing != ingredient]

    print(len(clean_ingredients))

    print(may_contain)
    mappings = dict()
    while may_contain:
        ingredients: Set[str]
        for (allergen, ingredients) in may_contain.items():
            if len(ingredients) == 1:
                ingredient = ingredients.pop()
                mappings[ingredient] = allergen

                may_contain = {a: (ings - {ingredient}) for (a, ings) in may_contain.items()}

                del may_contain[allergen]

    part2 = list(mappings.items())
    part2.sort(key=lambda m: m[1])
    print(part2)
    part2 = ','.join(map(lambda m: m[0], part2))
    print(part2)
