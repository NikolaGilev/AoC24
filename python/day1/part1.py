def main():
    left_list, right_list = read_inputs()
    result = sum(
        [distance(l, r) for l, r in zip(sorted(left_list), sorted(right_list))]
    )
    print(result)


def distance(a, b):
    return abs(a - b)


def read_inputs():
    input_file = "input.txt"
    left_list, right_list = [], []
    with open(input_file) as input_content:
        content = input_content.read()
        lines = content.split("\n")
        for line in lines:
            split_items = line.split("   ")
            left_list.append(int(split_items[0]))
            right_list.append(int(split_items[1]))

    if len(left_list) != len(right_list):
        print("The lists aren't the same length")
        exit

    return left_list, right_list


main()
