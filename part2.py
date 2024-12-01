from part1 import read_inputs


def initial_solution(left_list, right_list):
    freq_items = get_frequency(right_list)

    result = 0
    for left_item in left_list:
        freq = freq_items.get(left_item)
        if freq is None:
            continue
        result += left_item * freq
    return result


def get_frequency(lista):
    return {item: lista.count(item) for item in set(lista)}


def refined_solution(left_list, right_list):
    return sum([left_item * right_list.count(left_item) for left_item in left_list])


def main():
    left_list, right_list = read_inputs()
    print(initial_solution(left_list, right_list))
    print(refined_solution(left_list, right_list))


main()
