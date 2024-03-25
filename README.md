# filescanner

A cheat engine like tool to scan and rescan values in files.

example usage:
```
# first scan
# find all 32bit values equal to -1000 in input.bin and save the addresses to compare.res
$ filescanner input.bin compare.res --endianness little --value-type i32 --value -1000
95 results found
95 results saved

# second scan
# find all 32bit values previously equal to -1000 and now equal to -2000 
$ filescanner input.bin output.res compare.res --endianness little --value-type i32 --value -2000
80 results found
2 results found after filter
2 results saved
```
