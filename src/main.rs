fn main() {
    let mut arr = [0; 6];
    arr[0] = 4;
    arr[1] = 2;
    arr[2] = 0;
    arr[3] = 1;
    arr[4] = 3;
    arr[5] = 5;

    merge_sort::<6>(&mut arr);
}

fn merge_sort<const LEN: usize>(arr: &mut [i32]) -> [i32; LEN] {
    if arr.len() == 1 {
        return arr.try_into().expect("error while convert slice to an array");
    };

    let length_divided_by_two = arr.len() / 2;

    let (arr_1, arr_2) = arr.split_at_mut(length_divided_by_two);

    let arr_1_sorted = merge_sort::<LEN>(arr_1);
    let arr_2_sorted = merge_sort::<LEN>(arr_2);

    merge(&arr_1_sorted, &arr_2_sorted, arr);

    arr.try_into().expect("error while convert slice to an array")
}

fn merge(arr1: &[i32], arr2: &[i32], result: &mut [i32]){
    let arr_1_length: usize = arr1.len();
    let arr_2_length: usize = arr2.len();

    let mut indext: usize = 0;
    let mut index1: usize = 0;
    let mut index2: usize = 0;

    while index1 < arr_1_length && index2 < arr_2_length {
        if arr1[index1] > arr2[index2] {
            result[indext] = arr2[index2];
            index2 += 1;
        }else{
            result[indext] = arr1[index1];
            index1 += 1;
        }

        indext += 1;
    }

    while index1 < arr_1_length{
        result[indext] = arr1[index1];
        index1 += 1;
        indext += 1;
    }

    while index2 < arr_2_length{
        result[indext] = arr1[index2];
        index2 += 1;
        indext += 1;
    }
}

// protected void Merge(int[] arr1, int[] arr2, ref int[] result_arr)
// {
//     var arr1_lenght = arr1.Length;
//     var arr2_lenght = arr2.Length;

//     int indext = 0;
//     int index1 = 0;
//     int index2 = 0;

//     while (index1 < arr1_lenght && index2 < arr2_lenght)
//     {
//         if (arr1[index1] > arr2[index2])
//         {
//             result_arr[indext] = arr2[index2];
//             index2++;
//         }
//         else
//         {
//             result_arr[indext] = arr1[index1];
//             index1++;
//         }

//         indext++;
//     }

//     //soit arr1 soit arr2 est vide

//     while (index1 < arr1_lenght)
//     {
//         result_arr[indext] = arr1[index1];
//         index1++;
//         indext++;
//     }

//     while (index2 < arr2_lenght)
//     {
//         result_arr[indext] = arr2[index2];
//         index2++;
//         indext++;
//     }
// }

// protected int[] Sort(int[] array)
// {
//     //sortie de la recursivite
//     if (array.Length == 1) return array;

//     //etape 1 split the array
//     var lenght_divid_by_two = (int)Math.Ceiling((double)(array.Length / 2));

//     var arr1 = array.Take(new Range(0, lenght_divid_by_two)).ToArray();
//     var arr2 = array.Take(new Range(lenght_divid_by_two, array.Length)).ToArray();

//     arr1 = Sort(arr1);
//     arr2 = Sort(arr2);

//     Merge(arr1, arr2, ref array);
//     return array;
// }

