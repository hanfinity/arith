/* Examples for testing */

true;
if false then true else false; 

0; 
succ (pred 0);
iszero (pred (succ (succ 0))); 
iszero 0;
pred (succ 0);
if true then succ 0 else false;
if iszero 0 then pred 0 else false;
if iszero (succ 0) then false else pred (succ 0);
