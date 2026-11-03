grammar Lolcode;

// Parser Rules
lolcode
    : HAI prologue KBYE EOF
    ;

prologue
    : comment prologue
    | head body
    | variable_define body
    | body_element body_items
    |
    ;

head
    : MAEK_HEAD title MKAY
    ;

title
    : GIMMEH_TITLE text OIC
    ;

comment
    : OBTW more_text TLDR
    ;

body
    : var_defs body_items
    ;

body_items
    : body_item*
    ;

body_item
    : comment
    | body_element
    ;

body_element
    : paragraph
    | bold
    | list
    | link
    | newline
    | variable_use
    | WORD
    ;

var_defs
    : variable_define*
    ;

variable_define
    : IHAZ WORD ITIZ text MKAY
    ;

variable_use
    : GIMMEH_VAR WORD OIC
    ;

paragraph
    : MAEK_PARAGRAF var_defs inner_paragraph MKAY
    ;

inner_paragraph
    : inner_text*
    ;

inner_text
    : comment
    | bold
    | list
    | link
    | newline
    | variable_use
    | WORD
    ;

bold
    : GIMMEH_BOLD var_defs inner_bold OIC
    ;

inner_bold
    : (variable_use | WORD)*
    ;

list
    : MAEK_LIST list_items MKAY
    ;

list_items
    : list_item more_items
    ;

more_items
    : (list_item | comment)*
    ;

list_item
    : GIMMEH_ITEM var_defs inner_list OIC
    ;

inner_list
    : (bold | variable_use | WORD)*
    ;

link
    : GIMMEH_LINX WORD OIC
    ;

newline
    : NEWLINE
    ;

text
    : WORD more_text
    ;

more_text
    : WORD*
    ;

// Lexical Tokens
HAI
    : '#' H A I
    ;

KBYE
    : '#' K B Y E
    ;

OBTW
    : '#' O B T W
    ;

TLDR
    : '#' T L D R
    ;

MKAY
    : '#' M K A Y
    ;

OIC
    : '#' O I C
    ;

IHAZ
    : '#' I H A Z
    ;

ITIZ
    : '#' I T I Z
    ;

NEWLINE
    : '#' N E W L I N E
    ;

MAEK_HEAD
    : '#' M A E K SPACE H E A D
    ;

MAEK_PARAGRAF
    : '#' M A E K SPACE P A R A G R A F
    ;

MAEK_LIST
    : '#' M A E K SPACE L I S T
    ;

GIMMEH_TITLE
    : '#' G I M M E H SPACE T I T L E
    ;

GIMMEH_BOLD
    : '#' G I M M E H SPACE B O L D
    ;

GIMMEH_ITEM
    : '#' G I M M E H SPACE I T E M
    ;

GIMMEH_LINX
    : '#' G I M M E H SPACE L I N X
    ;

GIMMEH_VAR
    : '#' G I M M E H SPACE V A R
    ;

WORD
    : TEXT_CHAR+
    ;

// Whitespace handling
WS
    : (' ' | '\t' | '\r' | '\n')+ { $channel = HIDDEN; }
    ;

fragment SPACE
    : (' ' | '\t' | '\r' | '\n')+
    ;

fragment TEXT_CHAR
    : 'a'..'z' | 'A'..'Z' | '0'..'9'
    | ',' | '.' | '"' | '\'' | ':' | '?' | '!' | '%' | '=' | '/'
    ;

// Case-insensitive letters
fragment A : 'a' | 'A' ;
fragment B : 'b' | 'B' ;
fragment C : 'c' | 'C' ;
fragment D : 'd' | 'D' ;
fragment E : 'e' | 'E' ;
fragment F : 'f' | 'F' ;
fragment G : 'g' | 'G' ;
fragment H : 'h' | 'H' ;
fragment I : 'i' | 'I' ;
fragment K : 'k' | 'K' ;
fragment L : 'l' | 'L' ;
fragment M : 'm' | 'M' ;
fragment N : 'n' | 'N' ;
fragment O : 'o' | 'O' ;
fragment P : 'p' | 'P' ;
fragment R : 'r' | 'R' ;
fragment S : 's' | 'S' ;
fragment T : 't' | 'T' ;
fragment V : 'v' | 'V' ;
fragment W : 'w' | 'W' ;
fragment X : 'x' | 'X' ;
fragment Y : 'y' | 'Y' ;
fragment Z : 'z' | 'Z' ;
