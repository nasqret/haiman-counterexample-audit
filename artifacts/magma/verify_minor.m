// Independent Magma verifier for the nonzero maximal minor in Lee's Lemma 19.
// The matrix is regenerated from the printed formulas; no Sage matrix entries
// are imported.

SetColumns(1000);
prime := 1000003;
F := GF(prime);

assignments := [
    862231,902114,920459,686557,444566,
    519439,378921,874935,467970,468016,
    584953,370016,309141,405193,969369,
    552990,606144,903829,482783,449854,
    848842,588001,920852,92590,620434,
    353822,501592,709658,69440,969953,
    150322,731728,390750,10887,336086,
    804335,385418,443899,583240,793944,
    743287,43814,574954,720916,178974
];

pivots := [
    0,1,2,3,4,5,6,7,8,9,
    10,11,12,13,14,15,16,17,18,19,
    20,21,22,23,24,25,26,27,28,29,
    30,31,32,33,34,35,36,37,38,39,
    40,41,42,43,44,45,46,47,48,49,
    50,51,52,53,54,55,56,59,60,61,
    62,63,64,65,66,67,68,69,70,73,
    74,75,76,77,78,79,80,81,82,83,
    87,88,89,90,91,92,93,94,95,101
];

function Coord(r,s,t)
    return 100*r + 10*Min(s,t) + Max(s,t);
end function;

variables := [];
for r in [1..3] do
    for s in [4..8] do
        for t in [s..8] do
            Append(~variables,Coord(r,s,t));
        end for;
    end for;
end for;
variableIndex := AssociativeArray(Integers());
for index in [1..#variables] do
    variableIndex[variables[index]] := index;
end for;

rows := [];
for a in [1..3] do
    for j in [4..8] do
        for i in [j+1..8] do
            for k in [i+1..8] do
                Append(~rows,<a,j,i,k>);
            end for;
        end for;
    end for;
    for k in [4..8] do
        for j in [k+1..8] do
            for i in [j+1..8] do
                Append(~rows,<a,j,i,k>);
            end for;
        end for;
    end for;
    for k in [5..8] do Append(~rows,<a,4,4,k>); end for;
    for j in [5..8] do Append(~rows,<a,j,4,j>); end for;
    Append(~rows,<a,5,5,6>);
    Append(~rows,<a,6,5,6>);
end for;

columns := [];
for r in [1..3] do
    for s in [1..3] do
        for t in [4..8] do
            Append(~columns,Coord(r,s,t));
        end for;
    end for;
end for;
for r in [4..8] do
    for s in [4..8] do
        for t in [s..8] do
            if not (r eq s and s eq t) then
                Append(~columns,Coord(r,s,t));
            end if;
        end for;
    end for;
end for;
columnIndex := AssociativeArray(Integers());
for index in [1..#columns] do
    columnIndex[columns[index]] := index;
end for;

assert #variables eq 45;
assert #assignments eq 45;
assert #rows eq 90;
assert #columns eq 115;
assert #Seqset(rows) eq 90;
assert #Seqset(columns) eq 115;
assert #pivots eq 90;

M := ZeroMatrix(F,90,115);
translationResidual := ZeroMatrix(Integers(),90,360);

for rowIndex in [1..90] do
    a := rows[rowIndex][1];
    j := rows[rowIndex][2];
    i := rows[rowIndex][3];
    k := rows[rowIndex][4];
    for m in [1..8] do
        terms := [
            <1,Coord(m,i,j),Coord(a,k,m)>,
            <-1,Coord(m,k,j),Coord(a,i,m)>
        ];
        for term in terms do
            sign := term[1];
            left := term[2];
            right := term[3];
            leftIsVariable := IsDefined(variableIndex,left);
            rightIsVariable := IsDefined(variableIndex,right);
            assert leftIsVariable xor rightIsVariable;
            if leftIsVariable then
                variable := variableIndex[left];
                complement := right;
            else
                variable := variableIndex[right];
                complement := left;
            end if;

            r := complement div 100;
            s := (complement mod 100) div 10;
            t := complement mod 10;
            if r eq s and s eq t then
                residualColumn := (r-1)*45 + variable;
                translationResidual[rowIndex,residualColumn] +:= 2*sign;
            else
                assert IsDefined(columnIndex,complement);
                column := columnIndex[complement];
                M[rowIndex,column] +:= F!(sign*assignments[variable]);
                if r eq t then
                    residualColumn := (s-1)*45 + variable;
                    translationResidual[rowIndex,residualColumn] +:= sign;
                end if;
                if r eq s then
                    residualColumn := (t-1)*45 + variable;
                    translationResidual[rowIndex,residualColumn] +:= sign;
                end if;
            end if;
        end for;
    end for;
end for;

assert IsZero(translationResidual);
assert #[x : x in Eltseq(M) | not IsZero(x)] eq 1410;

selectedMinor := Matrix(F,90,90,[
    M[i,pivots[j]+1] : i in [1..90], j in [1..90]
]);
determinantResidue := Determinant(selectedMinor);
assert Rank(M) eq 90;
assert determinantResidue eq F!970351;

printf "{\"implementation\":\"Magma\",\"shape\":[90,115],\"nonzero_entries\":1410,\"prime\":1000003,\"rank\":90,\"determinant_mod_prime\":970351,\"translation_residual_zero\":true,\"status\":\"verified\"}\n";
quit;
