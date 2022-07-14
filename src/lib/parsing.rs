extern crate prog_rs;
use prog_rs::prelude::*;
use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Token {
    LCurlyBraces, // {
    RCurlyBraces, // }
    Div,          // div
    Html,         //html
    Head,         //head
    Title,        //title
    Body,         //body
    Link,
    Script,
    Crossorigin,
    Integrity,
    Href,
    H1,    //h1
    H2,    //h2
    H3,    //h3
    P,     //p
    Id,    //Id
    Class, //Class
    Type,
    CSS,
    Src,
    Style,
    RDiv,  // div
    RHtml, //html
    RScript,
    RHead,  //head
    RTitle, //title
    RBody,  //body
    RH1,    //h1
    RH2,    //h2
    RH3,    //h3
    RP,     //p
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
    Id,
    Src,
    Crossorigin,
    Class,
    Style,
    Href,
    Attribute,
    Type,
    Other,
    None,
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
        let mut st_flag = true;
        let mut func = HashMap::new();
        //let sleeptime =  time::Duration::from_millis(10);
        //let sleeptime2 =  time::Duration::from_millis(100);
        //let sleeptime3 =  time::Duration::from_millis(500);

        loop {
            let toki = input.next_token(&mut st_flag, &mut func);
            println!("{:?}", toki);
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
        let mut cba = 0;

        for i in (0..result_count)
            .progress()
            .with_prefix("Closing tag analysis in progress...")
        {
            //println!("{:?}", result[i]);

            //println!("{:?}", [i]);
            let mut ctagcount = 0;
            let mut stagcount = 0;
            //tmp.push(&result[i]);

            if result[i].Type == TokenType::Tag {
                //tmp3 = result[i].Tag;

                tmp3 = match result[i].Tag {
                    Token::Div => Token::RDiv,
                    Token::Body => Token::RBody,
                    Token::Head => Token::RHead,
                    Token::Script => Token::RScript,
                    Token::H1 => Token::RH1,
                    Token::H2 => Token::RH2,
                    Token::P => Token::RP,
                    Token::Html => Token::RHtml,
                    _ => Token::None,
                };
            }

            for j in ((i + 1)..result.len()) {
                if result[i].Type == TokenType::Tag {
                    if result[j].Tag == Token::RCurlyBraces {
                        if stagcount == 0 {
                            result[j].Tag = tmp3;

                            //println!("{:?}", result[j]);
                            break;
                        }
                        stagcount -= 1;
                    }

                    if result[j].Type == TokenType::Tag {
                        stagcount += 1;
                        ctagcount += 1;
                    }
                }
                //thread::sleep(sleeptime);
            }

            //println!("{}", ctagcount);
            tmp2.push(ctagcount);
            //thread::sleep(sleeptime2);
        }

        //println!("{}","-".repeat(60));

        //println!("{}", tmp2.len());
        //println!("{}", result.len());

        let mut html = String::new();

        for j in (0..result.len() - 2)
            .progress()
            .with_prefix("                         Convert...")
        {
            //thread::sleep(sleeptime3);
            let mut idtag = " id= ".to_owned();
            let mut classtag = " class= ".to_owned();
            let mut styletag = " style= ".to_owned();
            let mut srctag = " src= ".to_owned();
            let mut typetag = " type= ".to_owned();
            let mut crossorigintag = " crossorigin= ".to_owned();

            html += match result[j].Type {
                TokenType::Text => result[j].Value.as_ref().unwrap().as_str(),
                _ => "",
            };

            html += match result[j].Tag {
                Token::Html => "<html",
                Token::Head => "<head",
                Token::Link => "<link",
                Token::Div => "<div",
                Token::Body => "<body",
                Token::H1 => "<h1",
                Token::H2 => "<h2",
                Token::P => "<p",
                Token::Script => "<script",
                Token::RScript => "</script>",
                Token::RP => "</p>",
                Token::RH1 => "</h1>",
                Token::RH2 => "</h2>",
                Token::RDiv => "</div>",
                Token::RHead => "</head>",
                Token::RHtml => "</html>",
                Token::RBody => "</body>",
                Token::RCurlyBraces => "",
                Token::LCurlyBraces => "",
                _ => "",
            };


            let mut ca = 0;


            for k in j..result.len() - 1 {

                

                if result[k].Type == TokenType::Attribute {
                    // /println!("{:?}: {:?}", k, result[k ].Value);
                    println!("cba: {:?}",cba);
                    println!("ca: {:?}",ca);

                    if cba == 1 && ca == 0 {
                        break
                    }

                    if cba > 0 {
                        cba = 0;
                        
                    }

                    /*
                    if cba  > ca {
                        cba = 0;
                        break
                    }
                    
                    
                    */
                    
                    




                    html += match result[k].Tag {
                        Token::Id => {
                            idtag.push_str(&result[k ].Value.as_ref().unwrap());
                            &idtag
                        }
                        Token::Class => {
                            classtag.push_str(&result[k ].Value.as_ref().unwrap());
                            &classtag
                        }
                        Token::Style => {
                            styletag.push_str(&result[k ].Value.as_ref().unwrap());
                            &styletag
                        }
                        Token::Src => {
                            srctag.push_str(&result[k ].Value.as_ref().unwrap());
                            &srctag
                        }
                        Token::Type => {
                            typetag.push_str(&result[k ].Value.as_ref().unwrap());
                            &typetag
                        }
                        Token::Crossorigin => {
                            crossorigintag.push_str(&result[k ].Value.as_ref().unwrap());
                            &crossorigintag
                        }
        
                        Token::RCurlyBraces => "",
                        Token::LCurlyBraces => "",
                        _ => "",
                    };

               
                    ca+=1;

                }


                if result[k + 1].Type == TokenType::Tag || result[k + 1].Type == TokenType::Text  {
                    cba+=1;
                    
                    break
                }

                

                println!("{:?}",result[k].Type);
                println!("{}",html);




            }



            

            html += match result[j].Tag {
                Token::Html => ">",
                Token::Head => ">",
                Token::Link => "/>",
                Token::Div => ">",
                Token::Body => ">",
                Token::H1 => ">",
                Token::H2 => ">",
                Token::P => ">",
                Token::Script => ">",
                Token::RCurlyBraces => "",
                Token::LCurlyBraces => "",
                _ => "",
            };



        }

        html
    }

    pub fn next_token(&mut self, st_flag: &mut bool, func: &mut HashMap<String, String>) -> Tokens {
        let mut flag = true;
        while flag {
            let c = char::from(self.ch);

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
            b':' => {
                self.read_char();
                Tokens {
                    Type: TokenType::Other,
                    Tag: Token::None,
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

            b'"' => {
                self.read_char();
                let mut count = 0;
                let mut st = String::new();

                loop {
                    if self.peek_char_usize(count) == '"' {
                        return Tokens {
                            Type: TokenType::Text,
                            Tag: Token::None,
                            Value: Some(st),
                        };
                    }

                    st.push(self.peek_char_usize(count));

                    count += 1;
                }
            }

            _ => {
                if self.ch > 0 {
                    if is_letter(&self.ch) {
                        let literal = self.read_identifier();
                        //println!("{}", literal);
                        let t = match literal.as_str() {
                            "id" => {
                                self.attribute("id")
                            }
                            "class" => {
                                self.attribute("class")
                            }
                            "style" => {
                                self.attribute("style")
                            }
                            "src" => {
                                self.attribute("src")
                            }
                            "type" => {
                                self.attribute("type")
                            }
                            "crossorigin" => {
                                self.attribute("crossorigin")
                            }
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
                            }
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
                            }
                            "body" => {
                                if self.peek_char() == '{' {
                                    self.read_char();
                                    Tokens {
                                        Type: TokenType::Tag,
                                        Tag: Token::Body,
                                        Value: None,
                                    }
                                } else {
                                    self.read_char();
                                    Tokens {
                                        Type: TokenType::Text,
                                        Tag: Token::None,
                                        Value: Some(String::from("body")),
                                    }
                                }
                            }
                            "h" => {
                                //println!("{}",self.peek_char_isize(2));
                                if self.peek_char_isize(0) == '1' {
                                    self.read_char();
                                    if self.peek_char() == '{' {
                                        self.read_char();
                                        Tokens {
                                            Type: TokenType::Tag,
                                            Tag: Token::H1,
                                            Value: None,
                                        }
                                    } else {
                                        self.read_char();
                                        Tokens {
                                            Type: TokenType::Text,
                                            Tag: Token::None,
                                            Value: Some(String::from("h1")),
                                        }
                                    }
                                } else if self.peek_char_isize(0) == '2' {
                                    self.read_char();
                                    if self.peek_char() == '{' {
                                        self.read_char();
                                        Tokens {
                                            Type: TokenType::Tag,
                                            Tag: Token::H2,
                                            Value: None,
                                        }
                                    } else {
                                        self.read_char();
                                        Tokens {
                                            Type: TokenType::Text,
                                            Tag: Token::None,
                                            Value: Some(String::from("h2")),
                                        }
                                    }
                                } else {
                                    self.read_char();
                                    Tokens {
                                        Type: TokenType::Other,
                                        Tag: Token::None,
                                        Value: None,
                                    }
                                }
                            }
                            "p" => {
                                if self.peek_char() == '{' {
                                    self.read_char();
                                    Tokens {
                                        Type: TokenType::Tag,
                                        Tag: Token::P,
                                        Value: None,
                                    }
                                } else {
                                    self.read_char();
                                    Tokens {
                                        Type: TokenType::Text,
                                        Tag: Token::None,
                                        Value: Some(String::from("p")),
                                    }
                                }
                            }
                            "head" => {
                                if self.peek_char() == '{' {
                                    self.read_char();
                                    Tokens {
                                        Type: TokenType::Tag,
                                        Tag: Token::Head,
                                        Value: None,
                                    }
                                } else {
                                    self.read_char();
                                    Tokens {
                                        Type: TokenType::Text,
                                        Tag: Token::None,
                                        Value: Some(String::from("head")),
                                    }
                                }
                            }
                            "fn" => {
                                let mut count = 1;
                                let mut l = String::new();
                                let mut fn_argument: bool = false;

                                loop {
                                    //println!("{}",self.peek_char_usize(count));
                                    l.push(self.peek_char_usize(count));

                                    if self.peek_char_usize(count) == '(' {
                                        fn_argument = true;
                                    }

                                    if self.peek_char_usize(count) == ')' && fn_argument == true {
                                        //let mut count2 = 0;

                                        loop {
                                            //println!("{}",self.peek_char_usize(count + 2));

                                            if self.peek_char_usize(count) == '-'
                                                && self.peek_char_usize(count + 1) == '>'
                                            {
                                                break;
                                            } else if self.peek_char_usize(count) == '-'
                                                && self.peek_char_usize(count + 1) != '>'
                                            {
                                                self.err();
                                            }

                                            count += 1;
                                        }

                                        //println!("{}",self.peek_char_usize(count))

                                        break;
                                    }

                                    count += 1;
                                }

                                //空白除去
                                l.retain(|c| c != ' ');

                                println!("fn name: {:?}", l);

                                func.insert(l, "".to_owned());

                                println!("{:?}", func);

                                //println!("{}",self.peek_char_usize(count + 3));

                                Tokens {
                                    Type: TokenType::None,
                                    Tag: Token::None,
                                    Value: None,
                                }
                            }

                            _ => {
                                self.read_char();
                                Tokens {
                                    Type: TokenType::Other,
                                    Tag: Token::None,
                                    Value: None,
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

        self.next();
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

    fn attribute(&mut self,tag:&str) -> Tokens {

        self.read_char();
        if self.peek_char_isize(-1) == ':' {
            let mut count = 0;
            let mut l = String::from('"');
            loop {
                if self.peek_char_usize(count) == ';' || count > 2 ^ 16 {
                    break;
                }

                l.push(self.peek_char_usize(count));

                count += 1;
            }
            l.push('"');


            match tag {
                "type" => Tokens {
                    Type: TokenType::Attribute,
                    Tag: Token::Type,
                    Value: Some(l.to_owned()),
                },
                "id" => Tokens {
                    Type: TokenType::Attribute,
                    Tag: Token::Id,
                    Value: Some(l.to_owned()),
                },
                "class" => Tokens {
                    Type: TokenType::Attribute,
                    Tag: Token::Class,
                    Value: Some(l.to_owned()),
                },
                "style" => Tokens {
                    Type: TokenType::Attribute,
                    Tag: Token::Style,
                    Value: Some(l.to_owned()),
                },
                "src" => Tokens {
                    Type: TokenType::Attribute,
                    Tag: Token::Src,
                    Value: Some(l.to_owned()),
                },
                "crossorigin" => Tokens {
                    Type: TokenType::Attribute,
                    Tag: Token::Crossorigin,
                    Value: Some(l.to_owned()),
                },
                _ => Tokens {
                    Type: TokenType::None,
                    Tag: Token::None,
                    Value: None,
                },
            }

        } else {
            return Tokens {
                Type: TokenType::Text,
                Tag: Token::None,
                Value: Some(tag.to_owned()),
            };
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            char::from(0)
        } else {
            char::from(self.input.as_bytes()[self.position + 1])
        }
    }

    fn peek_char_usize(&self, c: usize) -> char {
        if self.read_position >= self.input.len() {
            char::from(0)
        } else {
            char::from(self.input.as_bytes()[self.position + c])
        }
    }

    fn peek_char_isize(&self, c: isize) -> char {
        if self.read_position >= self.input.len() {
            char::from(0)
        } else {
            char::from(self.input.as_bytes()[(((self.position) as isize) + c) as usize])
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
