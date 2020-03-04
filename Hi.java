class Hi<T> {
    public T[] storage;

    public void mergeSort(T[] array, int lo1, int hi1, int lo2, int hi2) {
        int currentIndex = lo1;
        int i = lo1;
        int j = lo2;
        while (currentIndex <= hi2) {
            if (i == lo2) { // i > hi1
                storage[currentIndex] = array[j];
                j++;
            } else if (j > hi2) {
                storage[currentIndex] = array[j];
                i++;
            }
        }
    }
}