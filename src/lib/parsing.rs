#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Token {
    LCurlyBraces, // {
    RCurlyBraces, // }
    Div,          // div
    Html,         //html
    Head,         //head
    Title,        //title
    Body,         //body
    H1,           //h1
    H2,           //h2
    H3,           //h3
    P,            //p
    Id,           //Id
    RDiv,         // div
    RHtml,        //html
    RHead,        //head
    RTitle,       //title
    RBody,        //body
    RH1,          //h1
    RH2,          //h2
    RH3,          //h3
    RP,           //p
    RA,
    None, //None
}

pub enum CloseTag {
    None,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TokenType {
    Tag,
    Text,
    Other,
    None,
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tokens {
    pub Type: TokenType,
    pub Tag: Token,
    pub Value: Option<String>,
}

pub struct Lexer {
    input: String,
    read_position: usize,
    position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut tmp = Lexer {
            input: input.to_owned(),
            read_position: 0,
            position: 0,
            ch: 0,
        };
        tmp.read_char();
        tmp
    }

    pub fn convert(input: &mut Lexer) -> String {
        let mut result = Vec::new();
        let mut count = 0;

        loop {
            let toki = input.next_token();
            //println!("{:?}",toki);
            result.push(toki);
            let tmp3 = result.last().unwrap();

            if tmp3.Type == TokenType::None {
                if count > 2 {
                    break;
                }

                count += 1;
            };

            //if count > 10 {break};
            //count+=1;
        }

        let result_count = result.len();
        //let mut tmp = Vec::new();
        let mut tmp2 = Vec::new();

        let mut tmp3 = Token::None;

        for i in 0..result_count {
            //println!("{:?}", result[i]);

            //println!("{:?}", [i]);
            let mut ctagcount = 0;
            let mut stagcount = 0;
            //tmp.push(&result[i]);

            if result[i].Type == TokenType::Tag {
                //tmp3 = result[i].Tag;

                tmp3 = match result[i].Tag {
                    Token::Div => Token::RDiv,

                    Token::Html => Token::RHtml,

                    _ => Token::None,
                };
            }

            for j in (i + 1)..result.len() {
                if result[i].Type == TokenType::Tag {
                    if result[j].Tag == Token::RCurlyBraces {
                        if stagcount == 0 {
                            result[j].Tag = tmp3;

                            println!("{:?}", result[j]);
                            break;
                        }
                        stagcount -= 1;
                    }

                    if result[j].Type == TokenType::Tag {
                        stagcount += 1;
                        ctagcount += 1;
                    }
                }
            }

            //println!("{}", ctagcount);
            tmp2.push(ctagcount);
        }

        //println!("{}","-".repeat(60));

        println!("{}", tmp2.len());
        println!("{}", result.len());

        let mut html = String::new();

        for j in 0..result.len() {
            html += match result[j].Tag {
                Token::Html => "<html>",

                Token::Div => "<div>",

                Token::RCurlyBraces => "",

                Token::RDiv => "</div>",
                Token::RHtml => "</html>",

                

                _ => "",
            };

            html += match result[j].Type {
                TokenType::Text => 
                {
                    result[j].Value.as_ref().unwrap().as_str()
                    
                },

                _ => "",
            };
        }

        html
    }

    pub fn next_token(&mut self) -> Tokens {
        let mut flag = true;
        while flag {
            let c = char::from(self.ch);
            //println!("{}", c.is_whitespace());

            if c.is_whitespace() {
                self.read_char();
            } else {
                flag = false;
            }
        }

        let token = match self.ch {
            b'{' => {
                self.read_char();
                Tokens {
                    Type: TokenType::Other,
                    Tag: Token::LCurlyBraces,
                    Value: None,
                }
            }

            b'}' => {
                self.read_char();
                Tokens {
                    Type: TokenType::Other,
                    Tag: Token::RCurlyBraces,
                    Value: None,
                }
            }

            _ => {
                if self.ch > 0 {
                    if is_letter(&self.ch) {
                        let literal = self.read_identifier();
                        //println!("{}", literal);
                        let t = match literal.as_str() {
                            "div" => {
                                if self.peek_char() == '{' {
                                    self.read_char();
                                    Tokens {
                                        Type: TokenType::Tag,
                                        Tag: Token::Div,
                                        Value: None,
                                    }
                                } else {
                                    self.read_char();
                                    Tokens {
                                        Type: TokenType::Text,
                                        Tag: Token::None,
                                        Value: Some(String::from("div")),
                                    }
                                }
                            },
                            "html" => {
                                if self.peek_char() == '{' {
                                    self.read_char();
                                    Tokens {
                                        Type: TokenType::Tag,
                                        Tag: Token::Html,
                                        Value: None,
                                    }
                                } else {
                                    self.read_char();
                                    Tokens {
                                        Type: TokenType::Text,
                                        Tag: Token::None,
                                        Value: Some(String::from("html")),
                                    }
                                }
                            },


                            _ => {
                                self.read_char();
                                Tokens {
                                    Type: TokenType::Text,
                                    Tag: Token::None,
                                    Value: Some(literal),
                                }
                            }
                        };
                        self.read_char();
                        return t;
                    }
                }

                self.read_char();
                Tokens {
                    Type: TokenType::None,
                    Tag: Token::None,
                    Value: None,
                }
            }
        };

        return token;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            char::from(0)
        } else {
            char::from(self.input.as_bytes()[self.position + 1])
        }
    }

    fn peek_char_test(&self, c: usize) -> char {
        if self.read_position >= self.input.len() {
            char::from(0)
        } else {
            char::from(self.input.as_bytes()[self.position + c])
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(&self.ch) {
            self.read_char();
        }
        self.input.get(position..self.position).unwrap().to_string()
    }

    fn err(&mut self) -> Option<Token> {
        panic!(
            "\n{}\n{}\n",
            String::from_utf8([self.ch].to_vec()).unwrap(),
            self.read_position
        );
    }

    fn next(&mut self) {
        self.position += 1;
    }
}

fn is_letter(ch: &u8) -> bool {
    let c = char::from(*ch);
    c.is_alphabetic() || c == '_'
}
