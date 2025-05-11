## History
**Set** - was first formally introduced in 1879 r. by George Cantor.
## Definition
**Set** - set is collection of elements. e.g.: if $C$ is the set of all countries then a country e.g.: Poland is element of $C$. or if set $I$ is a list of integers from $1$ to $100$ then $34$ is element of $I$. The *axiom of extension* says that a set is completely determined by what its elements are and **not** the order in which they might be listed or the fact that some elements can be listed more than once.
## Notations
- **is in set** - if $S$ is a set and $x$ is element of that set we would write it: $x \in S$
- **is not in set** - if $S$ is a set and $x$ is **not** element of that set we would write it: $x \notin S$
### Set-Roster Notation
**specifying sets** - to specify a set you can write it's elements between a braces e.g.: to specify a set $S$ with elements $1, 2, 3, 4,$ we would write $S = \{1,2,3,4\}$ if a set is large or for some other reason we don't want to list all of its elements we can use $...$ (*ellipsis*) e.g.: $S = \{1,2,3,...,100\}$ which means elements of set $S$ are integers from $1$ to $100$. with the same variation of notation we can specify infinite sets e.g.: $S = \{1,2,3,...\}$ which means that elements of set are all integers from $1$ up to $\infty$. The *ellipsis* can be read "and so forth".
### Set-Builder Notation
Let $S$ denote a set and let $P(x)$ be a property that elements of $S$ may or may not satisfy. We may define a new set to be **The set of all elements $x$ in $S$ such that $P(x)$ is true** which is written as:$$\{x \in S | P(x)\}$$We can also write it without specifying from where x comes from$$\{x | P(x)\}$$Which will result with set with every element that satisfies $P(x)$We can say that the left side of $|$ is "the set of all" and $|$ is "such that"
## Special sets
Some sets are so frequently refereed to that they are given special symbolic names:
- **$\mathbb{R}$** - Set of all [[Real-numbers|real numbers]]
- **$\mathbb{Z}$** - Set of all [[Integers]]
- **$\mathbb{N}$** - Set of all [[Natural-numbers|Natural numbers]] (*set of non negative integers*)
- **$\mathbb{Q}$** - Set of all [[Rational-numbers|rational numbers]], or quotients of integers
- **$\mathbb{P}$** - Set of all [[Irrational-numbers|irrational numbers]]
- **$\mathbb{C}$** - Set of all [[Complex-numbers|complex numbers]]
Addition of superscript $+$ or $-$ or the letters $nonneg$ indicates that:
- **$+$** - Set consist only of positive elements e.g.: $\mathbb{R}^+$ would mean that we are referring to set of positive real numbers.
- **$-$** - Set consist only of negative elements e.g.: $\mathbb{R}^-$ would mean that we are referring to set of negative real numbers.
- **$nonneg$** - Set consist only of non zero elements e.g.: $\mathbb{Z}^{nonneg}$ would refer to the set of non negative integers.
# Relations
## Subset
The set $A$ is subset of $B$ if and only if every element of $A$ is also element of $B$. Which can be written: $A \subseteq B$. If there is any element x that $x \in A\ and\ x \notin B$ we can say that $A$ is not subset of $B$which is written: $A \not\subseteq B$. There are also **Proper subsets** $A$ is a proper subset of $B$ when every element of A is also element of $B$ but at least one element $x$ is $x \in B\ and\ x \not\in A$ that can be written as follows $A \subset B$ *note the lack of underline*
## Tuple & Ordered pair
To define ordered pair using sets for example $a$ and $b$ we would have to write it as $\{\{a\}, \{a, b\}\}$. and this is because in sets order doesn't matter e.g.: $\{1,2\} = \{2,1\}$ but using previous set if $a \not= b$ then $\{a\}$ and $\{a,b\}$ are two different sets which means even if the order doesn't matter the two elements are two different sets. More simply that set notation can be written as: $(a,b)$ ordered pair written in this way is named **Tuple**. $$(x_1,x_2,...,x_n) = (y_1,y_2,...,y_n) \iff x_1 = y_1, x_2 = y_2, ... , x_n = y_n$$
## Cartesian product
**Cartesian product** - is the set of all $n$ ordered n-tuples where $a\in A\ and\ b\in B$. Using the set-bulder notation: $$A\times B = \{(a,b|a\in A\ and \ b\in B)\}$$We can denote *Cartesian product* as $A\times B$. 