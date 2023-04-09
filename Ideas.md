# Number interpolation i.e

Using quotes (") to interpolate numbers

```umpl
(23"7"4)>       ! 2,374
(1"2".44)>      ! 12.44
(22"number-variable"4.4"3"5)> ! 2234.435
```

# Goto statement

Using the `section` keyword to define a section of code and the `jumpto` keyword to jump to that section of code.

```umpl
section start-section
! code goes here

jumpto start-section
```