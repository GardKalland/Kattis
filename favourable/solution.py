
def count_fav(section):
    if section in favorable_counts:
        return favorable_counts[section]

    next_sections = sections_to_next[section]
    paths_count = sum(count_fav(next_sec) for next_sec in next_sections)

    favorable_counts[section] = paths_count
    return paths_count

test_cases = int(input())
for _ in range(test_cases):
    section_count = int(input())


    sections_to_next = {}

    favorable_counts = {}

    for _ in range(section_count):
        raw_input = input().strip().split(" ", 1)
        section, outcome = raw_input[0], raw_input[1]

        if outcome in ["favourably", "catastrophically"]:
            favorable_counts[section] = 1 if outcome == "favourably" else 0
        else:
            sections_to_next[section] = outcome.split()

    count_fav("1")
    print(favorable_counts["1"])
