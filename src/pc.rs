pub struct ParseError {}

pub trait ParserFn<T>: Fn(&str) -> Result<(T, &str), ParseError> {
    fn clone_box<'a>(&self) -> Box<dyn ParserFn<T> + 'a>
    where
        Self: 'a;
}

impl<T, F> ParserFn<T> for F
where
    F: Fn(&str) -> Result<(T, &str), ParseError> + Clone,
{
    fn clone_box<'a>(&self) -> Box<dyn ParserFn<T> + 'a>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl<'a, T: 'a> Clone for Box<dyn 'a + ParserFn<T>> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

pub type Parser<T> = dyn ParserFn<T>;


// String parser (generator) String -> Parser(String)
// takes as input string to match
// return parser (function) that
// tries to match the (start of) the input to the input given to parser generator
// consuming? ie: String("hello")("heppo") -> 

pub fn string(match_string: &str) -> Box<Parser<&str>> {
    Box::new(|input|
        
    )
}


// many takes input parser of A -> Parser [A]
// never errors
// matches parser against input as many times as possible

// choice takes [parser A] -> Parser A
// goes through each parser attempts to match orignal input (not whatever left over from last parser in choice)
// errors if no parsers match
// consuming?

// map (Parser A, A->B) -> Parser B
// attempts to run parser on input
// if succes than change result to B via second function
// consuming on error?

// sequnece [Parser A] -> Parser [A]
// attempts to run next parser on result of last parser
// ie (p2 (p1 input)) ..
// consuming on  error


// eof () -> Parser A
// if there is nothing left ok otherwise error

