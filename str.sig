#sort Str

#func '' : Str
#func 'a' : Str
#func 'Z' : Str
#func . : Str * Str -> Str
#func eq : Str * Str -> Bool

#rule s : Str | s . '' = s
#rule s : Str | '' . s = s
#rule s t u : Str | (s . t) . u = s . (t . u)
