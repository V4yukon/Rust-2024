#  Five Unsafe features
--Dereference a raw pointer  
--Call an unsafe function or method  
--Access or modify a mutable static variable  
--Implement an unsafe trait  
--Access fields of a union  



# Different from references and smart pointers, raw pointers:

..Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location  
..Aren’t guaranteed to point to valid memory  
..Are allowed to be null  
..Don’t implement any automatic cleanup  


written by  ***\*const T and \*mut T***


# Difference between constants and  immutable static variables
--- variables have fixed memory address,all referece point the same data, constants will copy the value  
--- variables can mutable: unsafe{} 
