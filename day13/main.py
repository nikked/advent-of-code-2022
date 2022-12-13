import functools
from itertools import zip_longest
from typing import Iterator


def main():

    with open("input_data.txt", "r") as f:
        data: str = f.read()

    signal_tuples = list(parse_input_to_list(data))

    # A
    valid_signal_indeces = []
    for index, signal_tuple in enumerate(signal_tuples):
        if compare_signal_tuple(*signal_tuple) == -1:
            valid_signal_indeces.append(index + 1)

    print(sum(valid_signal_indeces))

    # B
    all_signals = []
    all_signals = [[[2]], [[6]]]
    for signal_tuple in signal_tuples:
        all_signals.extend(signal_tuple)

    all_signals.sort(key=functools.cmp_to_key(compare_signal_tuple))

    distress_1 = all_signals.index([[2]]) + 1
    distress_2 = all_signals.index([[6]]) + 1

    print(distress_1 * distress_2)


def compare_signal_tuple(left, right):

    if isinstance(left, int) and isinstance(right, int):
        if left < right:
            return -1
        elif left > right:
            return 1
        return 0

    left_list = left if isinstance(left, list) else [left]
    right_list = right if isinstance(right, list) else [right]

    for left_item, right_item in zip_longest(left_list, right_list):
        if left_item is None:
            return -1
        elif right_item is None:
            return 1

        compare_result = compare_signal_tuple(left_item, right_item)
        if compare_result != 0:
            return compare_result

    return 0


def parse_input_to_list(input_data: str) -> Iterator[tuple[list, list]]:
    pairs = input_data.split("\n\n")
    for pair in pairs:
        signals = pair.split("\n")
        yield eval(signals[0]), eval(signals[1])


if __name__ == "__main__":
    main()
