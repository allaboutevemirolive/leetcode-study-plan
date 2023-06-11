380. Insert Delete GetRandom O(1)
Medium
7.6K
389
Companies
Implement the RandomizedSet class:

RandomizedSet() Initializes the RandomizedSet object.
bool insert(int val) Inserts an item val into the set if not present. Returns true if the item was not present, false otherwise.
bool remove(int val) Removes an item val from the set if present. Returns true if the item was present, false otherwise.
int getRandom() Returns a random element from the current set of elements (it's guaranteed that at least one element exists when this method is called). Each element must have the same probability of being returned.
You must implement the functions of the class such that each function works in average O(1) time complexity.

 

Example 1:

Input
["RandomizedSet", "insert", "remove", "insert", "getRandom", "remove", "insert", "getRandom"]
[[], [1], [2], [2], [], [1], [2], []]
Output
[null, true, false, true, 2, true, false, 2]

Explanation
RandomizedSet randomizedSet = new RandomizedSet();
randomizedSet.insert(1); // Inserts 1 to the set. Returns true as 1 was inserted successfully.
randomizedSet.remove(2); // Returns false as 2 does not exist in the set.
randomizedSet.insert(2); // Inserts 2 to the set, returns true. Set now contains [1,2].
randomizedSet.getRandom(); // getRandom() should return either 1 or 2 randomly.
randomizedSet.remove(1); // Removes 1 from the set, returns true. Set now contains [2].
randomizedSet.insert(2); // 2 was already in the set, so return false.
randomizedSet.getRandom(); // Since 2 is the only number in the set, getRandom() will always return 2.
 

Constraints:

-231 <= val <= 231 - 1
At most 2 * 105 calls will be made to insert, remove, and getRandom.
There will be at least one element in the data structure when getRandom is called.

___


The given problem is about implementing a data structure called RandomizedSet. This data structure supports three operations: insert, remove, and getRandom. The goal is to implement these operations in such a way that they have an average time complexity of O(1).

Here's a simple explanation of the operations:

1. insert(val): This operation inserts an item `val` into the set if it is not already present. It returns true if the item was successfully inserted (i.e., it was not present previously), and false otherwise.

2. remove(val): This operation removes an item `val` from the set if it is present. It returns true if the item was successfully removed, and false otherwise.

3. getRandom(): This operation returns a random element from the current set of elements. It is guaranteed that at least one element exists when this method is called. Each element in the set must have an equal probability of being returned.

To implement these operations efficiently, we need to use a combination of data structures. One possible approach is to use a combination of an array and a hash map.

- We can use an array to store the elements in the set. This allows us to access elements by their indices, which will be important for the getRandom operation.

- We can use a hash map to store the mapping of elements to their indices in the array. This allows us to quickly check if an element exists in the set and retrieve its index for removal.

Here's a high-level overview of how the operations can be implemented:

1. For insert(val):
   - First, check if the element `val` already exists in the hash map. If it does, return false.
   - If `val` does not exist, append it to the end of the array and store its index in the hash map.
   - Return true.

2. For remove(val):
   - First, check if the element `val` exists in the hash map. If it does not, return false.
   - If `val` exists, retrieve its index from the hash map.
   - Swap the element at the retrieved index with the last element in the array.
   - Update the index of the last element in the hash map.
   - Remove `val` from the hash map.
   - Remove the last element from the array.
   - Return true.

3. For getRandom():
   - Generate a random index between 0 and the current size of the array.
   - Return the element at the randomly generated index.

By using this combination of array and hash map, we can achieve an average time complexity of O(1) for all three operations.