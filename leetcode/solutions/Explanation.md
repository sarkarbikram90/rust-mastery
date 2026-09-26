# Solution Explanations

## Problem 1: Two Sum

- Create a hash map that stores each number already visited and its index.
- Iterate through the input numbers once, calculating the complement as `target - value`.
- Check whether the complement is already in the map. If it is, return the stored index and the current index.
- Insert the current number and its index only after checking for the complement. This lets the input `[3, 3]` match the two distinct elements instead of using one element twice.
- The algorithm runs in O(n) time and uses O(n) additional space, where n is the number of input values.
