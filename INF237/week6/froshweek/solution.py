def merge_and_count_inversions(arr, left_start, middle, right_end):


    size_left = middle - left_start + 1
    size_right = right_end - middle

    left = [0] * size_left
    right = [0] * size_right


    for i in range(size_left):
        left[i] = arr[left_start + i]
    for j in range(size_right):
        right[j] = arr[middle + 1 + j]


    i = j = 0
    k = left_start
    global count

    while i < size_left and j < size_right:
        if left[i] <= right[j]:
            arr[k] = left[i]
            i += 1
        else:
            arr[k] = right[j]
            j += 1

            count += size_left - i
        k += 1


    while i < size_left:
        arr[k] = left[i]
        i += 1
        k += 1


    while j < size_right:
        arr[k] = right[j]
        j += 1
        k += 1



def merge_sort_and_count(arr, left, right):

    if left < right:

        middle = left + (right - left) // 2


        merge_sort_and_count(arr, left, middle)
        merge_sort_and_count(arr, middle + 1, right)
        merge_and_count_inversions(arr, left, middle, right)

if __name__ == "__main__":
    num = int(input())
    toBeSorted = []
    for _ in range(num):
        newNum = int(input())
        toBeSorted.append(newNum)

    count = 0
    merge_sort_and_count(toBeSorted, 0, num - 1)

    print(count)
