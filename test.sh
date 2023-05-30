rlox_loc="target/release/rlox"
jlox_loc="../../repos/craftinginterpreters/jlox"
j_out="jout.txt"
r_out="rout.txt"
for file in $(find ../../repos/craftinginterpreters/test/ -name "*.lox"); do
    eval "${jlox_loc} ${file} &> ${j_out}"
    eval "${rlox_loc} ${file} &> ${r_out}"
    DIFF=$(diff jout.txt rout.txt)
    if [ "$DIFF" != "" ] 
    then
	echo $file
	echo $DIFF
	eval "read -n 1"
    fi
done
eval "rm jout.txt" 
eval "rm rout.txt" 
