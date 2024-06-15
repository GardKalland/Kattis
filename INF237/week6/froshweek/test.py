
def merge(arr, start, mid, end):
    n1 = mid - start + 1
    n2 = end - mid

    temp_left = [arr[start + i] for i in range(n1)]
    temp_right = [arr[mid + 1 + j] for j in range(n2)]

    inversion_count = 0
    i = j = 0
    k = start

    while i < n1 and j < n2:
        if temp_left[i] <= temp_right[j]:
            arr[k] = temp_left[i]
            i += 1
        else:
            arr[k] = temp_right[j]
            j += 1
            inversion_count += n1 - i
        k += 1

    arr[k:k + n1 - i] = temp_left[i:n1]
    arr[k + n1 - i:] = temp_right[j:n2]

    return inversion_count

def sort(arr, start, end):
    total_inversions = 0
    if start < end:
        middle = (start + end) // 2

        total_inversions += sort(arr, start, middle)
        total_inversions += sort(arr, middle + 1, end)
        total_inversions += merge(arr, start, middle, end)

    return total_inversions

if __name__ == "__main__":
    num = int(input("Enter number of elements: "))
    array_to_sort = [int(input("Enter number: ")) for _ in range(num)]

    inversions = sort(array_to_sort, 0, len(array_to_sort) - 1)

    print(inversions)
