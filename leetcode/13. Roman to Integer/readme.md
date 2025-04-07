-   If a smaller numeral comes before a larger one, it is subtracted (e.g., `IV = 4`).
-   Otherwise, the values are added (e.g., `VI = 6`).
    We can traverse the string, compare each numeral with the next, and decide whether to add or subtract.

Approach
========

1.  Create a helper function to map Roman numerals (`I`, `V`, `X`, etc.) to their integer values.
2.  Initialize a variable `result` to store the final integer value.
3.  Loop through the string:
    -   If the current numeral is smaller than the next numeral, subtract its value from `result`.
    -   Otherwise, add its value to `result`.
4.  Return the final `result`